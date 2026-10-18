//! Все матрицы хранятся построчно. Производные вычисляются до обновления весов.
#[derive(Clone)]
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub fn uniform(&mut self) -> f32 {
        ((self.next() >> 40) as f32 + 0.5) / 16777216.0
    }
    pub fn index(&mut self, n: usize) -> usize {
        let n = n as u64;
        let threshold = n.wrapping_neg() % n;
        loop {
            let x = self.next();
            if x >= threshold {
                return (x % n) as usize;
            }
        }
    }
    fn normal(&mut self) -> f32 {
        (-2.0 * self.uniform().ln()).sqrt() * (std::f32::consts::TAU * self.uniform()).cos()
    }
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.index(i + 1);
            items.swap(i, j);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Example {
    pub context: [usize; 3],
    pub target: usize,
}
pub fn dataset(words: &[&str], chars: &[char]) -> Vec<Example> {
    let mut result = Vec::new();
    for word in words {
        let mut context = [0; 3];
        for ch in word.chars().chain(std::iter::once('.')) {
            let target = chars.iter().position(|&c| c == ch).expect("символ словаря");
            result.push(Example { context, target });
            context = [context[1], context[2], target];
        }
    }
    result
}

#[derive(Clone)]
pub struct Model {
    pub v: usize,
    pub d: usize,
    pub h: usize,
    // C, W1, b1, W2, b2 — те же пять обучаемых тензоров, что в notebook.
    pub p: [Vec<f32>; 5],
}
impl Model {
    pub fn new(v: usize, d: usize, h: usize, rng: &mut Rng) -> Self {
        Self {
            v,
            d,
            h,
            p: [v * d, 3 * d * h, h, h * v, v].map(|n| (0..n).map(|_| rng.normal()).collect()),
        }
    }
    pub fn forward(&self, context: [usize; 3]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let [c, w1, b1, w2, b2] = &self.p;
        // C[X]: (3,d), затем view: (3*d).
        let x: Vec<_> = context
            .iter()
            .flat_map(|&i| c[i * self.d..(i + 1) * self.d].iter().copied())
            .collect();
        let mut hidden = b1.clone();
        for (i, &xi) in x.iter().enumerate() {
            for (hj, &w) in hidden.iter_mut().zip(&w1[i * self.h..(i + 1) * self.h]) {
                *hj += xi * w;
            }
        }
        for hj in &mut hidden {
            *hj = hj.tanh();
        }
        let mut logits = b2.clone();
        for (i, &hi) in hidden.iter().enumerate() {
            for (z, &w) in logits.iter_mut().zip(&w2[i * self.v..(i + 1) * self.v]) {
                *z += hi * w;
            }
        }
        (x, hidden, logits)
    }
    pub fn loss(&self, data: &[Example]) -> f32 {
        assert!(!data.is_empty());
        data.iter()
            .map(|e| cross_entropy(&self.forward(e.context).2, e.target).0 as f64)
            .sum::<f64>() as f32
            / data.len() as f32
    }
    pub fn gradients(&self, batch: &[Example]) -> (f32, [Vec<f32>; 5]) {
        assert!(!batch.is_empty());
        let mut g = self.p.each_ref().map(|p| vec![0.0; p.len()]);
        let [gc, gw1, gb1, gw2, gb2] = &mut g;
        let mut loss = 0.0;
        for e in batch {
            let (x, hidden, logits) = self.forward(e.context);
            let (l, mut dz) = cross_entropy(&logits, e.target);
            loss += l;
            // dL/dlogits = (softmax - one_hot(target)) / batch_size.
            dz[e.target] -= 1.0;
            for z in &mut dz {
                *z /= batch.len() as f32;
            }
            for (b, &z) in gb2.iter_mut().zip(&dz) {
                *b += z;
            }
            let mut da = vec![0.0; self.h];
            for j in 0..self.h {
                for k in 0..self.v {
                    gw2[j * self.v + k] += hidden[j] * dz[k];
                    da[j] += self.p[3][j * self.v + k] * dz[k];
                }
                da[j] *= 1.0 - hidden[j] * hidden[j]; // производная tanh
                gb1[j] += da[j];
            }
            for i in 0..x.len() {
                let mut dx = 0.0;
                for j in 0..self.h {
                    gw1[i * self.h + j] += x[i] * da[j];
                    dx += self.p[1][i * self.h + j] * da[j];
                }
                // Scatter-add: повторяющиеся символы суммируют градиенты одной строки C.
                gc[e.context[i / self.d] * self.d + i % self.d] += dx;
            }
        }
        (loss / batch.len() as f32, g)
    }
    pub fn step(&mut self, batch: &[Example], lr: f32) -> f32 {
        let (loss, gradients) = self.gradients(batch);
        for (p, g) in self.p.iter_mut().zip(gradients) {
            for (p, g) in p.iter_mut().zip(g) {
                *p -= lr * g;
            }
        }
        loss
    }
    pub fn sample(&self, chars: &[char], rng: &mut Rng) -> String {
        let mut context = [0; 3];
        let mut name = String::new();
        for _ in 0..100 {
            let logits = self.forward(context).2;
            let (_, probs) = cross_entropy(&logits, 0);
            let u = rng.uniform();
            let mut sum = 0.0;
            let mut ix = self.v - 1;
            for (i, p) in probs.iter().enumerate() {
                sum += p;
                if u < sum {
                    ix = i;
                    break;
                }
            }
            name.push(chars[ix]);
            if ix == 0 {
                return name;
            }
            context = [context[1], context[2], ix];
        }
        name.push_str(" [лимит 100 символов]");
        name
    }
}

