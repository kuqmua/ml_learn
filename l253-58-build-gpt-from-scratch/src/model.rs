//! Архитектура из ng-video-lecture: отдельные головы, pre-LN, ReLU и обучаемые позиции.
//! Candle предоставляет операции над тензорами и backward, но не готовую GPT-модель.
use crate::{rng::Rng, settings::Settings};
use candle_core::{D, DType, Device, Result, Tensor, Var};

pub(crate) struct Parameters {
    pub(crate) named: Vec<(String, Var)>,
    rng: Rng,
}
impl Parameters {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            named: vec![],
            rng: Rng::new(seed),
        }
    }
    fn add(&mut self, name: &str, shape: &[usize], constant: Option<f32>) -> Result<Tensor> {
        let values: Vec<f32> = (0..shape.iter().product::<usize>())
            .map(|_: usize| -> f32 { constant.unwrap_or_else(|| -> f32 { self.rng.normal() * 0.02 }) })
            .collect();
        let var: Var = Var::from_vec(values, shape, &Device::Cpu)?;
        let tensor: Tensor = var.as_tensor().clone();
        self.named.push((name.to_owned(), var));
        Ok(tensor)
    }
    pub(crate) fn variables(&self) -> Vec<Var> {
        self.named.iter().map(|(_, v): &(String, Var)| -> Var { v.clone() }).collect()
    }
    pub(crate) fn count(&self) -> usize {
        self.named.iter().map(|(_, v): &(String, Var)| -> usize { v.elem_count() }).sum()
    }
}

// В eval отсоединяем параметры от графа: ни dropout, ни накопление графа для backward.
fn parameter(x: &Tensor, train: bool) -> Tensor {
    if train { x.clone() } else { x.detach() }
}

struct Linear {
    weight: Tensor,
    bias: Option<Tensor>,
}
impl Linear {
    fn new(p: &mut Parameters, name: &str, input: usize, output: usize, bias: bool) -> Result<Self> {
        Ok(Self {
            weight: p.add(&format!("{name}.weight"), &[output, input], None)?,
            bias: if bias {
                Some(p.add(&format!("{name}.bias"), &[output], Some(0.0))?)
            } else {
                None
            },
        })
    }
    fn forward(&self, x: &Tensor, train: bool) -> Result<Tensor> {
        let input: usize = x.dim(D::Minus1)?;
        let output: usize = self.weight.dim(0)?;
        let mut shape: Vec<usize> = x.dims().to_vec();
        *shape.last_mut().unwrap() = output;
        let y: Tensor = x.reshape((x.elem_count() / input, input))?.matmul(&parameter(&self.weight, train).t()?)?;
        let y: Tensor = self
            .bias
            .as_ref()
            .map(|bias: &Tensor| -> Result<Tensor> { y.broadcast_add(&parameter(bias, train)) })
            .transpose()?
            .unwrap_or(y);
        y.reshape(shape)
    }
}

struct Embedding {
    weight: Tensor,
    width: usize,
}
impl Embedding {
    fn new(p: &mut Parameters, name: &str, rows: usize, width: usize) -> Result<Self> {
        Ok(Self {
            weight: p.add(&format!("{name}.weight"), &[rows, width], None)?,
            width,
        })
    }
    fn forward(&self, ids: &Tensor, train: bool) -> Result<Tensor> {
        let mut shape: Vec<usize> = ids.dims().to_vec();
        shape.push(self.width);
        parameter(&self.weight, train).index_select(&ids.flatten_all()?, 0)?.reshape(shape)
    }
}

