use crate::{
    data,
    model::{Config, LanguageModel, Parameters, loss},
    rng::Rng,
};
use candle_core::{Device, Result, Tensor};
use candle_nn::{AdamW, Optimizer, ParamsAdamW};

pub(crate) struct Options {
    pub(crate) preset: String,
    pub(crate) model: String,
    pub(crate) steps: Option<usize>,
    pub(crate) eval_every: Option<usize>,
    pub(crate) eval_batches: Option<usize>,
    pub(crate) generate: usize,
    pub(crate) seed: u64,
    pub(crate) prompt: Option<String>,
    pub(crate) demo_only: bool,
}

pub(crate) struct Settings {
    pub(crate) config: Config,
    pub(crate) batch: usize,
    pub(crate) steps: usize,
    pub(crate) eval_every: usize,
    pub(crate) eval_batches: usize,
    pub(crate) learning_rate: f64,
}

impl Options {
    pub(crate) fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Self>> {
        let mut result = Self {
            preset: "tiny".into(),
            model: "gpt".into(),
            steps: None,
            eval_every: None,
            eval_batches: None,
            generate: 300,
            seed: 1337,
            prompt: None,
            demo_only: false,
        };
        let mut args = args.into_iter();
        while let Some(arg) = args.next() {
            if arg == "--help" || arg == "-h" {
                println!(
                    "Урок 253: bigram -> GPT на Tiny Shakespeare\n\
--model gpt|bigram       Модель (gpt)\n\
--preset smoke|tiny|lecture  Размеры и параметры (tiny, CPU)\n\
--steps N                Число обновлений AdamW, допустим 0\n\
--eval-every N           Интервал оценки loss, N>0\n\
--eval-batches N         Число batch для оценки каждого split, N>0\n\
--generate N             Число новых символов (300)\n\
--prompt TEXT            Начальный текст (по умолчанию ID 0 = newline)\n\
--seed N                 Seed (1337)\n\
--demo-only              Только данные и пример матрицы внимания"
                );
                return Ok(None);
            }
            if arg == "--demo-only" {
                result.demo_only = true;
                continue;
            }
            let value = args
                .next()
                .ok_or_else(|| candle_core::Error::Msg(format!("После {arg} нужно значение; см. --help")))?;
            let number = || {
                value
                    .parse::<usize>()
                    .map_err(|_| candle_core::Error::Msg(format!("После {arg} нужно целое неотрицательное число")))
            };
            match arg.as_str() {
                "--model" => result.model = value,
                "--preset" => result.preset = value,
                "--steps" => result.steps = Some(number()?),
                "--eval-every" => result.eval_every = Some(number()?),
                "--eval-batches" => result.eval_batches = Some(number()?),
                "--generate" => result.generate = number()?,
                "--seed" => result.seed = value.parse().map_err(|_| candle_core::Error::Msg("seed должен быть u64".into()))?,
                "--prompt" => result.prompt = Some(value),
                _ => candle_core::bail!("Неизвестный аргумент {arg}; см. --help"),
            }
        }
        result.settings()?;
        Ok(Some(result))
    }

    pub(crate) fn settings(&self) -> Result<Settings> {
        if self.model != "gpt" && self.model != "bigram" {
            candle_core::bail!("model должен быть gpt или bigram");
        }
        let mut s = match self.preset.as_str() {
            "smoke" => Settings {
                config: Config {
                    block: 8,
                    channels: 16,
                    heads: 2,
                    layers: 1,
                    dropout: 0.2,
                },
                batch: 2,
                steps: 20,
                eval_every: 10,
                eval_batches: 2,
                learning_rate: 0.003,
            },
            "tiny" => Settings {
                config: Config {
                    block: 32,
                    channels: 64,
                    heads: 4,
                    layers: 2,
                    dropout: 0.2,
                },
                batch: 8,
                steps: 500,
                eval_every: 100,
                eval_batches: 10,
                learning_rate: 0.001,
            },
            "lecture" => Settings {
                config: Config {
                    block: 256,
                    channels: 384,
                    heads: 6,
                    layers: 6,
                    dropout: 0.2,
                },
                batch: 64,
                steps: 5000,
                eval_every: 500,
                eval_batches: 200,
                learning_rate: 0.0003,
            },
            _ => candle_core::bail!("preset должен быть smoke, tiny или lecture"),
        };
        if self.model == "bigram" {
            s.learning_rate = 0.01;
            if self.preset != "smoke" {
                s.config.block = 8;
                s.batch = 32;
                s.steps = if self.preset == "lecture" { 3000 } else { 1000 };
                s.eval_every = if self.preset == "lecture" { 300 } else { 200 };
            }
        }
        if let Some(n) = self.steps {
            s.steps = n;
        }
        if let Some(n) = self.eval_every {
            s.eval_every = n;
        }
        if let Some(n) = self.eval_batches {
            s.eval_batches = n;
        }
        if s.eval_every == 0 || s.eval_batches == 0 {
            candle_core::bail!("eval-every и eval-batches должны быть >0");
        }
        s.config.validate()?;
        Ok(s)
    }
}

