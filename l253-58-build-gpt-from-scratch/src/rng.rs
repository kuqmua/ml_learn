//! Собственный RNG: одинаковый seed управляет инициализацией, batch, dropout и sampling.
//! Разные потоки не дают оценке loss изменить будущие обучающие batch.
#[derive(Clone)]
pub(crate) struct Rng(u64);
impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        Self(seed)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z: u64 = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    pub(crate) fn uniform(&mut self) -> f64 {
        (self.next() >> 11) as f64 / 9007199254740992.0
    }
    pub(crate) fn normal(&mut self) -> f32 {
        let radius: f64 = (-2.0 * (1.0 - self.uniform()).ln()).sqrt();
        (radius * (std::f64::consts::TAU * self.uniform()).cos()) as f32
    }
    pub(crate) fn index(&mut self, upper: usize) -> usize {
        assert!(upper > 0);
        let bound: u64 = upper as u64;
        loop {
            let n: u64 = self.next();
            if n >= bound.wrapping_neg() % bound {
                return (n % bound) as usize;
            }
        }
    }
}
