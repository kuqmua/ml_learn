use crate::{
    data,
    model::{GptLanguageModel, ModelParameters, mean_next_token_loss},
    random::RandomNumberGenerator,
    settings::Settings,
};
use candle_core::{Device, Result, Tensor};
#[cfg(test)]
use candle_nn::ParamsAdamW;
use candle_nn::{AdamW, Optimizer};

// При каждом вызове random_seed оценки задаёт те же окна split_token_ids.
// Этот генератор не расходует числа из batch_random_number_generator и dropout_random_number_generator.
// Ошибка validation используется для наблюдения за обучением; отдельной финальной тестовой выборки нет.
pub(crate) fn estimate_mean_loss(gpt_language_model: &GptLanguageModel, split_token_ids: &[u32], settings: &Settings, random_seed: u64) -> Result<f32> {
    let mut random_number_generator: RandomNumberGenerator = RandomNumberGenerator::new(random_seed);
    let mut accumulated_loss: f32 = 0f32;
    for _ in 0..settings.evaluation_batch_count() {
        let (input_token_tensor, target_token_tensor): (Tensor, Tensor) =
            data::sample_text_batch(split_token_ids, settings.batch_size(), settings.block_size(), &mut random_number_generator)?;
        let next_token_logits: Tensor = gpt_language_model.forward(&input_token_tensor, false, &mut random_number_generator)?;
        accumulated_loss += mean_next_token_loss(&next_token_logits, &target_token_tensor)?.to_scalar::<f32>()?;
    }
    Ok(accumulated_loss / settings.evaluation_batch_count() as f32)
}

// Выбираем следующий ID случайно по next_token_logits при температуре 1, а не берём максимум.
// Вычитаем maximum_logit перед exp; sampling_weights и cumulative_weight считаем в f64.
fn sample_next_token_id(next_token_logits: &[f32], random_number_generator: &mut RandomNumberGenerator) -> Result<u32> {
    if next_token_logits.is_empty() || next_token_logits.iter().any(|logit_value: &f32| -> bool { !logit_value.is_finite() }) {
        candle_core::bail!("Некорректные logits при sampling");
    }
    let maximum_logit: f64 = next_token_logits.iter().copied().fold(f32::NEG_INFINITY, f32::max) as f64;
    let sampling_weights: Vec<f64> = next_token_logits
        .iter()
        .map(|&logit_value: &f32| -> f64 { (logit_value as f64 - maximum_logit).exp() })
        .collect();
    let sampling_threshold: f64 = random_number_generator.uniform_probability() * sampling_weights.iter().sum::<f64>();
    let mut cumulative_weight: f64 = 0.0;
    let sampled_token_index: usize = sampling_weights
        .iter()
        .position(|sampling_weight: &f64| -> bool {
            cumulative_weight += sampling_weight;
            sampling_threshold < cumulative_weight
        })
        .unwrap_or(sampling_weights.len() - 1);
    Ok(sampled_token_index as u32)
}

pub(crate) fn generate_token_ids(
    gpt_language_model: &GptLanguageModel,
    prompt_token_ids: &[u32],
    new_token_count: usize,
    random_seed: u64,
) -> Result<Vec<u32>> {
    if prompt_token_ids.is_empty() {
        candle_core::bail!("Prompt не может быть пустым");
    }
    let mut generated_token_ids: Vec<u32> = prompt_token_ids.to_vec();
    let mut random_number_generator: RandomNumberGenerator = RandomNumberGenerator::new(random_seed);
    for _ in 0..new_token_count {
        // В модель передаём не более block_size последних ID, а generated_token_ids сохраняем целиком.
        // Внутри GptLanguageModel позиции снова нумеруются от 0 до длины окна минус 1.
        // Как в gpt.py, пересчитываем всё окно: кеш ключей и значений внимания не используется.
        let context_token_ids: &[u32] = &generated_token_ids[generated_token_ids.len().saturating_sub(gpt_language_model.maximum_context_length())..];
        let input_token_tensor: Tensor = Tensor::from_slice(context_token_ids, (1, context_token_ids.len()), &Device::Cpu)?;
        let next_token_logits: Tensor = gpt_language_model.forward(&input_token_tensor, false, &mut random_number_generator)?;
        let last_position_logits: Vec<f32> = next_token_logits.narrow(1, context_token_ids.len() - 1, 1)?.flatten_all()?.to_vec1::<f32>()?;
        generated_token_ids.push(sample_next_token_id(&last_position_logits, &mut random_number_generator)?);
    }
    Ok(generated_token_ids)
}

