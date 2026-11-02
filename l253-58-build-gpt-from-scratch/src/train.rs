use crate::{
    data,
    model::{Gpt, Parameters, loss},
    rng::Rng,
    settings::Settings,
};
use candle_core::{Device, Result, Tensor};
#[cfg(test)]
use candle_nn::ParamsAdamW;
use candle_nn::{AdamW, Optimizer};

// Одинаковые окна при каждой оценке: seed оценки не зависит от шага и RNG обучения.
// Это monitoring validation, а не независимый окончательный test.
pub(crate) fn estimate(model: &Gpt, split: &[u32], s: &Settings, seed: u64) -> Result<f32> {
    let mut rng: Rng = Rng::new(seed);
    let mut sum: f32 = 0f32;
    for _ in 0..s.eval_batches() {
        let (x, y): (Tensor, Tensor) = data::batch(split, s.batch(), s.block(), &mut rng)?;
        let logits: Tensor = model.forward(&x, false, &mut rng)?;
        sum += loss(&logits, &y)?.to_scalar::<f32>()?;
    }
    Ok(sum / s.eval_batches() as f32)
}

// Temperature=1, categorical sampling (не argmax). f64 для устойчивой суммы вероятностей.
fn sample(logits: &[f32], rng: &mut Rng) -> Result<u32> {
    if logits.is_empty() || logits.iter().any(|x: &f32| -> bool { !x.is_finite() }) {
        candle_core::bail!("Некорректные logits при sampling");
    }
    let max: f64 = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max) as f64;
    let weights: Vec<f64> = logits.iter().map(|&x: &f32| -> f64 { (x as f64 - max).exp() }).collect();
    let threshold: f64 = rng.uniform() * weights.iter().sum::<f64>();
    let mut cumulative: f64 = 0.0;
    let id: usize = weights
        .iter()
        .position(|p: &f64| -> bool {
            cumulative += p;
            threshold < cumulative
        })
        .unwrap_or(weights.len() - 1);
    Ok(id as u32)
}

pub(crate) fn generate(model: &Gpt, prompt: &[u32], new_tokens: usize, seed: u64) -> Result<Vec<u32>> {
    if prompt.is_empty() {
        candle_core::bail!("Prompt не может быть пустым");
    }
    let mut ids: Vec<u32> = prompt.to_vec();
    let mut rng: Rng = Rng::new(seed);
    for _ in 0..new_tokens {
        // Позиции снова 0..T; это такое же обрезание окна, как в gpt.py, без KV-cache.
        let context: &[u32] = &ids[ids.len().saturating_sub(model.context())..];
        let x: Tensor = Tensor::from_slice(context, (1, context.len()), &Device::Cpu)?;
        let logits: Tensor = model.forward(&x, false, &mut rng)?;
        let last: Vec<f32> = logits.narrow(1, context.len() - 1, 1)?.flatten_all()?.to_vec1::<f32>()?;
        ids.push(sample(&last, &mut rng)?);
    }
    Ok(ids)
}