pub fn cross_entropy(logits: &[f32], target: usize) -> (f32, Vec<f32>) {
    // log-sum-exp: exp(logits) напрямую может переполниться.
    let max = logits.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let mut probs: Vec<_> = logits.iter().map(|z| (z - max).exp()).collect();
    let sum: f32 = probs.iter().sum();
    for p in &mut probs {
        *p /= sum;
    }
    ((max - logits[target]) + sum.ln(), probs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contexts_and_end_marker() {
        let d = dataset(&["ab", "a"], &['.', 'a', 'b']);
        assert_eq!(
            d,
            vec![
                Example {
                    context: [0, 0, 0],
                    target: 1
                },
                Example {
                    context: [0, 0, 1],
                    target: 2
                },
                Example {
                    context: [0, 1, 2],
                    target: 0
                },
                Example {
                    context: [0, 0, 0],
                    target: 1
                },
                Example {
                    context: [0, 0, 1],
                    target: 0
                }
            ]
        );
    }
    #[test]
    fn stable_loss() {
        let (l, p) = cross_entropy(&[1000.0, 1001.0, 999.0], 1);
        assert!((l - 0.407606).abs() < 1e-5);
        assert!((p.iter().sum::<f32>() - 1.0).abs() < 1e-6);
    }
    #[test]
    fn all_gradients_match_finite_differences_including_repeated_embeddings() {
        let mut m = Model::new(3, 2, 4, &mut Rng::new(17));
        let data = dataset(&["aa", "ab"], &['.', 'a', 'b']);
        let (_, g) = m.gradients(&data);
        for (tensor, grads) in g.iter().enumerate() {
            for (i, &analytic) in grads.iter().enumerate() {
                let old = m.p[tensor][i];
                m.p[tensor][i] = old + 0.002;
                let plus = m.loss(&data);
                m.p[tensor][i] = old - 0.002;
                let minus = m.loss(&data);
                m.p[tensor][i] = old;
                let numeric = (plus - minus) / 0.004;
                assert!(
                    (analytic - numeric).abs() < 0.0004,
                    "tensor={tensor} index={i}: {analytic} vs {numeric}"
                );
            }
        }
    }
    #[test]
    fn learns_tiny_dataset() {
        let data = dataset(&["ab"], &['.', 'a', 'b']);
        let mut m = Model::new(3, 2, 8, &mut Rng::new(42));
        let before = m.loss(&data);
        for _ in 0..1000 {
            m.step(&data, 0.1);
        }
        assert!(m.loss(&data) < 0.03);
        assert!(m.loss(&data) < before);
        assert_eq!(m.sample(&['.', 'a', 'b'], &mut Rng::new(12)), "ab.");
    }
}
