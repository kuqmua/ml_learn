//! Rust-перенос урока Karpathy ng-video-lecture. Путь: данные -> bigram -> GPT -> генерация.
mod data;
mod model;
mod rng;
mod train;

fn main() -> candle_core::Result<()> {
    let Some(options) = train::Options::parse(std::env::args().skip(1))? else {
        return Ok(());
    };
    let vocab = data::Vocabulary::new(data::TEXT);
    let ids = vocab.encode(data::TEXT)?;
    let (train_ids, val_ids) = data::split(&ids);
    println!(
        "Tiny Shakespeare: {} символов, V={}, train={}, val={}",
        ids.len(),
        vocab.chars.len(),
        train_ids.len(),
        val_ids.len()
    );
    println!("Словарь: {:?}", vocab.chars.iter().collect::<String>());
    let example = "First Citizen:\n";
    let encoded = vocab.encode(example)?;
    println!("{example:?} -> {encoded:?} -> {:?}", vocab.decode(&encoded)?);
    let settings = options.settings()?;
    let (x, y) = data::batch(train_ids, 1, 8, &mut rng::Rng::new(options.seed))?;
    println!("Пример окна: x={:?}, y={:?}", x.to_vec2::<u32>()?, y.to_vec2::<u32>()?);
    println!("Для позиции t модель получает x[0..=t] и должна предсказать y[t] = следующий символ.");
    model::demonstrate_attention()?;
    if !options.demo_only {
        train::run(&options, &settings, &vocab, &ids)?;
    }
    Ok(())
}
