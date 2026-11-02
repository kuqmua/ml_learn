//! RandomNumberGenerator воспроизводит последовательность чисел по random_seed.
//! Генераторы используются для инициализации весов, выборки окон, случайного обнуления и генерации текста.
//! Независимые генераторы не позволяют оценке ошибки изменить будущие обучающие окна.
//! Это отдельные последовательности случайных чисел, а не параллельные потоки ОС.
#[derive(Clone)]
pub(crate) struct RandomNumberGenerator {
    generator_state: u64,
}
impl RandomNumberGenerator {
    pub(crate) fn new(random_seed: u64) -> Self {
        Self { generator_state: random_seed }
    }
    fn next_u64(&mut self) -> u64 {
        self.generator_state = self.generator_state.wrapping_add(0x9e3779b97f4a7c15);
        let mut mixed_bits: u64 = self.generator_state;
        mixed_bits = (mixed_bits ^ (mixed_bits >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        mixed_bits = (mixed_bits ^ (mixed_bits >> 27)).wrapping_mul(0x94d049bb133111eb);
        mixed_bits ^ (mixed_bits >> 31)
    }
    pub(crate) fn uniform_probability(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / 9007199254740992.0
    }
    pub(crate) fn standard_normal_value(&mut self) -> f32 {
        let normal_sample_radius: f64 = (-2.0 * (1.0 - self.uniform_probability()).ln()).sqrt();
        (normal_sample_radius * (std::f64::consts::TAU * self.uniform_probability()).cos()) as f32
    }
    pub(crate) fn sample_index(&mut self, upper_bound_exclusive: usize) -> usize {
        assert!(upper_bound_exclusive > 0);
        let upper_bound_u64: u64 = upper_bound_exclusive as u64;
        loop {
            let random_bits: u64 = self.next_u64();
            if random_bits >= upper_bound_u64.wrapping_neg() % upper_bound_u64 {
                return (random_bits % upper_bound_u64) as usize;
            }
        }
    }
}
