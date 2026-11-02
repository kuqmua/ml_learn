//! Архитектура из ng-video-lecture: отдельные головы, pre-LN, ReLU и обучаемые позиции.
//! Candle предоставляет операции над тензорами и backward, но не готовую GPT-модель.
use crate::{random::RandomNumberGenerator, settings::Settings};
use candle_core::{D, DType, Device, Result, Tensor, Var};

pub(crate) struct ModelParameters {
    pub(crate) named_variables: Vec<(String, Var)>,
    random_number_generator: RandomNumberGenerator,
}
impl ModelParameters {
    pub(crate) fn new(random_seed: u64) -> Self {
        Self {
            named_variables: vec![],
            random_number_generator: RandomNumberGenerator::new(random_seed),
        }
    }
    fn add_variable(&mut self, parameter_name: &str, dimensions: &[usize], initial_constant: Option<f32>) -> Result<Tensor> {
        let initial_parameter_values: Vec<f32> = (0..dimensions.iter().product::<usize>())
            .map(|_: usize| -> f32 { initial_constant.unwrap_or_else(|| -> f32 { self.random_number_generator.standard_normal_value() * 0.02 }) })
            .collect();
        let parameter_variable: Var = Var::from_vec(initial_parameter_values, dimensions, &Device::Cpu)?;
        let parameter_tensor: Tensor = parameter_variable.as_tensor().clone();
        self.named_variables.push((parameter_name.to_owned(), parameter_variable));
        Ok(parameter_tensor)
    }
    pub(crate) fn trainable_variables(&self) -> Vec<Var> {
        self.named_variables
            .iter()
            .map(|(_, parameter_variable): &(String, Var)| -> Var { parameter_variable.clone() })
            .collect()
    }
    pub(crate) fn parameter_count(&self) -> usize {
        self.named_variables
            .iter()
            .map(|(_, parameter_variable): &(String, Var)| -> usize { parameter_variable.elem_count() })
            .sum()
    }
}

// В eval отсоединяем параметры от графа: ни dropout, ни накопление графа для backward.
fn parameter_for_mode(parameter_tensor: &Tensor, is_training: bool) -> Tensor {
    if is_training { parameter_tensor.clone() } else { parameter_tensor.detach() }
}

struct LinearLayer {
    weight_matrix: Tensor,
    bias_vector: Option<Tensor>,
}
impl LinearLayer {
    fn new(parameters: &mut ModelParameters, parameter_prefix: &str, input_dimension: usize, output_dimension: usize, use_bias: bool) -> Result<Self> {
        Ok(Self {
            weight_matrix: parameters.add_variable(&format!("{parameter_prefix}.weight"), &[output_dimension, input_dimension], None)?,
            bias_vector: if use_bias {
                Some(parameters.add_variable(&format!("{parameter_prefix}.bias"), &[output_dimension], Some(0.0))?)
            } else {
                None
            },
        })
    }
    fn forward(&self, input_vectors: &Tensor, is_training: bool) -> Result<Tensor> {
        let input_dimension: usize = input_vectors.dim(D::Minus1)?;
        let output_dimension: usize = self.weight_matrix.dim(0)?;
        let mut output_dimensions: Vec<usize> = input_vectors.dims().to_vec();
        *output_dimensions.last_mut().unwrap() = output_dimension;
        let output_vectors: Tensor = input_vectors
            .reshape((input_vectors.elem_count() / input_dimension, input_dimension))?
            .matmul(&parameter_for_mode(&self.weight_matrix, is_training).t()?)?;
        let output_vectors: Tensor = self
            .bias_vector
            .as_ref()
            .map(|bias_vector: &Tensor| -> Result<Tensor> { output_vectors.broadcast_add(&parameter_for_mode(bias_vector, is_training)) })
            .transpose()?
            .unwrap_or(output_vectors);
        output_vectors.reshape(output_dimensions)
    }
}