// Одинаковые окна при каждой оценке: seed оценки не зависит от шага и RNG обучения.
// Это monitoring validation, а не независимый окончательный test.
pub(crate) fn estimate(model: &LanguageModel, split: &[u32], s: &Settings, seed: u64) -> Result<f32> {
    let mut rng = Rng::new(seed);
    let mut sum = 0f32;
    for _ in 0..s.eval_batches {
        let (x, y) = data::batch(split, s.batch, s.config.block, &mut rng)?;
        let logits = model.forward(&x, false, &mut rng)?;
        sum += loss(&logits, &y)?.to_scalar::<f32>()?;
    }
    Ok(sum / s.eval_batches as f32)
}

// Temperature=1, categorical sampling (не argmax). f64 для устойчивой суммы вероятностей.
fn sample(logits: &[f32], rng: &mut Rng) -> Result<u32> {
    if logits.is_empty() || logits.iter().any(|x| !x.is_finite()) {
        candle_core::bail!("Некорректные logits при sampling");
    }
    let max = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max) as f64;
    let weights: Vec<f64> = logits.iter().map(|&x| (x as f64 - max).exp()).collect();
    let threshold = rng.uniform() * weights.iter().sum::<f64>();
    let mut cumulative = 0.0;
    for (id, p) in weights.iter().enumerate() {
        cumulative += p;
        if threshold < cumulative {
            return Ok(id as u32);
        }
    }
    Ok((weights.len() - 1) as u32)
}

pub(crate) fn generate(model: &LanguageModel, prompt: &[u32], new_tokens: usize, seed: u64) -> Result<Vec<u32>> {
    if prompt.is_empty() {
        candle_core::bail!("Prompt не может быть пустым");
    }
    let mut ids = prompt.to_vec();
    let mut rng = Rng::new(seed);
    for _ in 0..new_tokens {
        // Позиции снова 0..T; это такое же обрезание окна, как в gpt.py, без KV-cache.
        let context = &ids[ids.len().saturating_sub(model.context())..];
        let x = Tensor::from_slice(context, (1, context.len()), &Device::Cpu)?;
        let logits = model.forward(&x, false, &mut rng)?;
        let last = logits.narrow(1, context.len() - 1, 1)?.flatten_all()?.to_vec1::<f32>()?;
        ids.push(sample(&last, &mut rng)?);
    }
    Ok(ids)
}

