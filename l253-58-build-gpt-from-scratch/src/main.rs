//! Rust-перенос урока Karpathy ng-video-lecture. Путь: данные -> GPT -> генерация.
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
    // Все настройки меняй здесь, затем запускай cargo run --release.
    // Каждый тип проверяет своё значение через TryFrom; ноль шагов допустим.
    let settings: Settings = Settings {
        block_size: BlockSize::try_from(32)?,                         // T: максимум символов в контексте.
        embedding_dimension: EmbeddingDimension::try_from(64)?,       // C: ширина вектора; должна делиться на число голов.
        attention_head_count: AttentionHeadCount::try_from(4)?,       // H: число голов внимания.
        transformer_block_count: TransformerBlockCount::try_from(2)?, // L: число Transformer-блоков.
        dropout_probability: DropoutProbability::try_from(0.2)?,      // Доля обнуляемых значений при обучении.
        batch_size: BatchSize::try_from(8)?,                          // B: сколько окон текста в одном batch.
        training_step_count: TrainingStepCount::from(500),            // Число обновлений весов; 0 — без обучения.
        evaluation_interval: EvaluationInterval::try_from(100)?,      // Через сколько обновлений измерять loss; >0.
        evaluation_batch_count: EvaluationBatchCount::try_from(10)?,  // Число batch для оценки каждого split; >0.
        optimizer_settings: AdamWSettings {
            learning_rate: LearningRate::try_from(0.001)?,                                 // Размер шага AdamW.
            weight_decay: WeightDecay::try_from(0.01)?,                                    // Уменьшение весов.
            gradient_average_decay: GradientAverageDecay::try_from(0.9)?,                  // Коэффициент среднего градиента.
            squared_gradient_average_decay: SquaredGradientAverageDecay::try_from(0.999)?, // Коэффициент среднего квадрата градиента.
            numerical_stability_epsilon: NumericalStabilityEpsilon::try_from(1e-8)?,       // Добавка в знаменатель AdamW.
        },
        random_seed: 1337,
        prompt_text: PromptText::try_from("\n")?,              // Какой текст продолжать, например "ROMEO:".
        generated_token_count: GeneratedTokenCount::from(300), // Сколько новых символов написать.
        demonstrate_only: false,                               // true — только данные и пример внимания.
    };
    settings.validate()?;
    let vocabulary: data::CharacterVocabulary = data::CharacterVocabulary::new(data::SHAKESPEARE_TEXT);
    let corpus_token_ids: Vec<u32> = vocabulary.encode(data::SHAKESPEARE_TEXT)?;
    let (training_token_ids, validation_token_ids): (&[u32], &[u32]) = data::split_training_and_validation(&corpus_token_ids);
    println!(
        "Tiny Shakespeare: {} символов, V={}, train={}, val={}",
        corpus_token_ids.len(),
        vocabulary.characters_by_token_id.len(),
        training_token_ids.len(),
        validation_token_ids.len()
    );
    println!("Словарь: {:?}", vocabulary.characters_by_token_id.iter().collect::<String>());
    let example_text: &str = "First Citizen:\n";
    let example_token_ids: Vec<u32> = vocabulary.encode(example_text)?;
    println!("{example_text:?} -> {example_token_ids:?} -> {:?}", vocabulary.decode(&example_token_ids)?);
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
        train::run(&settings, &vocabulary, &corpus_token_ids)?;
    }
    Ok(())
}
