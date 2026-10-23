//! Простой вариант урока: весь алгоритм находится в main, без собственных структур и методов.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CONTEXT: usize = 3;
    const EMBEDDING_DIMENSIONS: usize = 10;
    const INPUT: usize = CONTEXT * EMBEDDING_DIMENSIONS;
    const FIRST_LAYER: usize = 200;
    const TRAINING_STEPS: usize = 200_000;
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(42);
    let (vocabulary, train, dev, test): (
        Vec<char>,
        Vec<([usize; CONTEXT], usize)>,
        Vec<([usize; CONTEXT], usize)>,
        Vec<([usize; CONTEXT], usize)>,
    ) = {
        // 1. Одинаковые имена группируем: они должны целиком попасть в одну часть данных.
        let mut groups: std::collections::BTreeMap<&str, Vec<&str>> = std::collections::BTreeMap::<&str, Vec<&str>>::new();
        for name in include_str!("../data/names.txt").lines() {
            groups.entry(name).or_default().push(name);
        }
        let mut groups: Vec<Vec<&str>> = groups.into_values().collect();
        rand::seq::SliceRandom::shuffle(groups.as_mut_slice(), &mut rng);
        let train_end: usize = groups.len() * 8 / 10;
        let dev_end: usize = groups.len() * 9 / 10;
        let train_names: Vec<&str> = groups[..train_end].iter().flatten().copied().collect();
        let dev_names: Vec<&str> = groups[train_end..dev_end].iter().flatten().copied().collect();
        let test_names: Vec<&str> = groups[dev_end..].iter().flatten().copied().collect();
        // 2. Словарь строим только по train; точка с индексом 0 обозначает границу имени.
        let mut vocabulary: Vec<char> = train_names.iter().flat_map(|name: &&str| -> std::str::Chars<'_> { name.chars() }).collect();
        vocabulary.sort_unstable();
        vocabulary.dedup();
        vocabulary.insert(0, '.');
        let examples = |names: &[&str]| -> Vec<([usize; CONTEXT], usize)> {
            let mut result: Vec<([usize; CONTEXT], usize)> = Vec::new();
            for name in names {
                let mut previous_characters_indexes: [usize; CONTEXT] = [0; CONTEXT];
                for character in name.chars().chain(std::iter::once('.')) {
                    let next: usize = vocabulary
                        .iter()
                        .position(|&item: &char| -> bool { item == character })
                        .expect("символ отсутствует в train-словаре");
                    result.push((previous_characters_indexes, next));
                    previous_characters_indexes.rotate_left(1);
                    previous_characters_indexes[CONTEXT - 1] = next;
                }
            }
            result
        };
        let train: Vec<([usize; CONTEXT], usize)> = examples(&train_names);
        let dev: Vec<([usize; CONTEXT], usize)> = examples(&dev_names);
        let test: Vec<([usize; CONTEXT], usize)> = examples(&test_names);
        (vocabulary, train, dev, test)
    };
    // 3. Пять обычных массивов параметров: эмбеддинги, веса и смещения двух слоёв.
    let (mut embeddings, mut first_weights, mut first_biases, mut second_weights, mut second_biases): (
        Vec<[f32; EMBEDDING_DIMENSIONS]>,
        [[f32; FIRST_LAYER]; INPUT],
        [f32; FIRST_LAYER],
        Vec<[f32; FIRST_LAYER]>,
        Vec<f32>,
    ) = {
        let mut normal = || -> f32 { rand_distr::Distribution::sample(&rand_distr::StandardNormal, &mut rng) };
        let embeddings: Vec<[f32; EMBEDDING_DIMENSIONS]> = (0..vocabulary.len())
            .map(|_: usize| -> [f32; EMBEDDING_DIMENSIONS] { std::array::from_fn(|_: usize| -> f32 { normal() }) })
            .collect();
        let first_weights: [[f32; FIRST_LAYER]; INPUT] =
            std::array::from_fn(|_: usize| -> [f32; FIRST_LAYER] { std::array::from_fn(|_: usize| -> f32 { normal() }) });
        let first_biases: [f32; FIRST_LAYER] = std::array::from_fn(|_: usize| -> f32 { normal() });
        let mut second_weights: Vec<[f32; FIRST_LAYER]> = vec![[0.0; FIRST_LAYER]; vocabulary.len()];
        for first_index in 0..FIRST_LAYER {
            for weights in &mut second_weights {
                weights[first_index] = normal();
            }
        }
        let second_biases: Vec<f32> = (0..vocabulary.len()).map(|_: usize| -> f32 { normal() }).collect();
        (embeddings, first_weights, first_biases, second_weights, second_biases)
    };
    let forward = |previous_characters_indexes: [usize; CONTEXT],
                   embeddings: &[[f32; EMBEDDING_DIMENSIONS]],
                   first_weights: &[[f32; FIRST_LAYER]; INPUT],
                   first_biases: &[f32; FIRST_LAYER],
                   second_weights: &[[f32; FIRST_LAYER]],
                   second_biases: &[f32]|
     -> ([f32; INPUT], [f32; FIRST_LAYER], Vec<f32>) {
        // Достаём три эмбеддинга и объединяем в 30 координат.
        let mut input: [f32; INPUT] = [0.0; INPUT];
        for (position, character_index) in previous_characters_indexes.into_iter().enumerate() {
            input[position * EMBEDDING_DIMENSIONS..(position + 1) * EMBEDDING_DIMENSIONS].copy_from_slice(&embeddings[character_index]);
        }
        // Первый слой: 30 входов → 200 значений, затем tanh.
        let mut first_values: [f32; FIRST_LAYER] = *first_biases;
        for (input_index, &coordinate) in input.iter().enumerate() {
            for (first_index, value) in first_values.iter_mut().enumerate() {
                *value += coordinate * first_weights[input_index][first_index];
            }
        }
        for value in &mut first_values {
            *value = value.tanh();
        }
        // Второй слой: 200 значений → одна оценка для каждого символа.
        let mut scores: Vec<f32> = second_biases.to_vec();
        for (first_index, &value) in first_values.iter().enumerate() {
            for (character_index, score) in scores.iter_mut().enumerate() {
                *score += value * second_weights[character_index][first_index];
            }
        }
        (input, first_values, scores)
    };
    // Softmax и устойчивая формула ошибки правильного следующего символа.
    let probabilities_and_loss = |scores: &[f32], target: usize| -> (Vec<f32>, f32) {
        let maximum: f32 = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let exponentials: Vec<f32> = scores.iter().map(|score: &f32| -> f32 { (score - maximum).exp() }).collect();
        let sum: f32 = exponentials.iter().sum();
        let probabilities: Vec<f32> = exponentials.into_iter().map(|value: f32| -> f32 { value / sum }).collect();
        let loss: f32 = (maximum - scores[target]) + sum.ln();
        (probabilities, loss)
    };
    let _initial_dev_loss: f64 = {
        let mut initial_loss: f64 = 0.0f64;
        for &(previous_characters_indexes, next) in &dev {
            let (_, _, scores): ([f32; INPUT], [f32; FIRST_LAYER], Vec<f32>) = forward(
                previous_characters_indexes,
                &embeddings,
                &first_weights,
                &first_biases,
                &second_weights,
                &second_biases,
            );
            initial_loss += probabilities_and_loss(&scores, next).1 as f64;
        }
        initial_loss / dev.len() as f64
    };
    // 4. Берём последовательные батчи, вычисляем loss и градиенты, обновляем параметры.
    {
        const BATCH: usize = 32;
        let mut batch_start: usize = 0;
        for step in 0..TRAINING_STEPS {
            let batch_end: usize = (batch_start + BATCH).min(train.len());
            let batch: &[([usize; CONTEXT], usize)] = &train[batch_start..batch_end];
            batch_start = if batch_end == train.len() { 0 } else { batch_end };
            let mut embedding_gradients: Vec<[f32; EMBEDDING_DIMENSIONS]> = vec![[0.0; EMBEDDING_DIMENSIONS]; vocabulary.len()];
            let mut first_weight_gradients: [[f32; FIRST_LAYER]; INPUT] = [[0.0; FIRST_LAYER]; INPUT];
            let mut first_bias_gradients: [f32; FIRST_LAYER] = [0.0; FIRST_LAYER];
            let mut second_weight_gradients: Vec<[f32; FIRST_LAYER]> = vec![[0.0; FIRST_LAYER]; vocabulary.len()];
            let mut second_bias_gradients: Vec<f32> = vec![0.0; vocabulary.len()];
            for &(previous_characters_indexes, next) in batch {
                let (input, first_values, scores): ([f32; INPUT], [f32; FIRST_LAYER], Vec<f32>) = forward(
                    previous_characters_indexes,
                    &embeddings,
                    &first_weights,
                    &first_biases,
                    &second_weights,
                    &second_biases,
                );
                let (probabilities, _): (Vec<f32>, f32) = probabilities_and_loss(&scores, next);
                // Производная softmax + cross-entropy: (вероятность − правильный ответ) / размер батча.
                let mut score_gradients: Vec<f32> = probabilities;
                for (index, gradient) in score_gradients.iter_mut().enumerate() {
                    *gradient = if index == next {
                        (*gradient - 1.0) / batch.len() as f32
                    } else {
                        *gradient / batch.len() as f32
                    };
                }
                // Второй слой: накапливаем производные по его смещениям и весам.
                for (character_index, &gradient) in score_gradients.iter().enumerate() {
                    second_bias_gradients[character_index] += gradient;
                    for first_index in 0..FIRST_LAYER {
                        second_weight_gradients[character_index][first_index] += first_values[first_index] * gradient;
                    }
                }
                // Возвращаем градиент через второй слой и tanh к первому слою.
                let mut first_gradients: [f32; FIRST_LAYER] = [0.0; FIRST_LAYER];
                for first_index in 0..FIRST_LAYER {
                    for (character_index, &gradient) in score_gradients.iter().enumerate() {
                        first_gradients[first_index] += second_weights[character_index][first_index] * gradient;
                    }
                    first_gradients[first_index] *= 1.0 - first_values[first_index] * first_values[first_index];
                    first_bias_gradients[first_index] += first_gradients[first_index];
                }
                // Первый слой и эмбеддинги. Повторяющиеся символы получают сумму вкладов.
                for input_index in 0..INPUT {
                    let mut input_gradient: f32 = 0.0;
                    for first_index in 0..FIRST_LAYER {
                        first_weight_gradients[input_index][first_index] += input[input_index] * first_gradients[first_index];
                        input_gradient += first_weights[input_index][first_index] * first_gradients[first_index];
                    }
                    embedding_gradients[previous_characters_indexes[input_index / EMBEDDING_DIMENSIONS]][input_index % EMBEDDING_DIMENSIONS] += input_gradient;
                }
            }
            let learning_rate: f32 = if step < 100_000 { 0.1 } else { 0.01 };
            let update = |parameters: &mut [f32], gradients: &[f32]| -> () {
                for (parameter, gradient) in parameters.iter_mut().zip(gradients) {
                    *parameter -= learning_rate * gradient;
                }
            };
            update(embeddings.as_flattened_mut(), embedding_gradients.as_flattened());
            update(first_weights.as_flattened_mut(), first_weight_gradients.as_flattened());
            update(&mut first_biases, &first_bias_gradients);
            update(second_weights.as_flattened_mut(), second_weight_gradients.as_flattened());
            update(&mut second_biases, &second_bias_gradients);
        }
    }
    // 5. Оценка на всех трёх частях: здесь параметры уже не изменяются.
    let _losses: Vec<f64> = {
        let mut losses: Vec<f64> = Vec::new();
        for data in [&train, &dev, &test] {
            let mut total_loss: f64 = 0.0f64;
            for &(previous_characters_indexes, next) in data {
                let (_, _, scores): ([f32; INPUT], [f32; FIRST_LAYER], Vec<f32>) = forward(
                    previous_characters_indexes,
                    &embeddings,
                    &first_weights,
                    &first_biases,
                    &second_weights,
                    &second_biases,
                );
                total_loss += probabilities_and_loss(&scores, next).1 as f64;
            }
            losses.push(total_loss / data.len() as f64);
        }
        losses
    };
    // 6. Генерация: начинаем с «...», выбираем символ по вероятностям и сдвигаем контекст.
    let _generated_names: Vec<String> = {
        let mut names: Vec<String> = Vec::new();
        for _ in 0..20 {
            let mut previous_characters_indexes: [usize; CONTEXT] = [0; CONTEXT];
            let mut name: String = String::new();
            for _ in 0..100 {
                let (_, _, scores): ([f32; INPUT], [f32; FIRST_LAYER], Vec<f32>) = forward(
                    previous_characters_indexes,
                    &embeddings,
                    &first_weights,
                    &first_biases,
                    &second_weights,
                    &second_biases,
                );
                let (probabilities, _): (Vec<f32>, f32) = probabilities_and_loss(&scores, 0);
                let mut remaining: f32 = rand::RngExt::random::<f32>(&mut rng);
                let mut next: usize = vocabulary.len() - 1;
                for (index, probability) in probabilities.iter().enumerate() {
                    remaining -= probability;
                    if remaining < 0.0 {
                        next = index;
                        break;
                    }
                }
                if next == 0 {
                    break;
                }
                name.push(vocabulary[next]);
                previous_characters_indexes.rotate_left(1);
                previous_characters_indexes[CONTEXT - 1] = next;
            }
            names.push(name);
        }
        names
    };
    Ok(())
}