struct EmbeddingTable {
    embedding_vectors: Tensor,
    embedding_dimension: usize,
}
impl EmbeddingTable {
    fn new(parameters: &mut ModelParameters, parameter_prefix: &str, embedding_count: usize, embedding_dimension: usize) -> Result<Self> {
        Ok(Self {
            embedding_vectors: parameters.add_variable(&format!("{parameter_prefix}.weight"), &[embedding_count, embedding_dimension], None)?,
            embedding_dimension,
        })
    }
    fn forward(&self, embedding_indices: &Tensor, is_training: bool) -> Result<Tensor> {
        let mut output_dimensions: Vec<usize> = embedding_indices.dims().to_vec();
        output_dimensions.push(self.embedding_dimension);
        parameter_for_mode(&self.embedding_vectors, is_training)
            .index_select(&embedding_indices.flatten_all()?, 0)?
            .reshape(output_dimensions)
    }
}

// Нормируем C координат КАЖДОЙ позиции отдельно; не batch и не ось времени.
// Пишем через базовые операции, чтобы путь backward был виден и поддерживался.
struct LayerNormalization {
    gain_vector: Tensor,
    bias_vector: Tensor,
}
impl LayerNormalization {
    fn new(parameters: &mut ModelParameters, parameter_prefix: &str, embedding_dimension: usize) -> Result<Self> {
        Ok(Self {
            gain_vector: parameters.add_variable(&format!("{parameter_prefix}.weight"), &[embedding_dimension], Some(1.0))?,
            bias_vector: parameters.add_variable(&format!("{parameter_prefix}.bias"), &[embedding_dimension], Some(0.0))?,
        })
    }
    fn forward(&self, input_vectors: &Tensor, is_training: bool) -> Result<Tensor> {
        let centered_vectors: Tensor = input_vectors.broadcast_sub(&input_vectors.mean_keepdim(D::Minus1)?)?;
        let coordinate_variances: Tensor = centered_vectors.sqr()?.mean_keepdim(D::Minus1)?;
        centered_vectors
            .broadcast_div(&(coordinate_variances + 1e-5)?.sqrt()?)?
            .broadcast_mul(&parameter_for_mode(&self.gain_vector, is_training))?
            .broadcast_add(&parameter_for_mode(&self.bias_vector, is_training))
    }
}

// Inverted dropout: оставшиеся значения делим на 1-p; в eval это тождественная функция.
fn apply_dropout(input_values: &Tensor, dropout_probability: f64, is_training: bool, random_number_generator: &mut RandomNumberGenerator) -> Result<Tensor> {
    if !is_training || dropout_probability == 0.0 {
        return Ok(input_values.clone());
    }
    let dropout_multipliers: Vec<f32> = (0..input_values.elem_count())
        .map(|_: usize| -> f32 {
            if random_number_generator.uniform_probability() < dropout_probability {
                0.0
            } else {
                (1.0 / (1.0 - dropout_probability)) as f32
            }
        })
        .collect();
    input_values * Tensor::from_vec(dropout_multipliers, input_values.shape(), input_values.device())?
}

pub(crate) fn causal_mask(sequence_length: usize) -> Result<Tensor> {
    let mask_values: Vec<f32> = (0..sequence_length)
        .flat_map(|query_position: usize| {
            (0usize..sequence_length).map(move |key_position: usize| -> f32 {
                if key_position > query_position { f32::NEG_INFINITY } else { 0.0 }
            })
        })
        .collect();
    Tensor::from_vec(mask_values, (sequence_length, sequence_length), &Device::Cpu)
}

// Q: что ищу; K: по каким признакам меня искать; V: что передам другим позициям.
fn compute_attention_probabilities(query_vectors: &Tensor, key_vectors: &Tensor) -> Result<Tensor> {
    let (_, sequence_length, head_dimension): (usize, usize, usize) = query_vectors.dims3()?;
    let attention_scores: Tensor = (query_vectors.matmul(&key_vectors.transpose(1, 2)?)? / (head_dimension as f64).sqrt())?;
    let masked_attention_scores: Tensor = attention_scores.broadcast_add(&causal_mask(sequence_length)?.to_dtype(query_vectors.dtype())?)?;
    candle_nn::ops::softmax(&masked_attention_scores, D::Minus1)
}

