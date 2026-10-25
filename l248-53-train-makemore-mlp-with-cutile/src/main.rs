//! GPU-вариант: все шаги SGD исполняются в заранее выделенной памяти.
#[cutile::module]
mod kernels {
    use cutile::core::*;
    #[cutile::entry()]
    fn build_context_embedding_batch(
        context_embeddings_flattened: &mut Tensor<f32, { [1, 32] }>,
        batch_character_indexes: &mut Tensor<i32, { [1, 4] }>,
        all_character_index_pairs: &Tensor<i32, { [-1, 4] }>,
        character_embeddings: &Tensor<f32, { [32, 8] }>,
        training_step: &Tensor<i32, { [1] }>,
        batch_size: i32,
        sample_random_examples: i32,
    ) {
        let tile_block_indexes = get_tile_block_id();
        let training_step_index: i32 = tile_to_scalar(
            training_step
                .load_tile(shape![1], [0])
                .reshape(shape![]),
        );
        let mut example_index = training_step_index * batch_size + tile_block_indexes.0;
        if sample_random_examples != 0 {
            // Хеш счётчика: выборка с возвращением без CPU-копий на каждом шаге.
            let hashed_counter_bits: Tile<u32, { [] }> =
                bitcast(scalar_to_tile::<i32>(example_index));
            let mut hashed_counter: u32 = tile_to_scalar(hashed_counter_bits);
            hashed_counter = hashed_counter ^ (hashed_counter >> 16u32);
            hashed_counter = hashed_counter * 2146121005u32;
            hashed_counter = hashed_counter ^ (hashed_counter >> 15u32);
            // 0x846ca68b: JIT 0.4 не разбирает целый литерал > i32::MAX.
            hashed_counter = hashed_counter * (1110856517u32 * 2u32 + 1u32);
            hashed_counter = hashed_counter ^ (hashed_counter >> 16u32);
            let example_count_bits: Tile<u32, { [] }> = bitcast(
                scalar_to_tile::<i32>(all_character_index_pairs.shape()[0]),
            );
            let example_count: u32 = tile_to_scalar(example_count_bits);
            let example_index_bits: Tile<i32, { [] }> = bitcast(
                scalar_to_tile::<u32>(hashed_counter % example_count),
            );
            example_index = tile_to_scalar(example_index_bits);
        } else {
            example_index = example_index % all_character_index_pairs.shape()[0];
        }
        let context_and_next_character_indexes =
            all_character_index_pairs.load_tile(shape![1, 4], [example_index, 0]);
        batch_character_indexes.store(context_and_next_character_indexes);
        let mut embedding_batch_tiles =
            context_embeddings_flattened.partition_mut(shape![1, 8]);
        for context_slot in 0i32..4i32 {
            let zero_embedding: Tile<f32, { [1, 8] }> =
                constant(0.0f32, shape![1, 8]);
            let mut embedding_values: Tile<f32, { [1, 8] }> = zero_embedding;
            if context_slot < 3 {
                let vocabulary_character_index: i32 = tile_to_scalar(
                    all_character_index_pairs
                        .load_tile(shape![1, 1], [example_index, context_slot])
                        .reshape(shape![]),
                );
                embedding_values = character_embeddings
                    .load_tile(shape![1, 8], [vocabulary_character_index, 0]);
            }
            embedding_batch_tiles.store(embedding_values, [0, context_slot]);
        }
    }
    // Универсальный GEMM; транспонирование выполняется внутри тайла.
    // MATRIX_OPERATION: 1 — прибавить смещения и применить tanh; 2 — умножить градиент на производную tanh; 0 — только матричное произведение.
    #[cutile::entry()]
    fn multiply_matrices<
        const TRANSPOSE_LEFT_MATRIX: i32,
        const TRANSPOSE_RIGHT_MATRIX: i32,
        const MATRIX_OPERATION: i32,
    >(
        output_matrix: &mut Tensor<f32, { [32, 32] }>,
        left_matrix: &Tensor<f32, { [-1, -1] }>,
        right_matrix: &Tensor<f32, { [-1, -1] }>,
        layer_biases: &Tensor<f32, { [-1] }>,
        first_layer_activations: &Tensor<f32, { [-1, -1] }>,
        shared_dimension_size: i32,
    ) {
        let tile_block_indexes = get_tile_block_id();
        let zero_matrix: Tile<f32, { [32, 32] }> =
            constant(0.0f32, shape![32, 32]);
        let mut matrix_product_sum = zero_matrix;
        for shared_dimension_tile_index in 0i32..(shared_dimension_size / 32) {
            let left_matrix_tile = if TRANSPOSE_LEFT_MATRIX != 0 {
                left_matrix
                    .load_tile(
                        shape![32, 32],
                        [shared_dimension_tile_index, tile_block_indexes.0],
                    )
                    .transpose()
            } else {
                left_matrix.load_tile(
                    shape![32, 32],
                    [tile_block_indexes.0, shared_dimension_tile_index],
                )
            };
            let right_matrix_tile = if TRANSPOSE_RIGHT_MATRIX != 0 {
                right_matrix
                    .load_tile(
                        shape![32, 32],
                        [tile_block_indexes.1, shared_dimension_tile_index],
                    )
                    .transpose()
            } else {
                right_matrix.load_tile(
                    shape![32, 32],
                    [shared_dimension_tile_index, tile_block_indexes.1],
                )
            };
            matrix_product_sum =
                mma(left_matrix_tile, right_matrix_tile, matrix_product_sum);
        }
        if MATRIX_OPERATION == 1 {
            let layer_bias_values = layer_biases
                .load_tile(shape![32], [tile_block_indexes.1])
                .reshape(shape![1, 32]);
            matrix_product_sum = tanh(
                matrix_product_sum + layer_bias_values.broadcast(shape![32, 32]),
            );
        }
        if MATRIX_OPERATION == 2 {
            let first_layer_activation_values = first_layer_activations.load_like(output_matrix);
            let ones: Tile<f32, { [32, 32] }> =
                constant(1.0f32, shape![32, 32]);
            matrix_product_sum = matrix_product_sum
                * (ones - first_layer_activation_values * first_layer_activation_values);
        }
        output_matrix.store(matrix_product_sum);
    }
    #[cutile::entry()]
    fn compute_score_gradients_and_negative_log_likelihood(
        next_character_score_gradients: &mut Tensor<f32, { [32, 32] }>,
        negative_log_likelihood_losses: &mut Tensor<f32, { [32, 1] }>,
        character_scores_without_bias: &Tensor<f32, { [-1, 32] }>,
        batch_character_indexes: &Tensor<i32, { [-1, 4] }>,
        second_layer_biases: &Tensor<f32, { [32] }>,
        batch_size_as_f32: f32,
    ) {
        let tile_block_indexes = get_tile_block_id();
        let vocabulary_character_indexes =
            iota::<i32, { [32] }>(shape![32])
                .reshape(shape![1, 32])
                .broadcast(shape![32, 32]);
        let vocabulary_size: Tile<i32, { [32, 32] }> =
            constant(27i32, shape![32, 32]);
        let is_vocabulary_character = cmpi(
            vocabulary_character_indexes,
            vocabulary_size,
            predicate::LessThan,
        );
        let character_scores = character_scores_without_bias
            .load_like(next_character_score_gradients)
            + second_layer_biases
                .load_tile(shape![32], [0])
                .reshape(shape![1, 32])
                .broadcast(shape![32, 32]);
        let padding_character_score: Tile<f32, { [32, 32] }> =
            constant(-1.0e30f32, shape![32, 32]);
        let masked_character_scores = select(
            is_vocabulary_character,
            character_scores,
            padding_character_score,
        );
        let maximum_character_score: Tile<f32, { [32] }> =
            reduce_max(masked_character_scores, 1);
        let maximum_character_score = maximum_character_score.reshape(shape![32, 1]);
        let exponentials_from_character_scores_minus_maximum = exp(
            masked_character_scores
                - maximum_character_score.broadcast(shape![32, 32]),
        );
        let exponentials_from_character_scores_minus_maximum_sum: Tile<
            f32,
            { [32] },
        > = reduce_sum(exponentials_from_character_scores_minus_maximum, 1);
        let exponentials_from_character_scores_minus_maximum_sum =
            exponentials_from_character_scores_minus_maximum_sum
                .reshape(shape![32, 1]);
        let target_character_indexes = batch_character_indexes
            .load_tile(shape![32, 1], [tile_block_indexes.0, 3])
            .broadcast(shape![32, 32]);
        let is_target_character = cmpi(
            vocabulary_character_indexes,
            target_character_indexes,
            predicate::Equal,
        );
        let zeros: Tile<f32, { [32, 32] }> =
            constant(0.0f32, shape![32, 32]);
        let ones: Tile<f32, { [32, 32] }> =
            constant(1.0f32, shape![32, 32]);
        let target_character_one_hot = select(is_target_character, ones, zeros);
        let batch_size_tile: Tile<f32, { [32, 32] }> =
            batch_size_as_f32.broadcast(shape![32, 32]);
        next_character_score_gradients.store(
            (exponentials_from_character_scores_minus_maximum
                / exponentials_from_character_scores_minus_maximum_sum
                    .broadcast(shape![32, 32])
                - target_character_one_hot)
                / batch_size_tile,
        );
        let target_character_score: Tile<f32, { [32] }> =
            reduce_sum(masked_character_scores * target_character_one_hot, 1);
        let target_character_score = target_character_score.reshape(shape![32, 1]);
        negative_log_likelihood_losses.store(
            maximum_character_score
                + log(exponentials_from_character_scores_minus_maximum_sum)
                - target_character_score,
        );
    }
    #[cutile::entry()]
    fn accumulate_character_embedding_gradients(
        character_embeddings_gradients: &mut Tensor<f32, { [1, 8] }>,
        context_embeddings_flattened_gradients: &Tensor<f32, { [-1, 32] }>,
        batch_character_indexes: &Tensor<i32, { [-1, 4] }>,
        batch_size: i32,
    ) {
        let tile_block_indexes = get_tile_block_id();
        let zero_embedding: Tile<f32, { [8] }> =
            constant(0.0f32, shape![8]);
        let mut embedding_gradient_sum = zero_embedding;
        for batch_tile_index in 0i32..(batch_size / 32) {
            for context_slot in 0i32..3i32 {
                let context_character_indexes = batch_character_indexes.load_tile(
                    shape![32, 1],
                    [batch_tile_index, context_slot],
                );
                let current_vocabulary_character_index: Tile<i32, { [32, 1] }> =
                    tile_block_indexes.0.broadcast(shape![32, 1]);
                let is_current_character = cmpi(
                    context_character_indexes,
                    current_vocabulary_character_index,
                    predicate::Equal,
                )
                .broadcast(shape![32, 8]);
                let context_coordinate_gradients = context_embeddings_flattened_gradients
                    .load_tile(
                        shape![32, 8],
                        [batch_tile_index, context_slot],
                    );
                let zeros: Tile<f32, { [32, 8] }> =
                    constant(0.0f32, shape![32, 8]);
                embedding_gradient_sum = embedding_gradient_sum
                    + reduce_sum(
                        select(
                            is_current_character,
                            context_coordinate_gradients,
                            zeros,
                        ),
                        0,
                    );
            }
        }
        character_embeddings_gradients
            .store(embedding_gradient_sum.reshape(shape![1, 8]));
    }
    #[cutile::entry()]
    fn sum_bias_gradients(
        bias_gradients: &mut Tensor<f32, { [32] }>,
        layer_score_gradients: &Tensor<f32, { [-1, -1] }>,
        batch_size: i32,
    ) {
        let zero_bias: Tile<f32, { [32] }> =
            constant(0.0f32, shape![32]);
        let mut bias_gradient_sum = zero_bias;
        for batch_tile_index in 0i32..(batch_size / 32) {
            bias_gradient_sum = bias_gradient_sum
                + reduce_sum(
                    layer_score_gradients.load_tile(
                        shape![32, 32],
                        [batch_tile_index, get_tile_block_id().0],
                    ),
                    0,
                );
        }
        bias_gradients.store(bias_gradient_sum);
    }
    #[cutile::entry()]
    fn apply_vector_gradient_step(
        parameters: &mut Tensor<f32, { [32] }>,
        parameter_gradients: &Tensor<f32, { [-1] }>,
        training_step: &Tensor<i32, { [1] }>,
        training_steps: i32,
    ) {
        let training_step_index: i32 = tile_to_scalar(
            training_step
                .load_tile(shape![1], [0])
                .reshape(shape![]),
        );
        let mut learning_rate_value = 0.3f32;
        if training_step_index >= training_steps * 3 / 4 {
            learning_rate_value = 0.03f32;
        }
        let previous_parameters = parameters.load();
        let parameter_gradient_values = parameter_gradients.load_like(parameters);
        let learning_rate: Tile<f32, { [32] }> =
            learning_rate_value.broadcast(parameters.shape());
        parameters.store(previous_parameters - parameter_gradient_values * learning_rate);
    }
    #[cutile::entry()]
    fn apply_matrix_gradient_step(
        parameters: &mut Tensor<f32, { [8, 8] }>,
        parameter_gradients: &Tensor<f32, { [-1, -1] }>,
        training_step: &Tensor<i32, { [1] }>,
        training_steps: i32,
    ) {
        let training_step_index: i32 = tile_to_scalar(
            training_step
                .load_tile(shape![1], [0])
                .reshape(shape![]),
        );
        let mut learning_rate_value = 0.3f32;
        if training_step_index >= training_steps * 3 / 4 {
            learning_rate_value = 0.03f32;
        }
        let previous_parameters = parameters.load();
        let parameter_gradient_values = parameter_gradients.load_like(parameters);
        let learning_rate: Tile<f32, { [8, 8] }> =
            learning_rate_value.broadcast(parameters.shape());
        parameters.store(previous_parameters - parameter_gradient_values * learning_rate);
    }
    #[cutile::entry()]
    fn advance_training_step(training_step: &mut Tensor<i32, { [1] }>) {
        let previous_training_step = training_step.load();
        let step_increment: Tile<i32, { [1] }> =
            constant(1i32, shape![1]);
        training_step.store(previous_training_step + step_increment);
    }
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    // Три эмбеддинга по 8 координат дают 24 числа; ещё 8 нулей выравнивают вход для GPU.
    const PADDED_CONTEXT_EMBEDDING_SIZE: usize = 32;
    // Число компонентов результата первого полносвязного слоя.
    const FIRST_LAYER_SIZE: usize = 64;
    // Настоящие символы: точка и a–z. GPU-буферы имеют 32 позиции, лишние пять маскируются.
    const VOCABULARY_SIZE: usize = 27;
    struct TrainingConfiguration {
        // Число примеров, используемых в одном обновлении параметров.
        batch_size: usize,
        // Общее число обновлений параметров; один replay графа выполняет десять.
        training_steps: usize,
    }
    fn upload_tensor_to_gpu<T: cutile::DType>(
        host_values: Vec<T>,
        tensor_shape: &[usize],
        cuda_stream: &std::sync::Arc<cutile::cuda_core::Stream>,
    ) -> Result<cutile::tensor::Tensor<T>> {
        use cutile::cuda_async::device_operation::DeviceOp;
        use cutile::tensor::Reshape;
        Ok(
            cutile::api::copy_host_vec_to_device(&std::sync::Arc::new(host_values))
                .sync_on(cuda_stream)?
                .reshape(tensor_shape)?,
        )
    }
    // Каждая запись — три индекса предыдущих символов и индекс правильного следующего.
    struct VocabularyCharacterIndexPairs {
        // Плоская память: [previous_0, previous_1, previous_2, target, previous_0, ...].
        flattened_character_index_pairs: Vec<i32>,
    }
    impl VocabularyCharacterIndexPairs {
        fn example_count(&self) -> usize {
            self.flattened_character_index_pairs.len() / 4
        }
    }
    // По умолчанию удаляем копии до split, как в CPU-уроке.
    // Сохранение кратностей доступно только для воспроизведения бенчмарка.
    fn build_training_validation_test_datasets(random_generator: &mut rand::rngs::StdRng, unique_names: bool) -> [VocabularyCharacterIndexPairs; 3] {
        let mut name_groups = std::collections::BTreeMap::<&str, Vec<&str>>::new();
        for name in include_str!("../../l247-52-generate-names-with-makemore-mlp/data/names.txt").lines() {
            let group = name_groups.entry(name).or_default();
            if !unique_names || group.is_empty() {
                group.push(name);
            }
        }
        let mut name_groups: Vec<_> = name_groups.into_values().collect();
        rand::seq::SliceRandom::shuffle(name_groups.as_mut_slice(), random_generator);
        let unique_name_count = name_groups.len();
        let build_character_index_pairs = |name_groups: &[Vec<&str>]| {
            let mut flattened_character_index_pairs = Vec::new();
            for name in name_groups.iter().flatten() {
                let mut previous_vocabulary_character_indexes = [0; 3];
                for next_vocabulary_character_index in name.bytes().map(|character_byte| (character_byte - b'a' + 1) as i32).chain([0]) {
                    flattened_character_index_pairs.extend(previous_vocabulary_character_indexes);
                    flattened_character_index_pairs.push(next_vocabulary_character_index);
                    // Сдвигаем контекст влево, отбрасывая самый старый индекс.
                    for context_index in 0..previous_vocabulary_character_indexes.len() - 1 {
                        previous_vocabulary_character_indexes[context_index] = previous_vocabulary_character_indexes[context_index + 1];
                    }
                    previous_vocabulary_character_indexes[2] = next_vocabulary_character_index;
                }
            }
            VocabularyCharacterIndexPairs {
                flattened_character_index_pairs,
            }
        };
        [
            build_character_index_pairs(&name_groups[..unique_name_count * 8 / 10]),
            build_character_index_pairs(&name_groups[unique_name_count * 8 / 10..unique_name_count * 9 / 10]),
            build_character_index_pairs(&name_groups[unique_name_count * 9 / 10..]),
        ]
    }
    // Параметры модели на GPU; эти же формы используются для хранения их градиентов.
    struct Model {
        // C: [32, 8], обучаемые координаты символов; реальные индексы 0–26.
        character_embeddings: cutile::tensor::Tensor<f32>,
        // W1: [32, 64], от объединённых координат контекста к первому слою.
        first_layer_weights: cutile::tensor::Tensor<f32>,
        // b1: [64], смещения сумм первого слоя до tanh.
        first_layer_biases: cutile::tensor::Tensor<f32>,
        // W2: [64, 32], от активаций первого слоя к оценкам следующих символов.
        second_layer_weights: cutile::tensor::Tensor<f32>,
        // b2: [32], смещения оценок; последние пять позиций не участвуют в softmax.
        second_layer_biases: cutile::tensor::Tensor<f32>,
    }
    impl Model {
        fn new(
            random_generator: &mut rand::rngs::StdRng,
            cuda_stream: &std::sync::Arc<cutile::cuda_core::Stream>,
        ) -> Result<Self> {
            use cutile::cuda_async::device_operation::DeviceOp;
            let mut initialize_normal_values =
                |value_count: usize, standard_deviation: f32| -> Vec<f32> {
                    (0..value_count)
                        .map(|_| {
                            let normal_value: f32 = rand_distr::Distribution::sample(
                                &rand_distr::StandardNormal,
                                random_generator,
                            );
                            normal_value * standard_deviation
                        })
                        .collect()
                };
            let character_embeddings = initialize_normal_values(32 * 8, 1.0);
            let mut first_layer_weights = initialize_normal_values(
                PADDED_CONTEXT_EMBEDDING_SIZE * FIRST_LAYER_SIZE,
                (5.0 / 3.0) / (24.0f32).sqrt(),
            );
            first_layer_weights[24 * FIRST_LAYER_SIZE..].fill(0.0);
            let mut second_layer_weights = initialize_normal_values(FIRST_LAYER_SIZE * 32, 0.01);
            for second_layer_weight_row in second_layer_weights.chunks_mut(32) {
                second_layer_weight_row[VOCABULARY_SIZE..].fill(0.0);
            }
            Ok(Self {
                character_embeddings: upload_tensor_to_gpu(
                    character_embeddings,
                    &[32, 8],
                    cuda_stream,
                )?,
                first_layer_weights: upload_tensor_to_gpu(
                    first_layer_weights,
                    &[PADDED_CONTEXT_EMBEDDING_SIZE, FIRST_LAYER_SIZE],
                    cuda_stream,
                )?,
                first_layer_biases: cutile::api::zeros(&[FIRST_LAYER_SIZE]).sync_on(cuda_stream)?,
                second_layer_weights: upload_tensor_to_gpu(
                    second_layer_weights,
                    &[FIRST_LAYER_SIZE, 32],
                    cuda_stream,
                )?,
                second_layer_biases: cutile::api::zeros(&[32]).sync_on(cuda_stream)?,
            })
        }
        fn zeroed_parameter_tensors(
            cuda_stream: &std::sync::Arc<cutile::cuda_core::Stream>,
        ) -> Result<Self> {
            use cutile::cuda_async::device_operation::DeviceOp;
            Ok(Self {
                character_embeddings: cutile::api::zeros(&[32, 8]).sync_on(cuda_stream)?,
                first_layer_weights: cutile::api::zeros(&[
                    PADDED_CONTEXT_EMBEDDING_SIZE,
                    FIRST_LAYER_SIZE,
                ])
                .sync_on(cuda_stream)?,
                first_layer_biases: cutile::api::zeros(&[FIRST_LAYER_SIZE]).sync_on(cuda_stream)?,
                second_layer_weights: cutile::api::zeros(&[FIRST_LAYER_SIZE, 32])
                    .sync_on(cuda_stream)?,
                second_layer_biases: cutile::api::zeros(&[32]).sync_on(cuda_stream)?,
            })
        }
    }
    struct ForwardAndBackwardBuffers {
        // Вход: [batch_size, 32], три эмбеддинга подряд и нулевое дополнение.
        context_embeddings_flattened: cutile::tensor::Tensor<f32>,
        // Первый слой: [batch_size, 64], результаты tanh.
        first_layer_activations: cutile::tensor::Tensor<f32>,
        // Второй слой: [batch_size, 32], матричное произведение без b2.
        next_character_scores_without_bias: cutile::tensor::Tensor<f32>,
        // Производные среднего NLL по оценкам символов: (p - target) / batch_size.
        next_character_score_gradients: cutile::tensor::Tensor<f32>,
        // Производные по суммам первого слоя до tanh.
        first_layer_pre_activation_gradients: cutile::tensor::Tensor<f32>,
        // Производные по 32 входным координатам; реальные первые 24 идут в C.
        context_embeddings_flattened_gradients: cutile::tensor::Tensor<f32>,
        // Записи выбранного батча: [previous_0, previous_1, previous_2, target].
        batch_character_indexes: cutile::tensor::Tensor<i32>,
        // NLL каждого примера: [batch_size, 1], ещё без усреднения.
        negative_log_likelihood_losses: cutile::tensor::Tensor<f32>,
    }
    impl ForwardAndBackwardBuffers {
        fn new(
            batch_size: usize,
            cuda_stream: &std::sync::Arc<cutile::cuda_core::Stream>,
        ) -> Result<Self> {
            use cutile::cuda_async::device_operation::DeviceOp;
            Ok(Self {
                context_embeddings_flattened: cutile::api::zeros(&[
                    batch_size,
                    PADDED_CONTEXT_EMBEDDING_SIZE,
                ])
                .sync_on(cuda_stream)?,
                first_layer_activations: cutile::api::zeros(&[batch_size, FIRST_LAYER_SIZE])
                    .sync_on(cuda_stream)?,
                next_character_scores_without_bias: cutile::api::zeros(&[batch_size, 32])
                    .sync_on(cuda_stream)?,
                next_character_score_gradients: cutile::api::zeros(&[batch_size, 32])
                    .sync_on(cuda_stream)?,
                first_layer_pre_activation_gradients: cutile::api::zeros(&[
                    batch_size,
                    FIRST_LAYER_SIZE,
                ])
                .sync_on(cuda_stream)?,
                context_embeddings_flattened_gradients: cutile::api::zeros(&[
                    batch_size,
                    PADDED_CONTEXT_EMBEDDING_SIZE,
                ])
                .sync_on(cuda_stream)?,
                batch_character_indexes: cutile::api::zeros(&[batch_size, 4])
                    .sync_on(cuda_stream)?,
                negative_log_likelihood_losses: cutile::api::zeros(&[batch_size, 1])
                    .sync_on(cuda_stream)?,
            })
        }
    }
    // Записывает GPU-операции прямого прохода и расчёта NLL/градиентов оценок в CUDA Graph.
    fn record_forward_pass(
        cuda_graph_scope: &cutile::cuda_async::cuda_graph::Scope,
        model: &Model,
        buffers: &mut ForwardAndBackwardBuffers,
        all_character_index_pairs: &cutile::tensor::Tensor<i32>,
        training_step: &cutile::tensor::Tensor<i32>,
        batch_size: usize,
        sample_random_examples: bool,
    ) -> std::result::Result<(), cutile::cuda_async::error::DeviceError> {
        cuda_graph_scope.record(kernels::build_context_embedding_batch(
            cutile::tensor::PartitionMut::partition(
                &mut buffers.context_embeddings_flattened,
                [1, 32],
            ),
            cutile::tensor::PartitionMut::partition(&mut buffers.batch_character_indexes, [1, 4]),
            all_character_index_pairs,
            &model.character_embeddings,
            training_step,
            batch_size as i32,
            i32::from(sample_random_examples),
        ))?;
        cuda_graph_scope.record(cutile::tile_kernel::TileKernel::generics(
            kernels::multiply_matrices(
                cutile::tensor::PartitionMut::partition(
                    &mut buffers.first_layer_activations,
                    [32, 32],
                ),
                &buffers.context_embeddings_flattened,
                &model.first_layer_weights,
                &model.first_layer_biases,
                &buffers.context_embeddings_flattened,
                PADDED_CONTEXT_EMBEDDING_SIZE as i32,
            ),
            vec!["0".into(), "0".into(), "1".into()],
        ))?;
        cuda_graph_scope.record(cutile::tile_kernel::TileKernel::generics(
            kernels::multiply_matrices(
                cutile::tensor::PartitionMut::partition(
                    &mut buffers.next_character_scores_without_bias,
                    [32, 32],
                ),
                &buffers.first_layer_activations,
                &model.second_layer_weights,
                &model.second_layer_biases,
                &buffers.context_embeddings_flattened,
                FIRST_LAYER_SIZE as i32,
            ),
            vec!["0".into(), "0".into(), "0".into()],
        ))?;
        cuda_graph_scope.record(
            kernels::compute_score_gradients_and_negative_log_likelihood(
                cutile::tensor::PartitionMut::partition(
                    &mut buffers.next_character_score_gradients,
                    [32, 32],
                ),
                cutile::tensor::PartitionMut::partition(
                    &mut buffers.negative_log_likelihood_losses,
                    [32, 1],
                ),
                &buffers.next_character_scores_without_bias,
                &buffers.batch_character_indexes,
                &model.second_layer_biases,
                batch_size as f32,
            ),
        )?;
        Ok(())
    }
    // Записывает обратный проход: градиенты W2, b2, W1, b1 и координат символов C.
    fn record_backward_pass(
        cuda_graph_scope: &cutile::cuda_async::cuda_graph::Scope,
        model: &Model,
        buffers: &mut ForwardAndBackwardBuffers,
        model_gradients: &mut Model,
        batch_size: usize,
    ) -> std::result::Result<(), cutile::cuda_async::error::DeviceError> {
        cuda_graph_scope.record(cutile::tile_kernel::TileKernel::generics(
            kernels::multiply_matrices(
                cutile::tensor::PartitionMut::partition(
                    &mut buffers.first_layer_pre_activation_gradients,
                    [32, 32],
                ),
                &buffers.next_character_score_gradients,
                &model.second_layer_weights,
                &model.first_layer_biases,
                &buffers.first_layer_activations,
                32,
            ),
            vec!["0".into(), "1".into(), "2".into()],
        ))?;
        cuda_graph_scope.record(cutile::tile_kernel::TileKernel::generics(
            kernels::multiply_matrices(
                cutile::tensor::PartitionMut::partition(
                    &mut buffers.context_embeddings_flattened_gradients,
                    [32, 32],
                ),
                &buffers.first_layer_pre_activation_gradients,
                &model.first_layer_weights,
                &model.first_layer_biases,
                &buffers.context_embeddings_flattened,
                FIRST_LAYER_SIZE as i32,
            ),
            vec!["0".into(), "1".into(), "0".into()],
        ))?;
        cuda_graph_scope.record(cutile::tile_kernel::TileKernel::generics(
            kernels::multiply_matrices(
                cutile::tensor::PartitionMut::partition(
                    &mut model_gradients.second_layer_weights,
                    [32, 32],
                ),
                &buffers.first_layer_activations,
                &buffers.next_character_score_gradients,
                &model.second_layer_biases,
                &buffers.context_embeddings_flattened,
                batch_size as i32,
            ),
            vec!["1".into(), "0".into(), "0".into()],
        ))?;
        cuda_graph_scope.record(cutile::tile_kernel::TileKernel::generics(
            kernels::multiply_matrices(
                cutile::tensor::PartitionMut::partition(
                    &mut model_gradients.first_layer_weights,
                    [32, 32],
                ),
                &buffers.context_embeddings_flattened,
                &buffers.first_layer_pre_activation_gradients,
                &model.first_layer_biases,
                &buffers.context_embeddings_flattened,
                batch_size as i32,
            ),
            vec!["1".into(), "0".into(), "0".into()],
        ))?;
        cuda_graph_scope.record(kernels::accumulate_character_embedding_gradients(
            cutile::tensor::PartitionMut::partition(
                &mut model_gradients.character_embeddings,
                [1, 8],
            ),
            &buffers.context_embeddings_flattened_gradients,
            &buffers.batch_character_indexes,
            batch_size as i32,
        ))?;
        cuda_graph_scope.record(kernels::sum_bias_gradients(
            cutile::tensor::PartitionMut::partition(&mut model_gradients.first_layer_biases, [32]),
            &buffers.first_layer_pre_activation_gradients,
            batch_size as i32,
        ))?;
        cuda_graph_scope.record(kernels::sum_bias_gradients(
            cutile::tensor::PartitionMut::partition(&mut model_gradients.second_layer_biases, [32]),
            &buffers.next_character_score_gradients,
            batch_size as i32,
        ))?;
        Ok(())
    }
    // Записывает вычитание learning_rate × gradient из всех параметров и увеличение номера шага.
    fn record_parameter_updates(
        cuda_graph_scope: &cutile::cuda_async::cuda_graph::Scope,
        model: &mut Model,
        model_gradients: &Model,
        training_step: &mut cutile::tensor::Tensor<i32>,
        training_steps: usize,
    ) -> std::result::Result<(), cutile::cuda_async::error::DeviceError> {
        for (parameter_tensor, gradient_tensor) in [
            (
                &mut model.character_embeddings,
                &model_gradients.character_embeddings,
            ),
            (
                &mut model.first_layer_weights,
                &model_gradients.first_layer_weights,
            ),
            (
                &mut model.second_layer_weights,
                &model_gradients.second_layer_weights,
            ),
        ] {
            cuda_graph_scope.record(kernels::apply_matrix_gradient_step(
                cutile::tensor::PartitionMut::partition(parameter_tensor, [8, 8]),
                gradient_tensor,
                &*training_step,
                training_steps as i32,
            ))?;
        }
        for (parameter_tensor, gradient_tensor) in [
            (
                &mut model.first_layer_biases,
                &model_gradients.first_layer_biases,
            ),
            (
                &mut model.second_layer_biases,
                &model_gradients.second_layer_biases,
            ),
        ] {
            cuda_graph_scope.record(kernels::apply_vector_gradient_step(
                cutile::tensor::PartitionMut::partition(parameter_tensor, [32]),
                gradient_tensor,
                &*training_step,
                training_steps as i32,
            ))?;
        }
        cuda_graph_scope.record(kernels::advance_training_step(
            cutile::tensor::PartitionMut::partition(&mut *training_step, [1]),
        ))?;
        Ok(())
    }
    // Собирает CUDA Graph из десяти полных шагов обучения; replay повторяет эту последовательность.
    fn build_training_graph(
        model: &mut Model,
        buffers: &mut ForwardAndBackwardBuffers,
        model_gradients: &mut Model,
        training_character_index_pairs: &cutile::tensor::Tensor<i32>,
        training_step: &mut cutile::tensor::Tensor<i32>,
        training_configuration: &TrainingConfiguration,
        cuda_stream: &std::sync::Arc<cutile::cuda_core::Stream>,
    ) -> Result<cutile::cuda_async::cuda_graph::CudaGraph<()>> {
        let TrainingConfiguration {
            batch_size,
            training_steps,
        } = *training_configuration;
        Ok(cutile::cuda_async::cuda_graph::CudaGraph::scope(
            cuda_stream,
            |cuda_graph_scope| {
                // Несколько последовательных шагов в одном replay уменьшают затраты CPU.
                for _ in 0..10 {
                    record_forward_pass(
                        cuda_graph_scope,
                        model,
                        buffers,
                        training_character_index_pairs,
                        training_step,
                        batch_size,
                        true,
                    )?;
                    record_backward_pass(
                        cuda_graph_scope,
                        model,
                        buffers,
                        model_gradients,
                        batch_size,
                    )?;
                    record_parameter_updates(
                        cuda_graph_scope,
                        model,
                        model_gradients,
                        training_step,
                        training_steps,
                    )?;
                }
                Ok(())
            },
        )?)
    }
    // Оценивает средний NLL всего набора; последний неполный батч учитывается по фактической длине.
    fn evaluate_mean_negative_log_likelihood(
        model: &Model,
        character_index_pairs: &VocabularyCharacterIndexPairs,
        cuda_stream: &std::sync::Arc<cutile::cuda_core::Stream>,
    ) -> Result<f64> {
        use cutile::cuda_async::device_operation::DeviceOp;
        use cutile::tile_kernel::ToHostVecOp;
        let batch_size = 1024;
        let device_character_index_pairs = upload_tensor_to_gpu(
            character_index_pairs
                .flattened_character_index_pairs
                .clone(),
            &[character_index_pairs.example_count(), 4],
            cuda_stream,
        )?;
        let mut buffers = ForwardAndBackwardBuffers::new(batch_size, cuda_stream)?;
        let mut evaluation_batch_index = cutile::api::zeros::<i32>(&[1]).sync_on(cuda_stream)?;
        let evaluation_graph =
            cutile::cuda_async::cuda_graph::CudaGraph::scope(cuda_stream, |cuda_graph_scope| {
                record_forward_pass(
                    cuda_graph_scope,
                    model,
                    &mut buffers,
                    &device_character_index_pairs,
                    &evaluation_batch_index,
                    batch_size,
                    false,
                )?;
                cuda_graph_scope.record(kernels::advance_training_step(
                    cutile::tensor::PartitionMut::partition(&mut evaluation_batch_index, [1]),
                ))?;
                Ok(())
            })?;
        let mut total_negative_log_likelihood_loss = 0.0f64;
        for batch_start in (0..character_index_pairs.example_count()).step_by(batch_size) {
            evaluation_graph.launch().sync_on(cuda_stream)?;
            let batch_negative_log_likelihood_losses = buffers
                .negative_log_likelihood_losses
                .dup()
                .to_host_vec()
                .sync_on(cuda_stream)?;
            total_negative_log_likelihood_loss += batch_negative_log_likelihood_losses
                [..batch_size.min(character_index_pairs.example_count() - batch_start)]
                .iter()
                .map(|&negative_log_likelihood_loss| f64::from(negative_log_likelihood_loss))
                .sum::<f64>();
        }
        let mean_negative_log_likelihood_loss =
            total_negative_log_likelihood_loss / character_index_pairs.example_count() as f64;
        if !mean_negative_log_likelihood_loss.is_finite() {
            return Err("нечисловой loss при оценке".into());
        }
        Ok(mean_negative_log_likelihood_loss)
    }
    // Для последовательной генерации 20 имён копируем веса один раз: маленький
    // одиночный forward на CPU дешевле запуска GPU на каждый символ.
    fn generate_names(
        model: &Model,
        random_generator: &mut rand::rngs::StdRng,
        cuda_stream: &std::sync::Arc<cutile::cuda_core::Stream>,
    ) -> Result<()> {
        use cutile::cuda_async::device_operation::DeviceOp;
        use cutile::tile_kernel::ToHostVecOp;
        let character_embeddings = model
            .character_embeddings
            .dup()
            .to_host_vec()
            .sync_on(cuda_stream)?;
        let first_layer_weights = model
            .first_layer_weights
            .dup()
            .to_host_vec()
            .sync_on(cuda_stream)?;
        let first_layer_biases = model
            .first_layer_biases
            .dup()
            .to_host_vec()
            .sync_on(cuda_stream)?;
        let second_layer_weights = model
            .second_layer_weights
            .dup()
            .to_host_vec()
            .sync_on(cuda_stream)?;
        let second_layer_biases = model
            .second_layer_biases
            .dup()
            .to_host_vec()
            .sync_on(cuda_stream)?;
        for _ in 0..20 {
            let mut previous_vocabulary_character_indexes = [0usize; 3];
            let mut generated_name = String::new();
            for generated_character_index in 0..100 {
                let mut first_layer_activations = first_layer_biases.clone();
                for context_slot in 0..3 {
                    for embedding_coordinate_index in 0..8 {
                        let embedding_coordinate = character_embeddings
                            [previous_vocabulary_character_indexes[context_slot] * 8
                                + embedding_coordinate_index];
                        for first_layer_index in 0..FIRST_LAYER_SIZE {
                            first_layer_activations[first_layer_index] += embedding_coordinate
                                * first_layer_weights[(context_slot * 8
                                    + embedding_coordinate_index)
                                    * FIRST_LAYER_SIZE
                                    + first_layer_index];
                        }
                    }
                }
                for first_layer_activation in &mut first_layer_activations {
                    *first_layer_activation = first_layer_activation.tanh();
                }
                let mut next_character_scores = second_layer_biases[..VOCABULARY_SIZE].to_vec();
                for first_layer_index in 0..FIRST_LAYER_SIZE {
                    for vocabulary_character_index in 0..VOCABULARY_SIZE {
                        next_character_scores[vocabulary_character_index] += first_layer_activations
                            [first_layer_index]
                            * second_layer_weights
                                [first_layer_index * 32 + vocabulary_character_index];
                    }
                }
                let maximum_character_score = next_character_scores
                    .iter()
                    .copied()
                    .fold(f32::NEG_INFINITY, f32::max);
                let mut exponentials_from_character_scores_minus_maximum = next_character_scores;
                for exponential_from_character_score_minus_maximum in
                    &mut exponentials_from_character_scores_minus_maximum
                {
                    *exponential_from_character_score_minus_maximum =
                        (*exponential_from_character_score_minus_maximum - maximum_character_score)
                            .exp();
                }
                let mut remaining_sample_mass = rand::RngExt::random::<f32>(random_generator)
                    * exponentials_from_character_scores_minus_maximum
                        .iter()
                        .sum::<f32>();
                let mut sampled_vocabulary_character_index = VOCABULARY_SIZE - 1;
                for (vocabulary_character_index, unnormalized_character_probability) in
                    exponentials_from_character_scores_minus_maximum
                        .iter()
                        .enumerate()
                {
                    remaining_sample_mass -= unnormalized_character_probability;
                    if remaining_sample_mass <= 0.0 {
                        sampled_vocabulary_character_index = vocabulary_character_index;
                        break;
                    }
                }
                if sampled_vocabulary_character_index == 0 {
                    break;
                }
                generated_name.push((b'a' + sampled_vocabulary_character_index as u8 - 1) as char);
                // Сдвигаем контекст влево, отбрасывая самый старый индекс.
                for context_index in 0..previous_vocabulary_character_indexes.len() - 1 {
                    previous_vocabulary_character_indexes[context_index] = previous_vocabulary_character_indexes[context_index + 1];
                }
                previous_vocabulary_character_indexes[2] = sampled_vocabulary_character_index;
                if generated_character_index == 99 {
                    generated_name.push_str(" [обрезано]");
                }
            }
            println!("{generated_name}");
        }
        Ok(())
    }
    use cutile::cuda_async::device_operation::DeviceOp;
    use cutile::tile_kernel::ToHostVecOp;
    let program_started_at = std::time::Instant::now();
    let mut training_steps = 10_000usize;
    let mut batch_size = 128usize;
    let mut unique_train = false;
    let mut benchmark_repeats = false;
    let mut model_seed = None;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--unique-train" => unique_train = true,
            "--benchmark-repeats" => benchmark_repeats = true,
            "--model-seed" => model_seed = Some(arguments.next().ok_or("нет значения --model-seed")?.parse::<u64>()?),
            "--steps" => training_steps = arguments.next().ok_or("нет значения --steps")?.parse()?,
            "--batch" => batch_size = arguments.next().ok_or("нет значения --batch")?.parse()?,
            _ => return Err(format!("неизвестный GPU-аргумент: {argument}").into()),
        }
    }
    if training_steps == 0
        || !training_steps.is_multiple_of(10)
        || batch_size < 32
        || !batch_size.is_multiple_of(32)
        || batch_size > 4096
        || training_steps
            .checked_mul(batch_size)
            .is_none_or(|requested_example_count| requested_example_count > i32::MAX as usize)
    {
        return Err("steps должен быть положительным и кратным 10; batch — кратным 32 в 32..=4096; steps*batch <= i32::MAX".into());
    }
    let device = cutile::cuda_core::Device::new(0)?;
    let cuda_stream = device.new_stream()?;
    let mut random_generator = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(42);
    let [mut training_character_index_pairs, validation_character_index_pairs, test_character_index_pairs] =
        build_training_validation_test_datasets(&mut random_generator, !benchmark_repeats);
    let unique_evaluation_pairs = if benchmark_repeats {
        let mut split_generator = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(42);
        let [unique_training_pairs, unique_validation_pairs, unique_test_pairs] =
            build_training_validation_test_datasets(&mut split_generator, true);
        if unique_train {
            training_character_index_pairs = unique_training_pairs;
        }
        Some([unique_validation_pairs, unique_test_pairs])
    } else {
        None
    };
    println!("Дедупликация: {}; model_seed={model_seed:?}", if benchmark_repeats { "режим бенчмарка" } else { "train/dev/test" });
    if let Some(seed) = model_seed {
        random_generator = <rand::rngs::StdRng as rand::SeedableRng>::seed_from_u64(seed);
    }
    println!(
        "cuTile: batch={batch_size}, steps={training_steps}, embedding=8, first_layer_size={FIRST_LAYER_SIZE}; train/dev/test={}/{}/{}",
        training_character_index_pairs.example_count(),
        validation_character_index_pairs.example_count(),
        test_character_index_pairs.example_count()
    );
    let mut model = Model::new(&mut random_generator, &cuda_stream)?;
    let initial_validation_loss = evaluate_mean_negative_log_likelihood(&model, &validation_character_index_pairs, &cuda_stream)?;
    println!("Начальный dev loss={initial_validation_loss:.6}");
    let device_training_character_index_pairs = upload_tensor_to_gpu(
        training_character_index_pairs.flattened_character_index_pairs.clone(),
        &[training_character_index_pairs.example_count(), 4],
        &cuda_stream,
    )?;
    let mut buffers = ForwardAndBackwardBuffers::new(batch_size, &cuda_stream)?;
    let mut model_gradients = Model::zeroed_parameter_tensors(&cuda_stream)?;
    let mut training_step = cutile::api::zeros::<i32>(&[1]).sync_on(&cuda_stream)?;
    let training_graph = build_training_graph(
        &mut model,
        &mut buffers,
        &mut model_gradients,
        &device_training_character_index_pairs,
        &mut training_step,
        &TrainingConfiguration { batch_size, training_steps },
        &cuda_stream,
    )?;
    println!("JIT и подготовка: {:.3}s", program_started_at.elapsed().as_secs_f64());
    let training_started_at = std::time::Instant::now();
    for graph_replay_index in 0..training_steps / 10 {
        training_graph.launch().sync_on(&cuda_stream)?;
        if (graph_replay_index + 1) % 100 == 0 || graph_replay_index + 1 == training_steps / 10 {
            let batch_negative_log_likelihood_losses = buffers.negative_log_likelihood_losses.dup().to_host_vec().sync_on(&cuda_stream)?;
            let batch_mean_negative_log_likelihood_loss = batch_negative_log_likelihood_losses.iter().sum::<f32>() / batch_size as f32;
            if !batch_mean_negative_log_likelihood_loss.is_finite() {
                return Err("нечисловой loss".into());
            }
            println!(
                "step={} loss={batch_mean_negative_log_likelihood_loss:.6} elapsed={:.3}s",
                (graph_replay_index + 1) * 10,
                training_started_at.elapsed().as_secs_f64()
            );
        }
    }
    let training_elapsed_seconds = training_started_at.elapsed().as_secs_f64();
    println!(
        "Обучение: {training_elapsed_seconds:.3}s; {:.1} steps/s",
        training_steps as f64 / training_elapsed_seconds
    );
    for (dataset_name, character_index_pairs) in [
        ("train", &training_character_index_pairs),
        ("dev", &validation_character_index_pairs),
        ("test", &test_character_index_pairs),

    ] {
        println!(
            "{dataset_name}_loss={:.6}",
            evaluate_mean_negative_log_likelihood(&model, character_index_pairs, &cuda_stream)?
        );
    }
    if let Some([validation, test]) = &unique_evaluation_pairs {
        for (name, pairs) in [("dev_unique", validation), ("test_unique", test)] {
            println!("{name}_loss={:.6}", evaluate_mean_negative_log_likelihood(&model, pairs, &cuda_stream)?);
        }
    }
    generate_names(&model, &mut random_generator, &cuda_stream)?;
    println!("Общее время: {:.3}s", program_started_at.elapsed().as_secs_f64());
    Ok(())
}