// Нормируем C координат КАЖДОЙ позиции отдельно; не batch и не ось времени.
// Пишем через базовые операции, чтобы путь backward был виден и поддерживался.
struct LayerNorm {
    gain: Tensor,
    bias: Tensor,
}
impl LayerNorm {
    fn new(p: &mut Parameters, name: &str, c: usize) -> Result<Self> {
        Ok(Self {
            gain: p.add(&format!("{name}.weight"), &[c], Some(1.0))?,
            bias: p.add(&format!("{name}.bias"), &[c], Some(0.0))?,
        })
    }
    fn forward(&self, x: &Tensor, train: bool) -> Result<Tensor> {
        let centered: Tensor = x.broadcast_sub(&x.mean_keepdim(D::Minus1)?)?;
        let variance: Tensor = centered.sqr()?.mean_keepdim(D::Minus1)?;
        centered
            .broadcast_div(&(variance + 1e-5)?.sqrt()?)?
            .broadcast_mul(&parameter(&self.gain, train))?
            .broadcast_add(&parameter(&self.bias, train))
    }
}

// Inverted dropout: оставшиеся значения делим на 1-p; в eval это тождественная функция.
fn dropout(x: &Tensor, probability: f64, train: bool, rng: &mut Rng) -> Result<Tensor> {
    if !train || probability == 0.0 {
        return Ok(x.clone());
    }
    let mask: Vec<f32> = (0..x.elem_count())
        .map(|_: usize| -> f32 {
            if rng.uniform() < probability {
                0.0
            } else {
                (1.0 / (1.0 - probability)) as f32
            }
        })
        .collect();
    x * Tensor::from_vec(mask, x.shape(), x.device())?
}

pub(crate) fn causal_mask(t: usize) -> Result<Tensor> {
    let values: Vec<f32> = (0..t)
        .flat_map(|row: usize| {
            (0usize..t).map(move |col: usize| -> f32 {
                if col > row { f32::NEG_INFINITY } else { 0.0 }
            })
        })
        .collect();
    Tensor::from_vec(values, (t, t), &Device::Cpu)
}

// Q: что ищу; K: по каким признакам меня искать; V: что передам другим позициям.
fn attention_weights(q: &Tensor, k: &Tensor) -> Result<Tensor> {
    let (_, t, d): (usize, usize, usize) = q.dims3()?;
    let scores: Tensor = (q.matmul(&k.transpose(1, 2)?)? / (d as f64).sqrt())?;
    let masked: Tensor = scores.broadcast_add(&causal_mask(t)?.to_dtype(q.dtype())?)?;
    candle_nn::ops::softmax(&masked, D::Minus1)
}

struct Head {
    query: Linear,
    key: Linear,
    value: Linear,
}
impl Head {
    fn new(p: &mut Parameters, name: &str, c: usize, d: usize) -> Result<Self> {
        Ok(Self {
            query: Linear::new(p, &format!("{name}.query"), c, d, false)?,
            key: Linear::new(p, &format!("{name}.key"), c, d, false)?,
            value: Linear::new(p, &format!("{name}.value"), c, d, false)?,
        })
    }
    fn forward(&self, x: &Tensor, probability: f64, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let q: Tensor = self.query.forward(x, train)?; // B,T,D
        let k: Tensor = self.key.forward(x, train)?;
        let v: Tensor = self.value.forward(x, train)?;
        let weights: Tensor = attention_weights(&q, &k)?; // B,T,T: softmax по ключам
        dropout(&weights, probability, train, rng)?.matmul(&v)
    }
}

struct MultiHeadAttention {
    heads: Vec<Head>,
    projection: Linear,
    dropout: f64,
}
impl MultiHeadAttention {
    fn new(p: &mut Parameters, name: &str, settings: &Settings) -> Result<Self> {
        let heads: Vec<Head> = (0usize..settings.heads())
            .map(|i: usize| -> Result<Head> { Head::new(p, &format!("{name}.head{i}"), settings.channels(), settings.channels() / settings.heads()) })
            .collect::<Result<Vec<Head>>>()?;
        Ok(Self {
            heads,
            projection: Linear::new(p, &format!("{name}.projection"), settings.channels(), settings.channels(), true)?,
            dropout: settings.dropout(),
        })
    }
    fn forward(&self, x: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let heads: Vec<Tensor> = self
            .heads
            .iter()
            .map(|h: &Head| -> Result<Tensor> { h.forward(x, self.dropout, train, rng) })
            .collect::<Result<Vec<Tensor>>>()?;
        let joined: Tensor = Tensor::cat(&heads, 2)?; // B,T,H*D = B,T,C
        dropout(&self.projection.forward(&joined, train)?, self.dropout, train, rng)
    }
}