struct AttentionHead {
    query_projection: LinearLayer,
    key_projection: LinearLayer,
    value_projection: LinearLayer,
}
impl AttentionHead {
    fn new(parameters: &mut ModelParameters, parameter_prefix: &str, embedding_dimension: usize, head_dimension: usize) -> Result<Self> {
        Ok(Self {
            query_projection: LinearLayer::new(parameters, &format!("{parameter_prefix}.query"), embedding_dimension, head_dimension, false)?,
            key_projection: LinearLayer::new(parameters, &format!("{parameter_prefix}.key"), embedding_dimension, head_dimension, false)?,
            value_projection: LinearLayer::new(parameters, &format!("{parameter_prefix}.value"), embedding_dimension, head_dimension, false)?,
        })
    }
    fn forward(
        &self,
        input_vectors: &Tensor,
        dropout_probability: f64,
        is_training: bool,
        random_number_generator: &mut RandomNumberGenerator,
    ) -> Result<Tensor> {
        let query_vectors: Tensor = self.query_projection.forward(input_vectors, is_training)?; // B,T,D
        let key_vectors: Tensor = self.key_projection.forward(input_vectors, is_training)?;
        let value_vectors: Tensor = self.value_projection.forward(input_vectors, is_training)?;
        let attention_probabilities: Tensor = compute_attention_probabilities(&query_vectors, &key_vectors)?; // B,T,T: softmax по ключам
        apply_dropout(&attention_probabilities, dropout_probability, is_training, random_number_generator)?.matmul(&value_vectors)
    }
}

struct MultiHeadAttention {
    attention_heads: Vec<AttentionHead>,
    output_projection: LinearLayer,
    dropout_probability: f64,
}
impl MultiHeadAttention {
    fn new(parameters: &mut ModelParameters, parameter_prefix: &str, settings: &Settings) -> Result<Self> {
        let attention_heads: Vec<AttentionHead> = (0usize..settings.attention_head_count())
            .map(|head_index: usize| -> Result<AttentionHead> {
                AttentionHead::new(
                    parameters,
                    &format!("{parameter_prefix}.head{head_index}"),
                    settings.embedding_dimension(),
                    settings.embedding_dimension() / settings.attention_head_count(),
                )
            })
            .collect::<Result<Vec<AttentionHead>>>()?;
        Ok(Self {
            attention_heads,
            output_projection: LinearLayer::new(
                parameters,
                &format!("{parameter_prefix}.projection"),
                settings.embedding_dimension(),
                settings.embedding_dimension(),
                true,
            )?,
            dropout_probability: settings.dropout_probability(),
        })
    }
    fn forward(&self, input_vectors: &Tensor, is_training: bool, random_number_generator: &mut RandomNumberGenerator) -> Result<Tensor> {
        let head_output_vectors: Vec<Tensor> = self
            .attention_heads
            .iter()
            .map(|attention_head: &AttentionHead| -> Result<Tensor> {
                attention_head.forward(input_vectors, self.dropout_probability, is_training, random_number_generator)
            })
            .collect::<Result<Vec<Tensor>>>()?;
        let concatenated_head_vectors: Tensor = Tensor::cat(&head_output_vectors, 2)?; // B,T,H*D = B,T,C
        apply_dropout(
            &self.output_projection.forward(&concatenated_head_vectors, is_training)?,
            self.dropout_probability,
            is_training,
            random_number_generator,
        )
    }
}

