// Генератор случайных чисел: одинаковый seed воспроизводит ту же последовательность.
#[derive(Clone)]
pub struct RandomGenerator {
    state: u64,
}
impl RandomGenerator {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mixed_bits = self.state;
        let mixed_bits = (mixed_bits ^ (mixed_bits >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        let mixed_bits = (mixed_bits ^ (mixed_bits >> 27)).wrapping_mul(0x94d049bb133111eb);
        mixed_bits ^ (mixed_bits >> 31)
    }
    pub fn uniform(&mut self) -> f32 {
        ((self.next() >> 40) as f32 + 0.5) / 16777216.0
    }
    pub fn index(&mut self, upper_bound: usize) -> usize {
        let upper_bound = upper_bound as u64;
        loop {
            let random_bits = self.next();
            if random_bits >= upper_bound.wrapping_neg() % upper_bound {
                return (random_bits % upper_bound) as usize;
            }
        }
    }
    pub fn normal(&mut self) -> f32 {
        (-2.0 * self.uniform().ln()).sqrt() * (std::f32::consts::TAU * self.uniform()).cos()
    }
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for current_index in (1..items.len()).rev() {
            items.swap(current_index, self.index(current_index + 1));
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrainingExample {
    pub context_character_indices: [usize; 3],
    pub target_character_index: usize,
}
pub fn dataset(names: &[&str], vocabulary: &[char]) -> Vec<TrainingExample> {
    let mut examples = Vec::new();
    for name in names {
        let mut context_character_indices = [0; 3];

        for next_character in name.chars().chain(std::iter::once('.')) {
            let target_character_index = vocabulary
                .iter()
                .position(|&vocabulary_character| vocabulary_character == next_character)
                .expect("символ словаря");
            examples.push(TrainingExample {
                context_character_indices,
                target_character_index,
            });
            context_character_indices = [
                context_character_indices[1],
                context_character_indices[2],
                target_character_index,
            ];
        }
    }
    examples
}

// character_scores (logits) — оценки букв до softmax; это ещё не вероятности.
pub fn cross_entropy(character_scores: &[f32], target_character_index: usize) -> (f32, Vec<f32>) {
    // log-sum-exp: exp(logits) напрямую может переполниться.
    let maximum_score = character_scores
        .iter()
        .copied()
        .fold(f32::NEG_INFINITY, f32::max);
    let (character_probabilities, exponential_sum) = {
        let mut character_probabilities: Vec<_> = character_scores
            .iter()
            .map(|character_score| (character_score - maximum_score).exp())
            .collect();
        let exponential_sum: f32 = character_probabilities.iter().sum();
        for character_probability in &mut character_probabilities {
            *character_probability /= exponential_sum;
        }

        (character_probabilities, exponential_sum)
    };
    (
        (maximum_score - character_scores[target_character_index]) + exponential_sum.ln(),
        character_probabilities,
    )
}