struct FeedForward {
    up: Linear,
    down: Linear,
    dropout: f64,
}
impl FeedForward {
    fn new(p: &mut Parameters, name: &str, settings: &Settings) -> Result<Self> {
        Ok(Self {
            up: Linear::new(p, &format!("{name}.up"), settings.channels(), 4 * settings.channels(), true)?,
            down: Linear::new(p, &format!("{name}.down"), 4 * settings.channels(), settings.channels(), true)?,
            dropout: settings.dropout(),
        })
    }
    fn forward(&self, x: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let hidden: Tensor = self.up.forward(x, train)?.relu()?;
        dropout(&self.down.forward(&hidden, train)?, self.dropout, train, rng)
    }
}

struct Block {
    ln1: LayerNorm,
    attention: MultiHeadAttention,
    ln2: LayerNorm,
    ff: FeedForward,
}
impl Block {
    fn new(p: &mut Parameters, name: &str, settings: &Settings) -> Result<Self> {
        Ok(Self {
            ln1: LayerNorm::new(p, &format!("{name}.ln1"), settings.channels())?,
            attention: MultiHeadAttention::new(p, &format!("{name}.attention"), settings)?,
            ln2: LayerNorm::new(p, &format!("{name}.ln2"), settings.channels())?,
            ff: FeedForward::new(p, &format!("{name}.ff"), settings)?,
        })
    }
    fn forward(&self, x: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let x: Tensor = (x + self.attention.forward(&self.ln1.forward(x, train)?, train, rng)?)?;
        &x + self.ff.forward(&self.ln2.forward(&x, train)?, train, rng)?
    }
}

pub(crate) struct Gpt {
    token: Embedding,
    position: Embedding,
    blocks: Vec<Block>,
    final_norm: LayerNorm,
    lm_head: Linear,
    context: usize,
}
impl Gpt {
    pub(crate) fn new(p: &mut Parameters, vocab: usize, settings: &Settings) -> Result<Self> {
        settings.validate()?;
        Ok(Self {
            token: Embedding::new(p, "token", vocab, settings.channels())?,
            position: Embedding::new(p, "position", settings.block(), settings.channels())?,
            blocks: (0..settings.layers())
                .map(|i: usize| -> Result<Block> { Block::new(p, &format!("block{i}"), settings) })
                .collect::<Result<Vec<Block>>>()?,
            final_norm: LayerNorm::new(p, "final_norm", settings.channels())?,
            lm_head: Linear::new(p, "lm_head", settings.channels(), vocab, true)?,
            context: settings.block(),
        })
    }
    pub(crate) fn context(&self) -> usize {
        self.context
    }
    pub(crate) fn forward(&self, ids: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let (_, t): (usize, usize) = ids.dims2()?;
        if t == 0 || t > self.context {
            candle_core::bail!("Нужно 0<T<=block_size");
        }
        let positions: Tensor = Tensor::arange(0u32, t as u32, ids.device())?;
        let mut x: Tensor = self.token.forward(ids, train)?.broadcast_add(&self.position.forward(&positions, train)?)?;
        self.blocks.iter().try_for_each(|block: &Block| -> Result<()> {
            x = block.forward(&x, train, rng)?;
            Ok(())
        })?;
        self.lm_head.forward(&self.final_norm.forward(&x, train)?, train)
    }
}