struct FeedForwardNetwork {
    expansion_projection: LinearLayer,
    contraction_projection: LinearLayer,
    dropout_probability: f64,
}
impl FeedForwardNetwork {
    fn new(parameters: &mut ModelParameters, parameter_prefix: &str, settings: &Settings) -> Result<Self> {
        Ok(Self {
            expansion_projection: LinearLayer::new(
                parameters,
                &format!("{parameter_prefix}.expansion_projection"),
                settings.embedding_dimension(),
                4 * settings.embedding_dimension(),
                true,
            )?,
            contraction_projection: LinearLayer::new(
                parameters,
                &format!("{parameter_prefix}.contraction_projection"),
                4 * settings.embedding_dimension(),
                settings.embedding_dimension(),
                true,
            )?,
            dropout_probability: settings.dropout_probability(),
        })
    }
    fn forward(&self, input_vectors: &Tensor, is_training: bool, random_number_generator: &mut RandomNumberGenerator) -> Result<Tensor> {
        let expanded_activations: Tensor = self.expansion_projection.forward(input_vectors, is_training)?.relu()?;
        apply_dropout(
            &self.contraction_projection.forward(&expanded_activations, is_training)?,
            self.dropout_probability,
            is_training,
            random_number_generator,
        )
    }
}

struct TransformerBlock {
    attention_layer_normalization: LayerNormalization,
    self_attention: MultiHeadAttention,
    feed_forward_layer_normalization: LayerNormalization,
    feed_forward_network: FeedForwardNetwork,
}
impl TransformerBlock {
    fn new(parameters: &mut ModelParameters, parameter_prefix: &str, settings: &Settings) -> Result<Self> {
        Ok(Self {
            attention_layer_normalization: LayerNormalization::new(
                parameters,
                &format!("{parameter_prefix}.attention_layer_normalization"),
                settings.embedding_dimension(),
            )?,
            self_attention: MultiHeadAttention::new(parameters, &format!("{parameter_prefix}.attention"), settings)?,
            feed_forward_layer_normalization: LayerNormalization::new(
                parameters,
                &format!("{parameter_prefix}.feed_forward_layer_normalization"),
                settings.embedding_dimension(),
            )?,
            feed_forward_network: FeedForwardNetwork::new(parameters, &format!("{parameter_prefix}.ff"), settings)?,
        })
    }
    fn forward(&self, residual_vectors: &Tensor, is_training: bool, random_number_generator: &mut RandomNumberGenerator) -> Result<Tensor> {
        let residual_vectors: Tensor = (residual_vectors
            + self.self_attention.forward(
                &self.attention_layer_normalization.forward(residual_vectors, is_training)?,
                is_training,
                random_number_generator,
            )?)?;
        &residual_vectors
            + self.feed_forward_network.forward(
                &self.feed_forward_layer_normalization.forward(&residual_vectors, is_training)?,
                is_training,
                random_number_generator,
            )?
    }
}

pub(crate) struct GptLanguageModel {
    token_embeddings: EmbeddingTable,
    position_embeddings: EmbeddingTable,
    transformer_blocks: Vec<TransformerBlock>,
    final_layer_normalization: LayerNormalization,
    next_token_projection: LinearLayer,
    maximum_context_length: usize,
}
impl GptLanguageModel {
    pub(crate) fn new(parameters: &mut ModelParameters, vocabulary_size: usize, settings: &Settings) -> Result<Self> {
        settings.validate()?;
        Ok(Self {
            token_embeddings: EmbeddingTable::new(parameters, "token_embeddings", vocabulary_size, settings.embedding_dimension())?,
            position_embeddings: EmbeddingTable::new(parameters, "position_embeddings", settings.block_size(), settings.embedding_dimension())?,
            transformer_blocks: (0..settings.transformer_block_count())
                .map(|block_index: usize| -> Result<TransformerBlock> { TransformerBlock::new(parameters, &format!("block{block_index}"), settings) })
                .collect::<Result<Vec<TransformerBlock>>>()?,
            final_layer_normalization: LayerNormalization::new(parameters, "final_layer_normalization", settings.embedding_dimension())?,
            next_token_projection: LinearLayer::new(parameters, "next_token_projection", settings.embedding_dimension(), vocabulary_size, true)?,
            maximum_context_length: settings.block_size(),
        })
    }
    pub(crate) fn maximum_context_length(&self) -> usize {
        self.maximum_context_length
    }
    pub(crate) fn forward(&self, input_token_tensor: &Tensor, is_training: bool, random_number_generator: &mut RandomNumberGenerator) -> Result<Tensor> {
        let (_, sequence_length): (usize, usize) = input_token_tensor.dims2()?;
        if sequence_length == 0 || sequence_length > self.maximum_context_length {
            candle_core::bail!("Нужно 0 < sequence_length <= maximum_context_length");
        }
        let position_id_tensor: Tensor = Tensor::arange(0u32, sequence_length as u32, input_token_tensor.device())?;
        let mut context_vectors: Tensor = self
            .token_embeddings
            .forward(input_token_tensor, is_training)?
            .broadcast_add(&self.position_embeddings.forward(&position_id_tensor, is_training)?)?;
        self.transformer_blocks
            .iter()
            .try_for_each(|transformer_block: &TransformerBlock| -> Result<()> {
                context_vectors = transformer_block.forward(&context_vectors, is_training, random_number_generator)?;
                Ok(())
            })?;
        self.next_token_projection
            .forward(&self.final_layer_normalization.forward(&context_vectors, is_training)?, is_training)
    }
}

