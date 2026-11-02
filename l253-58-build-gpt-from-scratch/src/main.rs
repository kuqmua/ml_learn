//! Урок Karpathy на Rust: CharacterVocabulary -> GptLanguageModel -> generate_token_ids.
mod data;
mod model;
mod random;
mod settings;
mod train;
use settings::{
    AdamWSettings, AttentionHeadCount, BatchSize, BlockSize, DropoutProbability, EmbeddingDimension, EvaluationBatchCount, EvaluationInterval,
    GeneratedTokenCount, GradientAverageDecay, LearningRate, NumericalStabilityEpsilon, PromptText, Settings, SquaredGradientAverageDecay, TrainingStepCount,
    TransformerBlockCount, WeightDecay,
};

fn main() -> candle_core::Result<()> {
    // Меняй значения Settings здесь и запускай cargo run --release без аргументов.
    // TryFrom проверяет ограничения отдельного значения; validate ниже — совместимость полей.
    // TrainingStepCount и GeneratedTokenCount допускают ноль и поэтому используют From.
    let settings: Settings = Settings {
        block_size: BlockSize::try_from(32)?, // Максимальная длина окна; при генерации sequence_length может быть меньше.
        embedding_dimension: EmbeddingDimension::try_from(64)?, // Число координат вектора позиции; кратно attention_head_count, C в формулах.
        attention_head_count: AttentionHeadCount::try_from(4)?, // Число независимых AttentionHead, между которыми делятся координаты вектора.
        transformer_block_count: TransformerBlockCount::try_from(2)?, // Сколько TransformerBlock последовательно обрабатывают представление текста.
        dropout_probability: DropoutProbability::try_from(0.2)?, // Вероятность случайного обнуления при is_training=true; диапазон [0,1).
        batch_size: BatchSize::try_from(8)?,  // Число окон длины block_size в одном обучающем batch; B в формулах.
        training_step_count: TrainingStepCount::from(5000), // Число обновлений весов AdamW; 0 оставляет случайные начальные веса.
        evaluation_interval: EvaluationInterval::try_from(100)?, // Интервал между оценками training_loss и validation_loss, в обновлениях весов.
        evaluation_batch_count: EvaluationBatchCount::try_from(10)?, // По скольким batch усреднять ошибку каждой выборки при одной оценке.
        adamw_settings: AdamWSettings {
            learning_rate: LearningRate::try_from(0.001)?, // Масштаб изменения весов после адаптивного нормирования градиента.
            weight_decay: WeightDecay::try_from(0.01)?,    // За шаг веса умножаются на 1 - learning_rate * weight_decay.
            gradient_average_decay: GradientAverageDecay::try_from(0.9)?, // Вес накопленной истории градиентов; новому градиенту достаётся 1-beta1.
            squared_gradient_average_decay: SquaredGradientAverageDecay::try_from(0.999)?, // Вес истории квадратов градиентов; новому квадрату достаётся 1-beta2.
            numerical_stability_epsilon: NumericalStabilityEpsilon::try_from(1e-8)?, // Положительная добавка в знаменатель AdamW для защиты от деления на ноль.
        },
        random_seed: 1337,                                     // Одинаковое начальное число воспроизводит выборки и инициализацию весов.
        prompt_text: PromptText::try_from("\n")?,              // Текст для продолжения; "\n" — перевод строки с ID 0, можно задать "ROMEO:".
        generated_token_count: GeneratedTokenCount::from(300), // Сколько новых символов дописать после prompt_text, не считая сам prompt_text.
        demonstrate_only: false,                               // true пропускает обучение и генерацию, оставляя демонстрации данных и внимания.
    };
    settings.validate()?;
    let character_vocabulary: data::CharacterVocabulary = data::CharacterVocabulary::new(data::SHAKESPEARE_TEXT);
    let corpus_token_ids: Vec<u32> = character_vocabulary.encode(data::SHAKESPEARE_TEXT)?;
    let (training_token_ids, validation_token_ids): (&[u32], &[u32]) = data::split_training_and_validation(&corpus_token_ids);
    println!(
        "Tiny Shakespeare: {} символов, V={}, train={}, val={}",
        corpus_token_ids.len(),
        character_vocabulary.characters_by_token_id.len(),
        training_token_ids.len(),
        validation_token_ids.len()
    );
    println!("Словарь: {:?}", character_vocabulary.characters_by_token_id.iter().collect::<String>());
    let example_text: &str = "First Citizen:\n";
    let example_token_ids: Vec<u32> = character_vocabulary.encode(example_text)?;
    println!(
        "{example_text:?} -> {example_token_ids:?} -> {:?}",
        character_vocabulary.decode(&example_token_ids)?
    );
    let (input_token_tensor, target_token_tensor): (candle_core::Tensor, candle_core::Tensor) =
        data::sample_text_batch(training_token_ids, 1, 8, &mut random::RandomNumberGenerator::new(settings.random_seed()))?;
    println!(
        "Пример окна: x={:?}, y={:?}",
        input_token_tensor.to_vec2::<u32>()?,
        target_token_tensor.to_vec2::<u32>()?
    );
    println!("Для позиции t модель получает x[0..=t] и должна предсказать y[t] = следующий символ.");
    model::demonstrate_causal_averaging()?;
    if !settings.demonstrate_only() {
        train::run(&settings, &character_vocabulary, &corpus_token_ids)?;
    }
    Ok(())
}