pub(crate) fn loss(logits: &Tensor, targets: &Tensor) -> Result<Tensor> {
    let (b, t, v): (usize, usize, usize) = logits.dims3()?;
    if targets.dims() != [b, t] {
        candle_core::bail!("targets должны иметь форму B,T");
    }
    candle_nn::loss::cross_entropy(&logits.reshape((b * t, v))?, &targets.flatten_all()?)
}

// Три эквивалентных способа усреднить прошлое: цикл, нижнетреугольная матрица,
// masked softmax нулевых scores. Это отправная точка перехода к Q/K/V.
pub(crate) fn demonstrate_attention() -> Result<()> {
    let x: Tensor = Tensor::new(&[[1f32], [2.], [3.], [4.]], &Device::Cpu)?;
    let zeros: Tensor = Tensor::zeros((1, 4, 1), DType::F32, &Device::Cpu)?;
    let weights: Tensor = attention_weights(&zeros, &zeros)?.squeeze(0)?;
    let by_matrix: Vec<Vec<f32>> = weights.matmul(&x)?.to_vec2::<f32>()?;
    let by_loop: Vec<f32> = (1..=4).map(|t: usize| -> f32 { (1usize..=t).sum::<usize>() as f32 / t as f32 }).collect();
    println!("\nДо Q/K/V: causal average для [1,2,3,4]");
    let lower: Vec<f32> = (0..4)
        .flat_map(|row: usize| {
            (0usize..4).map(move |col: usize| -> f32 {
                if col <= row { 1.0 / (row + 1) as f32 } else { 0.0 }
            })
        })
        .collect();
    let by_lower: Vec<Vec<f32>> = Tensor::from_vec(lower, (4, 4), &Device::Cpu)?.matmul(&x)?.to_vec2::<f32>()?;
    println!("Цикл: {by_loop:?}; нижнетреугольная матрица × x: {by_lower:?}");
    println!("Masked softmax × x: {by_matrix:?}");
    println!("Матрица внимания: {:?}", weights.to_vec2::<f32>()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::{BlockSize, Channels, Dropout, HeadCount, LayerCount};
    fn settings() -> Result<Settings> {
        Ok(Settings {
            block: BlockSize::try_from(4)?,
            channels: Channels::try_from(8)?,
            heads: HeadCount::try_from(2)?,
            layers: LayerCount::try_from(2)?,
            dropout: Dropout::try_from(0.2)?,
            ..crate::settings::test_settings()?
        })
    }

    #[test]
    fn attention_is_normalized_causal_and_matches_prefix_averages() -> Result<()> {
        let zeros: Tensor = Tensor::zeros((1, 4, 2), DType::F32, &Device::Cpu)?;
        let weights: Vec<Vec<Vec<f32>>> = attention_weights(&zeros, &zeros)?.to_vec3::<f32>()?;
        weights[0].iter().enumerate().for_each(|(t, row): (usize, &Vec<f32>)| {
            assert!((row.iter().sum::<f32>() - 1.0).abs() < 1e-6);
            row.iter().enumerate().for_each(|(s, &w): (usize, &f32)| {
                let expected: f32 = if s > t { 0.0 } else { 1.0 / (t + 1) as f32 };
                assert!((w - expected).abs() < 1e-6);
            });
        });
        let values: Tensor = Tensor::new(&[[[1f32], [2.], [3.], [4.]]], &Device::Cpu)?;
        let output: Vec<f32> = attention_weights(&zeros, &zeros)?.matmul(&values)?.flatten_all()?.to_vec1::<f32>()?;
        output.iter().zip([1.0f32, 1.5, 2.0, 2.5]).for_each(|(a, b): (&f32, f32)| {
            assert!((a - b).abs() < 1e-6);
        });
        Ok(())
    }

    #[test]
    fn future_tokens_do_not_change_prefix_logits() -> Result<()> {
        let mut p: Parameters = Parameters::new(7);
        let model: Gpt = Gpt::new(&mut p, 5, &settings()?)?;
        let a: Tensor = Tensor::new(&[[0u32, 1, 2, 3]], &Device::Cpu)?;
        let b: Tensor = Tensor::new(&[[0u32, 1, 4, 0]], &Device::Cpu)?;
        let mut rng: Rng = Rng::new(3);
        let a: Tensor = model.forward(&a, false, &mut rng)?;
        let b: Tensor = model.forward(&b, false, &mut rng)?;
        let diff: f32 = (a.narrow(1, 0, 2)? - b.narrow(1, 0, 2)?)?.abs()?.max_all()?.to_scalar::<f32>()?;
        assert!(diff < 1e-6, "future leakage: {diff}");
        Ok(())
    }

    #[test]
    fn gradients_reach_all_parameters_and_eval_detaches_them() -> Result<()> {
        let mut p: Parameters = Parameters::new(7);
        let model: Gpt = Gpt::new(&mut p, 5, &settings()?)?;
        let x: Tensor = Tensor::new(&[[0u32, 1, 2, 3], [3, 2, 1, 4]], &Device::Cpu)?;
        let y: Tensor = Tensor::new(&[[1u32, 2, 3, 4], [2, 1, 4, 0]], &Device::Cpu)?;
        let mut rng: Rng = Rng::new(4);
        let grads: candle_core::backprop::GradStore = loss(&model.forward(&x, true, &mut rng)?, &y)?.backward()?;
        p.named.iter().try_for_each(|(name, var): &(String, Var)| -> Result<()> {
            let grad: &Tensor = grads.get(var).unwrap_or_else(|| -> &Tensor { panic!("Нет градиента: {name}") });
            let values: Vec<f32> = grad.flatten_all()?.to_vec1::<f32>()?;
            assert!(values.iter().all(|x: &f32| -> bool { x.is_finite() }), "{name}");
            assert!(values.iter().any(|x: &f32| -> bool { x.abs() > 0.0 }), "Нулевой градиент: {name}");
            Ok(())
        })?;
        let eval: Tensor = model.forward(&x, false, &mut rng)?;
        let again: Tensor = model.forward(&x, false, &mut rng)?;
        assert_eq!(eval.to_vec3::<f32>()?, again.to_vec3::<f32>()?);
        let grads: candle_core::backprop::GradStore = loss(&eval, &y)?.backward()?;
        assert!(p.named.iter().all(|(_, v): &(String, Var)| -> bool { grads.get(v).is_none() }));
        Ok(())
    }

    #[test]
    fn attention_backward_matches_finite_difference() -> Result<()> {
        // f64 и центральная разность: проверяем градиент Q через mask, softmax и AV.
        let q_values: Vec<f64> = vec![0.1f64, 0.2, 0.3, -0.4, 0.5, 0.6];
        let q: Var = Var::from_vec(q_values.clone(), (1, 3, 2), &Device::Cpu)?;
        let k: Tensor = Tensor::new(&[[[0.2f64, 0.4], [-0.3, 0.7], [0.8, 0.1]]], &Device::Cpu)?;
        let v: Tensor = Tensor::new(&[[[0.2f64], [-0.8], [1.2]]], &Device::Cpu)?;
        let objective: &dyn Fn(&Tensor) -> Result<Tensor> = &|q: &Tensor| -> Result<Tensor> { attention_weights(q, &k)?.matmul(&v)?.sqr()?.sum_all() };
        let gradients: candle_core::backprop::GradStore = objective(q.as_tensor())?.backward()?;
        let analytic: Vec<f64> = gradients.get(&q).unwrap().flatten_all()?.to_vec1::<f64>()?;
        (0usize..q_values.len()).try_for_each(|i: usize| -> Result<()> {
            let mut plus: Vec<f64> = q_values.clone();
            plus[i] += 1e-5;
            let mut minus: Vec<f64> = q_values.clone();
            minus[i] -= 1e-5;
            let fp: f64 = objective(&Tensor::from_vec(plus, (1, 3, 2), &Device::Cpu)?)?.to_scalar::<f64>()?;
            let fm: f64 = objective(&Tensor::from_vec(minus, (1, 3, 2), &Device::Cpu)?)?.to_scalar::<f64>()?;
            let numerical: f64 = (fp - fm) / 2e-5;
            assert!((analytic[i] - numerical).abs() < 1e-7, "i={i}: analytic={}, numerical={numerical}", analytic[i]);
            Ok(())
        })?;
        Ok(())
    }

    #[test]
    fn dropout_is_seeded_and_layer_norm_uses_channels_only() -> Result<()> {
        let ones: Tensor = Tensor::ones((2, 4, 8), DType::F32, &Device::Cpu)?;
        let a: Vec<f32> = dropout(&ones, 0.5, true, &mut Rng::new(1))?.flatten_all()?.to_vec1::<f32>()?;
        let b: Vec<f32> = dropout(&ones, 0.5, true, &mut Rng::new(1))?.flatten_all()?.to_vec1::<f32>()?;
        assert_eq!(a, b);
        assert!(a.contains(&0.0) && a.contains(&2.0));
        assert_eq!(dropout(&ones, 0.5, false, &mut Rng::new(1))?.flatten_all()?.to_vec1::<f32>()?, vec![1f32; 64]);
        let mut p: Parameters = Parameters::new(1);
        let norm: LayerNorm = LayerNorm::new(&mut p, "ln", 3)?;
        let x: Tensor = Tensor::new(&[[[1f32, 2., 3.], [101., 102., 103.]]], &Device::Cpu)?;
        let y: Vec<Vec<Vec<f32>>> = norm.forward(&x, false)?.to_vec3::<f32>()?;
        assert_eq!(y[0][0], y[0][1]);
        assert!(y[0][0].iter().sum::<f32>().abs() < 1e-6);
        assert!((y[0][0][0] + 1.2247356).abs() < 1e-5);
        Ok(())
    }

    #[test]
    fn loss_is_mean_cross_entropy() -> Result<()> {
        let zeros: Tensor = Tensor::zeros((2, 4, 5), DType::F32, &Device::Cpu)?;
        let targets: Tensor = Tensor::zeros((2, 4), DType::U32, &Device::Cpu)?;
        assert!((loss(&zeros, &targets)?.to_scalar::<f32>()? - 5f32.ln()).abs() < 1e-6);
        Ok(())
    }
}

#[cfg(test)]
mod lecture_tests {
    use super::*;
    use crate::settings::{BlockSize, Channels, Dropout, HeadCount, LayerCount};
    #[test]
    fn lecture_architecture_parameter_count_and_short_forward() -> Result<()> {
        let settings: Settings = Settings {
            block: BlockSize::try_from(256)?,
            channels: Channels::try_from(384)?,
            heads: HeadCount::try_from(6)?,
            layers: LayerCount::try_from(6)?,
            dropout: Dropout::try_from(0.2)?,
            ..crate::settings::test_settings()?
        };
        let mut params: Parameters = Parameters::new(1337);
        let model: Gpt = Gpt::new(&mut params, 65, &settings)?;
        // V*C + T*C + L*(12*C*C + 13*C) + 2*C + C*V + V.
        assert_eq!(params.count(), 10_788_929);
        let ids: Tensor = Tensor::new(&[[0u32, 1, 2]], &Device::Cpu)?;
        let out: Tensor = model.forward(&ids, false, &mut Rng::new(0))?;
        assert_eq!(out.dims(), [1, 3, 65]);
        assert!(out.flatten_all()?.to_vec1::<f32>()?.iter().all(|x: &f32| -> bool { x.is_finite() }));
        Ok(())
    }
}
