//! Rust-перенос урока Karpathy ng-video-lecture. Путь: данные -> GPT -> генерация.
mod data;
mod model;
mod rng;
mod settings;
mod train;
use settings::{
    AdamBeta1, AdamBeta2, AdamEpsilon, AdamWSettings, BatchSize, BlockSize, Channels, Dropout, EvaluationBatches, EvaluationInterval, GeneratedTokens,
    HeadCount, LayerCount, LearningRate, Prompt, Settings, TrainingSteps, WeightDecay,
};

fn main() -> candle_core::Result<()> {
    // Все настройки меняй здесь, затем запускай cargo run --release.
    // Каждый тип проверяет своё значение через TryFrom; ноль шагов допустим.
    let settings: Settings = Settings {
        block: BlockSize::try_from(32)?,                // T: максимум символов в контексте.
        channels: Channels::try_from(64)?,              // C: ширина вектора; должна делиться на heads.
        heads: HeadCount::try_from(4)?,                 // H: число голов внимания.
        layers: LayerCount::try_from(2)?,               // L: число Transformer-блоков.
        dropout: Dropout::try_from(0.2)?,               // Доля обнуляемых значений при обучении.
        batch: BatchSize::try_from(8)?,                 // B: сколько окон текста в одном batch.
        steps: TrainingSteps::from(500),                // Число обновлений весов; 0 — без обучения.
        eval_every: EvaluationInterval::try_from(100)?, // Через сколько обновлений измерять loss; >0.
        eval_batches: EvaluationBatches::try_from(10)?, // Число batch для оценки каждого split; >0.
        optimizer: AdamWSettings {
            lr: LearningRate::try_from(0.001)?,         // Размер шага AdamW.
            weight_decay: WeightDecay::try_from(0.01)?, // Уменьшение весов.
            beta1: AdamBeta1::try_from(0.9)?,           // Коэффициент среднего градиента.
            beta2: AdamBeta2::try_from(0.999)?,         // Коэффициент среднего квадрата градиента.
            eps: AdamEpsilon::try_from(1e-8)?,          // Добавка в знаменатель AdamW.
        },
        seed: 1337,
        prompt: Prompt::try_from("\n")?,      // Какой текст продолжать, например "ROMEO:".
        generate: GeneratedTokens::from(300), // Сколько новых символов написать.
        demo_only: false,                     // true — только данные и пример внимания.
    };
    settings.validate()?;
    let vocab: data::Vocabulary = data::Vocabulary::new(data::TEXT);
    let ids: Vec<u32> = vocab.encode(data::TEXT)?;
    let (train_ids, val_ids): (&[u32], &[u32]) = data::split(&ids);
    println!(
        "Tiny Shakespeare: {} символов, V={}, train={}, val={}",
        ids.len(),
        vocab.chars.len(),
        train_ids.len(),
        val_ids.len()
    );
    println!("Словарь: {:?}", vocab.chars.iter().collect::<String>());
    let example: &str = "First Citizen:\n";
    let encoded: Vec<u32> = vocab.encode(example)?;
    println!("{example:?} -> {encoded:?} -> {:?}", vocab.decode(&encoded)?);
    let (x, y): (candle_core::Tensor, candle_core::Tensor) = data::batch(train_ids, 1, 8, &mut rng::Rng::new(settings.seed()))?;
    println!("Пример окна: x={:?}, y={:?}", x.to_vec2::<u32>()?, y.to_vec2::<u32>()?);
    println!("Для позиции t модель получает x[0..=t] и должна предсказать y[t] = следующий символ.");
    model::demonstrate_attention()?;
    if !settings.demo_only() {
        train::run(&settings, &vocab, &ids)?;
    }
    Ok(())
}