pub(crate) fn run(options: &Options, settings: &Settings, vocab: &data::Vocabulary, ids: &[u32]) -> Result<()> {
    let started = std::time::Instant::now();
    let (train, val) = data::split(ids);
    let prompt = match &options.prompt {
        Some(s) => vocab.encode(s)?,
        None => vec![0],
    };
    if prompt.is_empty() {
        candle_core::bail!("Prompt не может быть пустым");
    }
    let mut params = Parameters::new(options.seed);
    let model = LanguageModel::new(&options.model, &mut params, vocab.chars.len(), &settings.config)?;
    println!("\nmodel={} preset={} seed={} device=CPU dtype=f32", options.model, options.preset, options.seed);
    println!(
        "B={} T={} steps={} lr={} weight_decay=0.01",
        settings.batch, settings.config.block, settings.steps, settings.learning_rate
    );
    if options.model == "gpt" {
        println!(
            "C={} H={} L={} dropout={}",
            settings.config.channels, settings.config.heads, settings.config.layers, settings.config.dropout
        );
    }
    println!("Параметров: {}; uniform loss ln(V)={:.4}", params.count(), (vocab.chars.len() as f32).ln());
    // AdamW как в PyTorch по умолчанию: beta1=.9, beta2=.999, eps=1e-8, wd=.01.
    let mut optimizer = AdamW::new(
        params.variables(),
        ParamsAdamW {
            lr: settings.learning_rate,
            ..Default::default()
        },
    )?;
    let mut batch_rng = Rng::new(options.seed.wrapping_add(1));
    let mut dropout_rng = Rng::new(options.seed.wrapping_add(2));
    for step in 0..=settings.steps {
        if step % settings.eval_every == 0 || step == settings.steps {
            let train_loss = estimate(&model, train, settings, options.seed.wrapping_add(3))?;
            let val_loss = estimate(&model, val, settings, options.seed.wrapping_add(4))?;
            if !train_loss.is_finite() || !val_loss.is_finite() {
                candle_core::bail!("Loss стал нечисловым");
            }
            println!(
                "step {step:5}: train {train_loss:.4}, val {val_loss:.4}, elapsed {:.2}s",
                started.elapsed().as_secs_f64()
            );
        }
        if step == settings.steps {
            break;
        }
        let (x, y) = data::batch(train, settings.batch, settings.config.block, &mut batch_rng)?;
        let logits = model.forward(&x, true, &mut dropout_rng)?;
        let objective = loss(&logits, &y)?;
        // Новый forward создаёт новый граф. backward возвращает свежий GradStore,
        // поэтому PyTorch zero_grad здесь не требуется.
        optimizer.backward_step(&objective)?;
    }
    let generated = generate(&model, &prompt, options.generate, options.seed.wrapping_add(5))?;
    println!(
        "\n--- Генерация: prompt + {} новых символов ---\n{}\n--- конец ---",
        options.generate,
        vocab.decode(&generated)?
    );
    println!("Всего {:.2}s", started.elapsed().as_secs_f64());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cli_rejects_invalid_settings_and_preserves_overrides() -> Result<()> {
        for args in [
            vec!["--preset", "wrong"],
            vec!["--eval-batches", "0"],
            vec!["--steps", "-1"],
            vec!["--model", "llama"],
            vec!["--steps"],
        ] {
            assert!(Options::parse(args.into_iter().map(str::to_string)).is_err());
        }
        let o = Options::parse(["--steps", "3", "--preset", "smoke"].map(str::to_string))?.unwrap();
        assert_eq!(o.settings()?.steps, 3);
        Ok(())
    }
    #[test]
    fn generation_crops_context_and_is_seeded() -> Result<()> {
        let cfg = Config {
            block: 3,
            channels: 8,
            heads: 2,
            layers: 1,
            dropout: 0.5,
        };
        let mut p = Parameters::new(1337);
        let m = LanguageModel::new("gpt", &mut p, 4, &cfg)?;
        let prompt = [0, 1, 2, 3, 2, 1];
        let a = generate(&m, &prompt, 20, 9)?;
        assert_eq!(&a[..prompt.len()], &prompt);
        assert_eq!(a.len(), 26);
        assert!(a.iter().all(|&id| id < 4));
        assert_eq!(a, generate(&m, &prompt, 20, 9)?);
        assert!(generate(&m, &[], 2, 0).is_err());
        Ok(())
    }
    #[test]
    fn trains_full_gpt_on_small_repeating_corpus() -> Result<()> {
        let cfg = Config {
            block: 4,
            channels: 8,
            heads: 2,
            layers: 1,
            dropout: 0.0,
        };
        let mut p = Parameters::new(1337);
        let model = LanguageModel::new("gpt", &mut p, 4, &cfg)?;
        let mut opt = AdamW::new(
            p.variables(),
            ParamsAdamW {
                lr: 0.02,
                weight_decay: 0.0,
                ..Default::default()
            },
        )?;
        let x = Tensor::new(&[[0u32, 1, 2, 3], [0, 1, 2, 3]], &Device::Cpu)?;
        let y = Tensor::new(&[[1u32, 2, 3, 0], [1, 2, 3, 0]], &Device::Cpu)?;
        let mut rng = Rng::new(1);
        let before = loss(&model.forward(&x, false, &mut rng)?, &y)?.to_scalar::<f32>()?;
        for _ in 0..60 {
            let objective = loss(&model.forward(&x, true, &mut rng)?, &y)?;
            opt.backward_step(&objective)?;
        }
        let after = loss(&model.forward(&x, false, &mut rng)?, &y)?.to_scalar::<f32>()?;
        assert!(after < 0.15 && after < before * 0.2, "before={before}, after={after}");
        Ok(())
    }
}