pub(crate) fn mean_next_token_loss(next_token_logits: &Tensor, target_token_tensor: &Tensor) -> Result<Tensor> {
    let (batch_size, sequence_length, vocabulary_size): (usize, usize, usize) = next_token_logits.dims3()?;
    if target_token_tensor.dims() != [batch_size, sequence_length] {
        candle_core::bail!("target_token_tensor должен иметь форму (batch_size, sequence_length)");
    }
    candle_nn::loss::cross_entropy(
        &next_token_logits.reshape((batch_size * sequence_length, vocabulary_size))?,
        &target_token_tensor.flatten_all()?,
    )
}

// Три эквивалентных способа усреднить прошлое: цикл, нижнетреугольная матрица,
// masked softmax нулевых scores. Это отправная точка перехода к Q/K/V.
pub(crate) fn demonstrate_causal_averaging() -> Result<()> {
    let position_values: Tensor = Tensor::new(&[[1f32], [2.], [3.], [4.]], &Device::Cpu)?;
    let zero_query_key_vectors: Tensor = Tensor::zeros((1, 4, 1), DType::F32, &Device::Cpu)?;
    let attention_probabilities: Tensor = compute_attention_probabilities(&zero_query_key_vectors, &zero_query_key_vectors)?.squeeze(0)?;
    let averages_from_attention: Vec<Vec<f32>> = attention_probabilities.matmul(&position_values)?.to_vec2::<f32>()?;
    let averages_from_prefix_sums: Vec<f32> = (1..=4)
        .map(|prefix_length: usize| -> f32 { (1usize..=prefix_length).sum::<usize>() as f32 / prefix_length as f32 })
        .collect();
    println!("\nДо Q/K/V: causal average для [1,2,3,4]");
    let prefix_average_coefficients: Vec<f32> = (0..4)
        .flat_map(|query_position: usize| {
            (0usize..4).map(move |key_position: usize| -> f32 {
                if key_position <= query_position {
                    1.0 / (query_position + 1) as f32
                } else {
                    0.0
                }
            })
        })
        .collect();
    let averages_from_triangular_matrix: Vec<Vec<f32>> = Tensor::from_vec(prefix_average_coefficients, (4, 4), &Device::Cpu)?
        .matmul(&position_values)?
        .to_vec2::<f32>()?;
    println!("Цикл: {averages_from_prefix_sums:?}; нижнетреугольная матрица × x: {averages_from_triangular_matrix:?}");
    println!("Masked softmax × x: {averages_from_attention:?}");
    println!("Матрица внимания: {:?}", attention_probabilities.to_vec2::<f32>()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{AttentionHeadCount, BlockSize, DropoutProbability, EmbeddingDimension, TransformerBlockCount};
    fn settings() -> Result<Settings> {
        Ok(Settings {
            block_size: BlockSize::try_from(4)?,
            embedding_dimension: EmbeddingDimension::try_from(8)?,
            attention_head_count: AttentionHeadCount::try_from(2)?,
            transformer_block_count: TransformerBlockCount::try_from(2)?,
            dropout_probability: DropoutProbability::try_from(0.2)?,
            ..crate::settings::test_settings()?
        })
    }

    #[test]
    fn attention_is_normalized_causal_and_matches_prefix_averages() -> Result<()> {
        let zero_query_key_vectors: Tensor = Tensor::zeros((1, 4, 2), DType::F32, &Device::Cpu)?;
        let attention_probabilities: Vec<Vec<Vec<f32>>> =
            compute_attention_probabilities(&zero_query_key_vectors, &zero_query_key_vectors)?.to_vec3::<f32>()?;
        attention_probabilities[0]
            .iter()
            .enumerate()
            .for_each(|(query_position, probabilities_by_key): (usize, &Vec<f32>)| {
                assert!((probabilities_by_key.iter().sum::<f32>() - 1.0).abs() < 1e-6);
                probabilities_by_key.iter().enumerate().for_each(|(key_position, &probability): (usize, &f32)| {
                    let expected_probability: f32 = if key_position > query_position {
                        0.0
                    } else {
                        1.0 / (query_position + 1) as f32
                    };
                    assert!((probability - expected_probability).abs() < 1e-6);
                });
            });
        let position_values: Tensor = Tensor::new(&[[[1f32], [2.], [3.], [4.]]], &Device::Cpu)?;
        let averaged_values: Vec<f32> = compute_attention_probabilities(&zero_query_key_vectors, &zero_query_key_vectors)?
            .matmul(&position_values)?
            .flatten_all()?
            .to_vec1::<f32>()?;
        averaged_values
            .iter()
            .zip([1.0f32, 1.5, 2.0, 2.5])
            .for_each(|(actual_average, expected_average): (&f32, f32)| {
                assert!((actual_average - expected_average).abs() < 1e-6);
            });
        Ok(())
    }

    #[test]
    fn future_tokens_do_not_change_prefix_logits() -> Result<()> {
        let mut parameters: ModelParameters = ModelParameters::new(7);
        let language_model: GptLanguageModel = GptLanguageModel::new(&mut parameters, 5, &settings()?)?;
        let original_token_tensor: Tensor = Tensor::new(&[[0u32, 1, 2, 3]], &Device::Cpu)?;
        let modified_token_tensor: Tensor = Tensor::new(&[[0u32, 1, 4, 0]], &Device::Cpu)?;
        let mut random_number_generator: RandomNumberGenerator = RandomNumberGenerator::new(3);
        let original_logits: Tensor = language_model.forward(&original_token_tensor, false, &mut random_number_generator)?;
        let modified_logits: Tensor = language_model.forward(&modified_token_tensor, false, &mut random_number_generator)?;
        let maximum_prefix_difference: f32 = (original_logits.narrow(1, 0, 2)? - modified_logits.narrow(1, 0, 2)?)?
            .abs()?
            .max_all()?
            .to_scalar::<f32>()?;
        assert!(maximum_prefix_difference < 1e-6, "future leakage: {maximum_prefix_difference}");
        Ok(())
    }

    #[test]
    fn gradients_reach_all_parameters_and_eval_detaches_them() -> Result<()> {
        let mut parameters: ModelParameters = ModelParameters::new(7);
        let language_model: GptLanguageModel = GptLanguageModel::new(&mut parameters, 5, &settings()?)?;
        let input_token_tensor: Tensor = Tensor::new(&[[0u32, 1, 2, 3], [3, 2, 1, 4]], &Device::Cpu)?;
        let target_token_tensor: Tensor = Tensor::new(&[[1u32, 2, 3, 4], [2, 1, 4, 0]], &Device::Cpu)?;
        let mut random_number_generator: RandomNumberGenerator = RandomNumberGenerator::new(4);
        let parameter_gradients: candle_core::backprop::GradStore = mean_next_token_loss(
            &language_model.forward(&input_token_tensor, true, &mut random_number_generator)?,
            &target_token_tensor,
        )?
        .backward()?;
        parameters
            .named_variables
            .iter()
            .try_for_each(|(parameter_name, parameter_variable): &(String, Var)| -> Result<()> {
                let gradient_tensor: &Tensor = parameter_gradients
                    .get(parameter_variable)
                    .unwrap_or_else(|| -> &Tensor { panic!("Нет градиента: {parameter_name}") });
                let gradient_values: Vec<f32> = gradient_tensor.flatten_all()?.to_vec1::<f32>()?;
                assert!(
                    gradient_values.iter().all(|gradient_value: &f32| -> bool { gradient_value.is_finite() }),
                    "{parameter_name}"
                );
                assert!(
                    gradient_values.iter().any(|gradient_value: &f32| -> bool { gradient_value.abs() > 0.0 }),
                    "Нулевой градиент: {parameter_name}"
                );
                Ok(())
            })?;
        let evaluation_logits: Tensor = language_model.forward(&input_token_tensor, false, &mut random_number_generator)?;
        let repeated_evaluation_logits: Tensor = language_model.forward(&input_token_tensor, false, &mut random_number_generator)?;
        assert_eq!(evaluation_logits.to_vec3::<f32>()?, repeated_evaluation_logits.to_vec3::<f32>()?);
        let parameter_gradients: candle_core::backprop::GradStore = mean_next_token_loss(&evaluation_logits, &target_token_tensor)?.backward()?;
        assert!(
            parameters
                .named_variables
                .iter()
                .all(|(_, parameter_variable): &(String, Var)| -> bool { parameter_gradients.get(parameter_variable).is_none() })
        );
        Ok(())
    }

    #[test]
    fn attention_backward_matches_finite_difference() -> Result<()> {
        // f64 и центральная разность: проверяем градиент Q через mask, softmax и AV.
        let query_coordinates: Vec<f64> = vec![0.1f64, 0.2, 0.3, -0.4, 0.5, 0.6];
        let query_variable: Var = Var::from_vec(query_coordinates.clone(), (1, 3, 2), &Device::Cpu)?;
        let key_vectors: Tensor = Tensor::new(&[[[0.2f64, 0.4], [-0.3, 0.7], [0.8, 0.1]]], &Device::Cpu)?;
        let value_vectors: Tensor = Tensor::new(&[[[0.2f64], [-0.8], [1.2]]], &Device::Cpu)?;
        let attention_objective: &dyn Fn(&Tensor) -> Result<Tensor> = &|query_vectors: &Tensor| -> Result<Tensor> {
            compute_attention_probabilities(query_vectors, &key_vectors)?
                .matmul(&value_vectors)?
                .sqr()?
                .sum_all()
        };
        let parameter_gradients: candle_core::backprop::GradStore = attention_objective(query_variable.as_tensor())?.backward()?;
        let analytical_query_gradients: Vec<f64> = parameter_gradients.get(&query_variable).unwrap().flatten_all()?.to_vec1::<f64>()?;
        (0usize..query_coordinates.len()).try_for_each(|coordinate_index: usize| -> Result<()> {
            let mut increased_query_coordinates: Vec<f64> = query_coordinates.clone();
            increased_query_coordinates[coordinate_index] += 1e-5;
            let mut decreased_query_coordinates: Vec<f64> = query_coordinates.clone();
            decreased_query_coordinates[coordinate_index] -= 1e-5;
            let increased_loss: f64 = attention_objective(&Tensor::from_vec(increased_query_coordinates, (1, 3, 2), &Device::Cpu)?)?.to_scalar::<f64>()?;
            let decreased_loss: f64 = attention_objective(&Tensor::from_vec(decreased_query_coordinates, (1, 3, 2), &Device::Cpu)?)?.to_scalar::<f64>()?;
            let numerical_gradient: f64 = (increased_loss - decreased_loss) / 2e-5;
            assert!(
                (analytical_query_gradients[coordinate_index] - numerical_gradient).abs() < 1e-7,
                "i={coordinate_index}: analytic={}, numerical={numerical_gradient}",
                analytical_query_gradients[coordinate_index]
            );
            Ok(())
        })?;
        Ok(())
    }

    #[test]
    fn dropout_is_seeded_and_layer_norm_uses_channels_only() -> Result<()> {
        let unit_values: Tensor = Tensor::ones((2, 4, 8), DType::F32, &Device::Cpu)?;
        let first_dropout_values: Vec<f32> = apply_dropout(&unit_values, 0.5, true, &mut RandomNumberGenerator::new(1))?
            .flatten_all()?
            .to_vec1::<f32>()?;
        let repeated_dropout_values: Vec<f32> = apply_dropout(&unit_values, 0.5, true, &mut RandomNumberGenerator::new(1))?
            .flatten_all()?
            .to_vec1::<f32>()?;
        assert_eq!(first_dropout_values, repeated_dropout_values);
        assert!(first_dropout_values.contains(&0.0) && first_dropout_values.contains(&2.0));
        assert_eq!(
            apply_dropout(&unit_values, 0.5, false, &mut RandomNumberGenerator::new(1))?
                .flatten_all()?
                .to_vec1::<f32>()?,
            vec![1f32; 64]
        );
        let mut parameters: ModelParameters = ModelParameters::new(1);
        let layer_normalization: LayerNormalization = LayerNormalization::new(&mut parameters, "layer_normalization", 3)?;
        let input_vectors: Tensor = Tensor::new(&[[[1f32, 2., 3.], [101., 102., 103.]]], &Device::Cpu)?;
        let normalized_vectors: Vec<Vec<Vec<f32>>> = layer_normalization.forward(&input_vectors, false)?.to_vec3::<f32>()?;
        assert_eq!(normalized_vectors[0][0], normalized_vectors[0][1]);
        assert!(normalized_vectors[0][0].iter().sum::<f32>().abs() < 1e-6);
        assert!((normalized_vectors[0][0][0] + 1.2247356).abs() < 1e-5);
        Ok(())
    }

    #[test]
    fn loss_is_mean_cross_entropy() -> Result<()> {
        let uniform_next_token_logits: Tensor = Tensor::zeros((2, 4, 5), DType::F32, &Device::Cpu)?;
        let target_token_tensor: Tensor = Tensor::zeros((2, 4), DType::U32, &Device::Cpu)?;
        assert!((mean_next_token_loss(&uniform_next_token_logits, &target_token_tensor)?.to_scalar::<f32>()? - 5f32.ln()).abs() < 1e-6);
        Ok(())
    }
}

#[cfg(test)]
mod lecture_tests {
    use super::*;
    use crate::settings::{AttentionHeadCount, BlockSize, DropoutProbability, EmbeddingDimension, TransformerBlockCount};
    #[test]
    fn lecture_architecture_parameter_count_and_short_forward() -> Result<()> {
        let settings: Settings = Settings {
            block_size: BlockSize::try_from(256)?,
            embedding_dimension: EmbeddingDimension::try_from(384)?,
            attention_head_count: AttentionHeadCount::try_from(6)?,
            transformer_block_count: TransformerBlockCount::try_from(6)?,
            dropout_probability: DropoutProbability::try_from(0.2)?,
            ..crate::settings::test_settings()?
        };
        let mut parameters: ModelParameters = ModelParameters::new(1337);
        let language_model: GptLanguageModel = GptLanguageModel::new(&mut parameters, 65, &settings)?;
        // V*C + T*C + L*(12*C*C + 13*C) + 2*C + C*V + V.
        assert_eq!(parameters.parameter_count(), 10_788_929);
        let input_token_tensor: Tensor = Tensor::new(&[[0u32, 1, 2]], &Device::Cpu)?;
        let output_logits: Tensor = language_model.forward(&input_token_tensor, false, &mut RandomNumberGenerator::new(0))?;
        assert_eq!(output_logits.dims(), [1, 3, 65]);
        assert!(
            output_logits
                .flatten_all()?
                .to_vec1::<f32>()?
                .iter()
                .all(|logit_value: &f32| -> bool { logit_value.is_finite() })
        );
        Ok(())
    }
}
