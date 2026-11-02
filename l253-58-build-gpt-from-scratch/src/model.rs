//! Архитектура из ng-video-lecture: отдельные головы, pre-LN, ReLU и обучаемые позиции.
//! Candle предоставляет операции над тензорами и backward, но не готовую GPT-модель.
use crate::rng::Rng;
use candle_core::{D, DType, Device, Result, Tensor, Var};

#[derive(Clone, Debug)]
pub(crate) struct Config {
    pub(crate) block: usize,
    pub(crate) channels: usize,
    pub(crate) heads: usize,
    pub(crate) layers: usize,
    pub(crate) dropout: f64,
}
impl Config {
    pub(crate) fn validate(&self) -> Result<()> {
        if self.block == 0 || self.channels == 0 || self.heads == 0 || self.layers == 0 || !self.channels.is_multiple_of(self.heads) {
            candle_core::bail!("Нужны положительные размеры и channels % heads == 0");
        }
        if !(0.0..1.0).contains(&self.dropout) {
            candle_core::bail!("dropout должен быть в [0,1)");
        }
        Ok(())
    }
}

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
        let values: Vec<f32> = (0..shape.iter().product())
            .map(|_| constant.unwrap_or_else(|| self.rng.normal() * 0.02))
            .collect();
        let var = Var::from_vec(values, shape, &Device::Cpu)?;
        let tensor = var.as_tensor().clone();
        self.named.push((name.to_owned(), var));
        Ok(tensor)
    }
    pub(crate) fn variables(&self) -> Vec<Var> {
        self.named.iter().map(|(_, v)| v.clone()).collect()
    }
    pub(crate) fn count(&self) -> usize {
        self.named.iter().map(|(_, v)| v.elem_count()).sum()
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
        let input = x.dim(D::Minus1)?;
        let output = self.weight.dim(0)?;
        let mut shape = x.dims().to_vec();
        *shape.last_mut().unwrap() = output;
        let y = x.reshape((x.elem_count() / input, input))?.matmul(&parameter(&self.weight, train).t()?)?;
        let y = match &self.bias {
            Some(b) => y.broadcast_add(&parameter(b, train))?,
            None => y,
        };
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
        let mut shape = ids.dims().to_vec();
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
        let centered = x.broadcast_sub(&x.mean_keepdim(D::Minus1)?)?;
        let variance = centered.sqr()?.mean_keepdim(D::Minus1)?;
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
        .map(|_| {
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
        .flat_map(|row| (0..t).map(move |col| if col > row { f32::NEG_INFINITY } else { 0.0 }))
        .collect();
    Tensor::from_vec(values, (t, t), &Device::Cpu)
}

// Q: что ищу; K: по каким признакам меня искать; V: что передам другим позициям.
fn attention_weights(q: &Tensor, k: &Tensor) -> Result<Tensor> {
    let (_, t, d) = q.dims3()?;
    let scores = (q.matmul(&k.transpose(1, 2)?)? / (d as f64).sqrt())?;
    let masked = scores.broadcast_add(&causal_mask(t)?.to_dtype(q.dtype())?)?;
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
        let q = self.query.forward(x, train)?; // B,T,D
        let k = self.key.forward(x, train)?;
        let v = self.value.forward(x, train)?;
        let weights = attention_weights(&q, &k)?; // B,T,T: softmax по ключам
        dropout(&weights, probability, train, rng)?.matmul(&v)
    }
}

struct MultiHeadAttention {
    heads: Vec<Head>,
    projection: Linear,
    dropout: f64,
}
impl MultiHeadAttention {
    fn new(p: &mut Parameters, name: &str, cfg: &Config) -> Result<Self> {
        let heads = (0..cfg.heads)
            .map(|i| Head::new(p, &format!("{name}.head{i}"), cfg.channels, cfg.channels / cfg.heads))
            .collect::<Result<_>>()?;
        Ok(Self {
            heads,
            projection: Linear::new(p, &format!("{name}.projection"), cfg.channels, cfg.channels, true)?,
            dropout: cfg.dropout,
        })
    }
    fn forward(&self, x: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let heads = self.heads.iter().map(|h| h.forward(x, self.dropout, train, rng)).collect::<Result<Vec<_>>>()?;
        let joined = Tensor::cat(&heads, 2)?; // B,T,H*D = B,T,C
        dropout(&self.projection.forward(&joined, train)?, self.dropout, train, rng)
    }
}

struct FeedForward {
    up: Linear,
    down: Linear,
    dropout: f64,
}
impl FeedForward {
    fn new(p: &mut Parameters, name: &str, cfg: &Config) -> Result<Self> {
        Ok(Self {
            up: Linear::new(p, &format!("{name}.up"), cfg.channels, 4 * cfg.channels, true)?,
            down: Linear::new(p, &format!("{name}.down"), 4 * cfg.channels, cfg.channels, true)?,
            dropout: cfg.dropout,
        })
    }
    fn forward(&self, x: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let hidden = self.up.forward(x, train)?.relu()?;
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
    fn new(p: &mut Parameters, name: &str, cfg: &Config) -> Result<Self> {
        Ok(Self {
            ln1: LayerNorm::new(p, &format!("{name}.ln1"), cfg.channels)?,
            attention: MultiHeadAttention::new(p, &format!("{name}.attention"), cfg)?,
            ln2: LayerNorm::new(p, &format!("{name}.ln2"), cfg.channels)?,
            ff: FeedForward::new(p, &format!("{name}.ff"), cfg)?,
        })
    }
    fn forward(&self, x: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let x = (x + self.attention.forward(&self.ln1.forward(x, train)?, train, rng)?)?;
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
    pub(crate) fn new(p: &mut Parameters, vocab: usize, cfg: &Config) -> Result<Self> {
        cfg.validate()?;
        Ok(Self {
            token: Embedding::new(p, "token", vocab, cfg.channels)?,
            position: Embedding::new(p, "position", cfg.block, cfg.channels)?,
            blocks: (0..cfg.layers).map(|i| Block::new(p, &format!("block{i}"), cfg)).collect::<Result<_>>()?,
            final_norm: LayerNorm::new(p, "final_norm", cfg.channels)?,
            lm_head: Linear::new(p, "lm_head", cfg.channels, vocab, true)?,
            context: cfg.block,
        })
    }
    fn forward(&self, ids: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        let (_, t) = ids.dims2()?;
        if t == 0 || t > self.context {
            candle_core::bail!("Нужно 0<T<=block_size");
        }
        let positions = Tensor::arange(0u32, t as u32, ids.device())?;
        let mut x = self.token.forward(ids, train)?.broadcast_add(&self.position.forward(&positions, train)?)?;
        for block in &self.blocks {
            x = block.forward(&x, train, rng)?;
        }
        self.lm_head.forward(&self.final_norm.forward(&x, train)?, train)
    }
}

pub(crate) enum LanguageModel {
    Bigram { table: Tensor, context: usize },
    Gpt(Box<Gpt>),
}
impl LanguageModel {
    pub(crate) fn new(kind: &str, p: &mut Parameters, vocab: usize, cfg: &Config) -> Result<Self> {
        cfg.validate()?;
        match kind {
            // Как в bigram.py: строка таблицы уже является logits следующего символа.
            "bigram" => Ok(Self::Bigram {
                table: p.add("bigram.weight", &[vocab, vocab], None)?,
                context: cfg.block,
            }),
            "gpt" => Ok(Self::Gpt(Box::new(Gpt::new(p, vocab, cfg)?))),
            _ => candle_core::bail!("model должен быть bigram или gpt"),
        }
    }
    pub(crate) fn context(&self) -> usize {
        match self {
            Self::Bigram { context, .. } => *context,
            Self::Gpt(g) => g.context,
        }
    }
    pub(crate) fn forward(&self, ids: &Tensor, train: bool, rng: &mut Rng) -> Result<Tensor> {
        match self {
            Self::Bigram { table, .. } => {
                let (b, t) = ids.dims2()?;
                parameter(table, train).index_select(&ids.flatten_all()?, 0)?.reshape((b, t, table.dim(1)?))
            }
            Self::Gpt(g) => g.forward(ids, train, rng),
        }
    }
}

pub(crate) fn loss(logits: &Tensor, targets: &Tensor) -> Result<Tensor> {
    let (b, t, v) = logits.dims3()?;
    if targets.dims() != [b, t] {
        candle_core::bail!("targets должны иметь форму B,T");
    }
    candle_nn::loss::cross_entropy(&logits.reshape((b * t, v))?, &targets.flatten_all()?)
}

// Три эквивалентных способа усреднить прошлое: цикл, нижнетреугольная матрица,
// masked softmax нулевых scores. Это отправная точка перехода к Q/K/V.
pub(crate) fn demonstrate_attention() -> Result<()> {
    let x = Tensor::new(&[[1f32], [2.], [3.], [4.]], &Device::Cpu)?;
    let zeros = Tensor::zeros((1, 4, 1), DType::F32, &Device::Cpu)?;
    let weights = attention_weights(&zeros, &zeros)?.squeeze(0)?;
    let by_matrix = weights.matmul(&x)?.to_vec2::<f32>()?;
    let by_loop: Vec<f32> = (1..=4).map(|t| (1..=t).sum::<usize>() as f32 / t as f32).collect();
    println!("\nДо Q/K/V: causal average для [1,2,3,4]");
    let lower: Vec<f32> = (0..4)
        .flat_map(|row| (0..4).map(move |col| if col <= row { 1.0 / (row + 1) as f32 } else { 0.0 }))
        .collect();
    let by_lower = Tensor::from_vec(lower, (4, 4), &Device::Cpu)?.matmul(&x)?.to_vec2::<f32>()?;
    println!("Цикл: {by_loop:?}; нижнетреугольная матрица × x: {by_lower:?}");
    println!("Masked softmax × x: {by_matrix:?}");
    println!("Матрица внимания: {:?}", weights.to_vec2::<f32>()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> Config {
        Config {
            block: 4,
            channels: 8,
            heads: 2,
            layers: 2,
            dropout: 0.2,
        }
    }

    #[test]
    fn attention_is_normalized_causal_and_matches_prefix_averages() -> Result<()> {
        let zeros = Tensor::zeros((1, 4, 2), DType::F32, &Device::Cpu)?;
        let weights = attention_weights(&zeros, &zeros)?.to_vec3::<f32>()?;
        for (t, row) in weights[0].iter().enumerate() {
            assert!((row.iter().sum::<f32>() - 1.0).abs() < 1e-6);
            for (s, &w) in row.iter().enumerate() {
                let expected = if s > t { 0.0 } else { 1.0 / (t + 1) as f32 };
                assert!((w - expected).abs() < 1e-6);
            }
        }
        let values = Tensor::new(&[[[1f32], [2.], [3.], [4.]]], &Device::Cpu)?;
        let output = attention_weights(&zeros, &zeros)?.matmul(&values)?.flatten_all()?.to_vec1::<f32>()?;
        for (a, b) in output.iter().zip([1.0, 1.5, 2.0, 2.5]) {
            assert!((a - b).abs() < 1e-6);
        }
        Ok(())
    }

    #[test]
    fn future_tokens_do_not_change_prefix_logits() -> Result<()> {
        let mut p = Parameters::new(7);
        let model = LanguageModel::new("gpt", &mut p, 5, &config())?;
        let a = Tensor::new(&[[0u32, 1, 2, 3]], &Device::Cpu)?;
        let b = Tensor::new(&[[0u32, 1, 4, 0]], &Device::Cpu)?;
        let mut rng = Rng::new(3);
        let a = model.forward(&a, false, &mut rng)?;
        let b = model.forward(&b, false, &mut rng)?;
        let diff = (a.narrow(1, 0, 2)? - b.narrow(1, 0, 2)?)?.abs()?.max_all()?.to_scalar::<f32>()?;
        assert!(diff < 1e-6, "future leakage: {diff}");
        Ok(())
    }

    #[test]
    fn gradients_reach_all_parameters_and_eval_detaches_them() -> Result<()> {
        let mut p = Parameters::new(7);
        let model = LanguageModel::new("gpt", &mut p, 5, &config())?;
        let x = Tensor::new(&[[0u32, 1, 2, 3], [3, 2, 1, 4]], &Device::Cpu)?;
        let y = Tensor::new(&[[1u32, 2, 3, 4], [2, 1, 4, 0]], &Device::Cpu)?;
        let mut rng = Rng::new(4);
        let grads = loss(&model.forward(&x, true, &mut rng)?, &y)?.backward()?;
        for (name, var) in &p.named {
            let grad = grads.get(var).unwrap_or_else(|| panic!("Нет градиента: {name}"));
            let values = grad.flatten_all()?.to_vec1::<f32>()?;
            assert!(values.iter().all(|x| x.is_finite()), "{name}");
            assert!(values.iter().any(|x| x.abs() > 0.0), "Нулевой градиент: {name}");
        }
        let eval = model.forward(&x, false, &mut rng)?;
        let again = model.forward(&x, false, &mut rng)?;
        assert_eq!(eval.to_vec3::<f32>()?, again.to_vec3::<f32>()?);
        let grads = loss(&eval, &y)?.backward()?;
        assert!(p.named.iter().all(|(_, v)| grads.get(v).is_none()));
        Ok(())
    }

    #[test]
    fn attention_backward_matches_finite_difference() -> Result<()> {
        // f64 и центральная разность: проверяем градиент Q через mask, softmax и AV.
        let q_values = vec![0.1f64, 0.2, 0.3, -0.4, 0.5, 0.6];
        let q = Var::from_vec(q_values.clone(), (1, 3, 2), &Device::Cpu)?;
        let k = Tensor::new(&[[[0.2f64, 0.4], [-0.3, 0.7], [0.8, 0.1]]], &Device::Cpu)?;
        let v = Tensor::new(&[[[0.2f64], [-0.8], [1.2]]], &Device::Cpu)?;
        let objective = |q: &Tensor| -> Result<Tensor> { attention_weights(q, &k)?.matmul(&v)?.sqr()?.sum_all() };
        let gradients = objective(q.as_tensor())?.backward()?;
        let analytic = gradients.get(&q).unwrap().flatten_all()?.to_vec1::<f64>()?;
        for i in 0..q_values.len() {
            let mut plus = q_values.clone();
            plus[i] += 1e-5;
            let mut minus = q_values.clone();
            minus[i] -= 1e-5;
            let fp = objective(&Tensor::from_vec(plus, (1, 3, 2), &Device::Cpu)?)?.to_scalar::<f64>()?;
            let fm = objective(&Tensor::from_vec(minus, (1, 3, 2), &Device::Cpu)?)?.to_scalar::<f64>()?;
            let numerical = (fp - fm) / 2e-5;
            assert!((analytic[i] - numerical).abs() < 1e-7, "i={i}: analytic={}, numerical={numerical}", analytic[i]);
        }
        Ok(())
    }

    #[test]
    fn dropout_is_seeded_and_layer_norm_uses_channels_only() -> Result<()> {
        let ones = Tensor::ones((2, 4, 8), DType::F32, &Device::Cpu)?;
        let a = dropout(&ones, 0.5, true, &mut Rng::new(1))?.flatten_all()?.to_vec1::<f32>()?;
        let b = dropout(&ones, 0.5, true, &mut Rng::new(1))?.flatten_all()?.to_vec1::<f32>()?;
        assert_eq!(a, b);
        assert!(a.contains(&0.0) && a.contains(&2.0));
        assert_eq!(dropout(&ones, 0.5, false, &mut Rng::new(1))?.flatten_all()?.to_vec1::<f32>()?, vec![1f32; 64]);
        let mut p = Parameters::new(1);
        let norm = LayerNorm::new(&mut p, "ln", 3)?;
        let x = Tensor::new(&[[[1f32, 2., 3.], [101., 102., 103.]]], &Device::Cpu)?;
        let y = norm.forward(&x, false)?.to_vec3::<f32>()?;
        assert_eq!(y[0][0], y[0][1]);
        assert!(y[0][0].iter().sum::<f32>().abs() < 1e-6);
        assert!((y[0][0][0] + 1.2247356).abs() < 1e-5);
        Ok(())
    }

    #[test]
    fn bigram_ignores_earlier_tokens_and_loss_is_mean_cross_entropy() -> Result<()> {
        let mut p = Parameters::new(2);
        let m = LanguageModel::new("bigram", &mut p, 5, &config())?;
        let a = Tensor::new(&[[0u32, 1, 2, 3]], &Device::Cpu)?;
        let b = Tensor::new(&[[4u32, 3, 2, 3]], &Device::Cpu)?;
        let mut rng = Rng::new(0);
        let a = m.forward(&a, false, &mut rng)?.narrow(1, 3, 1)?.flatten_all()?.to_vec1::<f32>()?;
        let b = m.forward(&b, false, &mut rng)?.narrow(1, 3, 1)?.flatten_all()?.to_vec1::<f32>()?;
        assert_eq!(a, b);
        let zeros = Tensor::zeros((2, 4, 5), DType::F32, &Device::Cpu)?;
        let targets = Tensor::zeros((2, 4), DType::U32, &Device::Cpu)?;
        assert!((loss(&zeros, &targets)?.to_scalar::<f32>()? - 5f32.ln()).abs() < 1e-6);
        Ok(())
    }
}

#[cfg(test)]
mod lecture_tests {
    use super::*;
    #[test]
    fn lecture_architecture_parameter_count_and_short_forward() -> Result<()> {
        let config = Config {
            block: 256,
            channels: 384,
            heads: 6,
            layers: 6,
            dropout: 0.2,
        };
        let mut params = Parameters::new(1337);
        let model = LanguageModel::new("gpt", &mut params, 65, &config)?;
        // V*C + T*C + L*(12*C*C + 13*C) + 2*C + C*V + V.
        assert_eq!(params.count(), 10_788_929);
        let ids = Tensor::new(&[[0u32, 1, 2]], &Device::Cpu)?;
        let out = model.forward(&ids, false, &mut Rng::new(0))?;
        assert_eq!(out.dims(), [1, 3, 65]);
        assert!(out.flatten_all()?.to_vec1::<f32>()?.iter().all(|x| x.is_finite()));
        Ok(())
    }
}