pub(crate) fn run(settings: &Settings, character_vocabulary: &data::CharacterVocabulary, corpus_token_ids: &[u32]) -> Result<()> {
    settings.validate()?;
    let training_started_at: std::time::Instant = std::time::Instant::now();
    let (training_token_ids, validation_token_ids): (&[u32], &[u32]) = data::split_training_and_validation(corpus_token_ids);
    let prompt_token_ids: Vec<u32> = character_vocabulary.encode(settings.prompt_text())?;
    if prompt_token_ids.is_empty() {
        candle_core::bail!("Prompt не может быть пустым");
    }
    let mut model_parameters: ModelParameters = ModelParameters::new(settings.random_seed());
    let gpt_language_model: GptLanguageModel = GptLanguageModel::new(&mut model_parameters, character_vocabulary.characters_by_token_id.len(), settings)?;
    println!("\nGPT seed={} device=CPU dtype=f32", settings.random_seed());
    println!(
        "B={} T={} steps={} lr={} weight_decay={}",
        settings.batch_size(),
        settings.block_size(),
        settings.training_step_count(),
        settings.optimizer_parameters().lr,
        settings.optimizer_parameters().weight_decay
    );
    println!(
        "C={} H={} L={} dropout={}",
        settings.embedding_dimension(),
        settings.attention_head_count(),
        settings.transformer_block_count(),
        settings.dropout_probability()
    );
    println!(
        "Параметров: {}; uniform loss ln(V)={:.4}",
        model_parameters.parameter_count(),
        (character_vocabulary.characters_by_token_id.len() as f32).ln()
    );
    // adamw_settings в начале main.rs содержат проверенные типы параметров AdamW.
    // optimizer_parameters() переводит их в библиотечный ParamsAdamW для создания adamw_optimizer.
    let mut adamw_optimizer: AdamW = AdamW::new(model_parameters.trainable_variables(), settings.optimizer_parameters())?;
    let mut batch_random_number_generator: RandomNumberGenerator = RandomNumberGenerator::new(settings.random_seed().wrapping_add(1));
    let mut dropout_random_number_generator: RandomNumberGenerator = RandomNumberGenerator::new(settings.random_seed().wrapping_add(2));
    (0usize..=settings.training_step_count()).try_for_each(|training_step_index: usize| -> Result<()> {
        if training_step_index.is_multiple_of(settings.evaluation_interval()) || training_step_index == settings.training_step_count() {
            let training_loss: f32 = estimate_mean_loss(&gpt_language_model, training_token_ids, settings, settings.random_seed().wrapping_add(3))?;
            let validation_loss: f32 = estimate_mean_loss(&gpt_language_model, validation_token_ids, settings, settings.random_seed().wrapping_add(4))?;
            if !training_loss.is_finite() || !validation_loss.is_finite() {
                candle_core::bail!("Loss стал нечисловым");
            }
            println!(
                "step {training_step_index:5}: train {training_loss:.4}, val {validation_loss:.4}, elapsed {:.2}s",
                training_started_at.elapsed().as_secs_f64()
            );
        }
        if training_step_index < settings.training_step_count() {
            let (input_token_tensor, target_token_tensor): (Tensor, Tensor) = data::sample_text_batch(
                training_token_ids,
                settings.batch_size(),
                settings.block_size(),
                &mut batch_random_number_generator,
            )?;
            let next_token_logits: Tensor = gpt_language_model.forward(&input_token_tensor, true, &mut dropout_random_number_generator)?;
            let next_token_loss: Tensor = mean_next_token_loss(&next_token_logits, &target_token_tensor)?;
            // gpt_language_model.forward создаёт новый граф, а backward_step вычисляет градиенты и обновляет веса.
            // Candle возвращает новый GradStore для каждого обратного прохода; отдельный zero_grad не требуется.
            adamw_optimizer.backward_step(&next_token_loss)?;
        }
        Ok(())
    })?;
    let generated_token_ids: Vec<u32> = generate_token_ids(
        &gpt_language_model,
        &prompt_token_ids,
        settings.generated_token_count(),
        settings.random_seed().wrapping_add(5),
    )?;
    println!(
        "\n--- Генерация: prompt + {} новых символов ---\n{}\n--- конец ---",
        settings.generated_token_count(),
        character_vocabulary.decode(&generated_token_ids)?
    );
    println!("Всего {:.2}s", training_started_at.elapsed().as_secs_f64());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{AttentionHeadCount, BlockSize, DropoutProbability, EmbeddingDimension, TransformerBlockCount, test_settings};
    #[test]
    fn generation_crops_context_and_is_seeded() -> Result<()> {
        let settings: Settings = Settings {
            block_size: BlockSize::try_from(3)?,
            embedding_dimension: EmbeddingDimension::try_from(8)?,
            attention_head_count: AttentionHeadCount::try_from(2)?,
            transformer_block_count: TransformerBlockCount::try_from(1)?,
            dropout_probability: DropoutProbability::try_from(0.5)?,
            ..test_settings()?
        };
        let mut model_parameters: ModelParameters = ModelParameters::new(1337);
        let gpt_language_model: GptLanguageModel = GptLanguageModel::new(&mut model_parameters, 4, &settings)?;
        let prompt_token_ids: [u32; 6] = [0, 1, 2, 3, 2, 1];
        let generated_token_ids: Vec<u32> = generate_token_ids(&gpt_language_model, &prompt_token_ids, 20, 9)?;
        assert_eq!(&generated_token_ids[..prompt_token_ids.len()], &prompt_token_ids);
        assert_eq!(generated_token_ids.len(), 26);
        assert!(generated_token_ids.iter().all(|&token_id: &u32| -> bool { token_id < 4 }));
        assert_eq!(generated_token_ids, generate_token_ids(&gpt_language_model, &prompt_token_ids, 20, 9)?);
        assert!(generate_token_ids(&gpt_language_model, &[], 2, 0).is_err());
        Ok(())
    }
    #[test]
    fn trains_full_gpt_on_small_repeating_corpus() -> Result<()> {
        let settings: Settings = Settings {
            block_size: BlockSize::try_from(4)?,
            embedding_dimension: EmbeddingDimension::try_from(8)?,
            attention_head_count: AttentionHeadCount::try_from(2)?,
            transformer_block_count: TransformerBlockCount::try_from(1)?,
            dropout_probability: DropoutProbability::try_from(0.0)?,
            ..test_settings()?
        };
        let mut model_parameters: ModelParameters = ModelParameters::new(1337);
        let gpt_language_model: GptLanguageModel = GptLanguageModel::new(&mut model_parameters, 4, &settings)?;
        let mut adamw_optimizer: AdamW = AdamW::new(
            model_parameters.trainable_variables(),
            ParamsAdamW {
                lr: 0.02,
                weight_decay: 0.0,
                ..Default::default()
            },
        )?;
        let input_token_tensor: Tensor = Tensor::new(&[[0u32, 1, 2, 3], [0, 1, 2, 3]], &Device::Cpu)?;
        let target_token_tensor: Tensor = Tensor::new(&[[1u32, 2, 3, 0], [1, 2, 3, 0]], &Device::Cpu)?;
        let mut random_number_generator: RandomNumberGenerator = RandomNumberGenerator::new(1);
        let loss_before_training: f32 = mean_next_token_loss(
            &gpt_language_model.forward(&input_token_tensor, false, &mut random_number_generator)?,
            &target_token_tensor,
        )?
        .to_scalar::<f32>()?;
        for _ in 0..60 {
            let next_token_loss: Tensor = mean_next_token_loss(
                &gpt_language_model.forward(&input_token_tensor, true, &mut random_number_generator)?,
                &target_token_tensor,
            )?;
            adamw_optimizer.backward_step(&next_token_loss)?;
        }
        let loss_after_training: f32 = mean_next_token_loss(
            &gpt_language_model.forward(&input_token_tensor, false, &mut random_number_generator)?,
            &target_token_tensor,
        )?
        .to_scalar::<f32>()?;
        assert!(
            loss_after_training < 0.15 && loss_after_training < loss_before_training * 0.2,
            "before={loss_before_training}, after={loss_after_training}"
        );
        Ok(())
    }
}