pub(crate) fn run(settings: &Settings, vocab: &data::Vocabulary, ids: &[u32]) -> Result<()> {
    settings.validate()?;
    let started: std::time::Instant = std::time::Instant::now();
    let (train, val): (&[u32], &[u32]) = data::split(ids);
    let prompt: Vec<u32> = vocab.encode(settings.prompt())?;
    if prompt.is_empty() {
        candle_core::bail!("Prompt не может быть пустым");
    }
    let mut params: Parameters = Parameters::new(settings.seed());
    let model: Gpt = Gpt::new(&mut params, vocab.chars.len(), settings)?;
    println!("\nGPT seed={} device=CPU dtype=f32", settings.seed());
    println!(
        "B={} T={} steps={} lr={} weight_decay={}",
        settings.batch(),
        settings.block(),
        settings.steps(),
        settings.optimizer().lr,
        settings.optimizer().weight_decay
    );
    println!(
        "C={} H={} L={} dropout={}",
        settings.channels(),
        settings.heads(),
        settings.layers(),
        settings.dropout()
    );
    println!("Параметров: {}; uniform loss ln(V)={:.4}", params.count(), (vocab.chars.len() as f32).ln());
    // Параметры AdamW заданы константами в начале main.rs.
    let mut optimizer: AdamW = AdamW::new(params.variables(), settings.optimizer())?;
    let mut batch_rng: Rng = Rng::new(settings.seed().wrapping_add(1));
    let mut dropout_rng: Rng = Rng::new(settings.seed().wrapping_add(2));
    (0usize..=settings.steps()).try_for_each(|step: usize| -> Result<()> {
        if step.is_multiple_of(settings.eval_every()) || step == settings.steps() {
            let train_loss: f32 = estimate(&model, train, settings, settings.seed().wrapping_add(3))?;
            let val_loss: f32 = estimate(&model, val, settings, settings.seed().wrapping_add(4))?;
            if !train_loss.is_finite() || !val_loss.is_finite() {
                candle_core::bail!("Loss стал нечисловым");
            }
            println!(
                "step {step:5}: train {train_loss:.4}, val {val_loss:.4}, elapsed {:.2}s",
                started.elapsed().as_secs_f64()
            );
        }
        if step < settings.steps() {
            let (x, y): (Tensor, Tensor) = data::batch(train, settings.batch(), settings.block(), &mut batch_rng)?;
            let logits: Tensor = model.forward(&x, true, &mut dropout_rng)?;
            let objective: Tensor = loss(&logits, &y)?;
            // Новый forward создаёт новый граф. backward возвращает свежий GradStore,
            // поэтому PyTorch zero_grad здесь не требуется.
            optimizer.backward_step(&objective)?;
        }
        Ok(())
    })?;
    let generated: Vec<u32> = generate(&model, &prompt, settings.generate(), settings.seed().wrapping_add(5))?;
    println!(
        "\n--- Генерация: prompt + {} новых символов ---\n{}\n--- конец ---",
        settings.generate(),
        vocab.decode(&generated)?
    );
    println!("Всего {:.2}s", started.elapsed().as_secs_f64());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{BlockSize, Channels, Dropout, HeadCount, LayerCount, test_settings};
    #[test]
    fn generation_crops_context_and_is_seeded() -> Result<()> {
        let settings: Settings = Settings {
            block: BlockSize::try_from(3)?,
            channels: Channels::try_from(8)?,
            heads: HeadCount::try_from(2)?,
            layers: LayerCount::try_from(1)?,
            dropout: Dropout::try_from(0.5)?,
            ..test_settings()?
        };
        let mut p: Parameters = Parameters::new(1337);
        let m: Gpt = Gpt::new(&mut p, 4, &settings)?;
        let prompt: [u32; 6] = [0, 1, 2, 3, 2, 1];
        let a: Vec<u32> = generate(&m, &prompt, 20, 9)?;
        assert_eq!(&a[..prompt.len()], &prompt);
        assert_eq!(a.len(), 26);
        assert!(a.iter().all(|&id: &u32| -> bool { id < 4 }));
        assert_eq!(a, generate(&m, &prompt, 20, 9)?);
        assert!(generate(&m, &[], 2, 0).is_err());
        Ok(())
    }
    #[test]
    fn trains_full_gpt_on_small_repeating_corpus() -> Result<()> {
        let settings: Settings = Settings {
            block: BlockSize::try_from(4)?,
            channels: Channels::try_from(8)?,
            heads: HeadCount::try_from(2)?,
            layers: LayerCount::try_from(1)?,
            dropout: Dropout::try_from(0.0)?,
            ..test_settings()?
        };
        let mut p: Parameters = Parameters::new(1337);
        let model: Gpt = Gpt::new(&mut p, 4, &settings)?;
        let mut opt: AdamW = AdamW::new(
            p.variables(),
            ParamsAdamW {
                lr: 0.02,
                weight_decay: 0.0,
                ..Default::default()
            },
        )?;
        let x: Tensor = Tensor::new(&[[0u32, 1, 2, 3], [0, 1, 2, 3]], &Device::Cpu)?;
        let y: Tensor = Tensor::new(&[[1u32, 2, 3, 0], [1, 2, 3, 0]], &Device::Cpu)?;
        let mut rng: Rng = Rng::new(1);
        let before: f32 = loss(&model.forward(&x, false, &mut rng)?, &y)?.to_scalar::<f32>()?;
        for _ in 0..60 {
            let objective: Tensor = loss(&model.forward(&x, true, &mut rng)?, &y)?;
            opt.backward_step(&objective)?;
        }
        let after: f32 = loss(&model.forward(&x, false, &mut rng)?, &y)?.to_scalar::<f32>()?;
        assert!(after < 0.15 && after < before * 0.2, "before={before}, after={after}");
        Ok(())
    }
}
