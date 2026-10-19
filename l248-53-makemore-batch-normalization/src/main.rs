#[cfg_attr(test, test)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_started_at: std::time::Instant = std::time::Instant::now();
    // Генератор случайных чисел: одинаковый seed воспроизводит ту же последовательность.
    #[derive(Clone)]
    struct RandomGenerator {
        state: u64,
    }
    impl RandomGenerator {
        fn new(seed: u64) -> Self {
            Self { state: seed }
        }
        fn next(&mut self) -> u64 {
            self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
            let mixed_bits: u64 = self.state;
            let mixed_bits: u64 =
                (mixed_bits ^ (mixed_bits >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            let mixed_bits: u64 =
                (mixed_bits ^ (mixed_bits >> 27)).wrapping_mul(0x94d049bb133111eb);
            mixed_bits ^ (mixed_bits >> 31)
        }
        fn uniform(&mut self) -> f32 {
            ((self.next() >> 40) as f32 + 0.5) / 16777216.0
        }
        fn index(&mut self, upper_bound: usize) -> usize {
            let upper_bound: u64 = upper_bound as u64;
            loop {
                let random_bits: u64 = self.next();
                if random_bits >= upper_bound.wrapping_neg() % upper_bound {
                    return (random_bits % upper_bound) as usize;
                }
            }
        }
        fn normal(&mut self) -> f32 {
            (-2.0 * self.uniform().ln()).sqrt() * (std::f32::consts::TAU * self.uniform()).cos()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TrainingExample {
        context_character_indices: [usize; 3],
        target_character_index: usize,
    }
    impl TrainingExample {
        fn dataset(names: &[&str], vocabulary: &[char]) -> Vec<TrainingExample> {
            let mut examples: Vec<TrainingExample> = Vec::new();
            for name in names {
                let mut context_character_indices: [usize; 3] = [0; 3];

                for next_character in name.chars().chain(std::iter::once('.')) {
                    let target_character_index: usize = vocabulary
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
    }

    #[derive(Clone)]
    struct Parameter {
        values: Vec<f32>,
        gradients: Vec<f32>,
    }
    impl Parameter {
        fn new(values: Vec<f32>) -> Self {
            Self {
                gradients: vec![0.; values.len()],
                values,
            }
        }
        fn update(&mut self, rate: f32) {
            for (value, gradient) in self.values.iter_mut().zip(&self.gradients) {
                *value -= rate * gradient;
            }
        }
    }
    #[derive(Clone)]
    struct Linear {
        inputs: usize,
        outputs: usize,
        weights: Parameter,
        input: Vec<f32>,
        output: Vec<f32>,
    }
    impl Linear {
        fn new(inputs: usize, outputs: usize, gain: f32, rng: &mut RandomGenerator) -> Self {
            Self {
                inputs,
                outputs,
                weights: Parameter::new(
                    (0..inputs * outputs)
                        .map(|_| rng.normal() * gain / (inputs as f32).sqrt())
                        .collect(),
                ),
                input: vec![],
                output: vec![],
            }
        }
        fn forward(&mut self, input: Vec<f32>) -> Vec<f32> {
            let output: Vec<f32> = {
                let mut output: Vec<f32> = vec![0.; input.len() / self.inputs * self.outputs];
                for (row, result) in input
                    .chunks_exact(self.inputs)
                    .zip(output.chunks_exact_mut(self.outputs))
                {
                    for (index, &value) in row.iter().enumerate() {
                        for (target, &weight) in result.iter_mut().zip(
                            &self.weights.values[index * self.outputs..(index + 1) * self.outputs],
                        ) {
                            *target += value * weight;
                        }
                    }
                }

                output
            };
            self.input = input;
            self.output = output.clone();
            output
        }
        fn backward(&mut self, gradient: Vec<f32>) -> Vec<f32> {
            self.weights.gradients.fill(0.);
            let mut input_gradient: Vec<f32> = vec![0.; self.input.len()];
            for ((input, upstream), downstream) in self
                .input
                .chunks_exact(self.inputs)
                .zip(gradient.chunks_exact(self.outputs))
                .zip(input_gradient.chunks_exact_mut(self.inputs))
            {
                for index in 0..self.inputs {
                    for target in 0..self.outputs {
                        self.weights.gradients[index * self.outputs + target] +=
                            input[index] * upstream[target];
                        downstream[index] +=
                            self.weights.values[index * self.outputs + target] * upstream[target];
                    }
                }
            }
            input_gradient
        }
    }
    #[derive(Clone)]
    struct BatchNorm {
        width: usize,
        gain: Parameter,
        bias: Parameter,
        running_mean: Vec<f32>,
        running_scale: Vec<f32>,
        momentum: f32,
        epsilon: f32,
        // Первая часть усредняет std, вторая — variance. Это разные алгоритмы.
        average_standard_deviation: bool,
        normalized: Vec<f32>,
        inverse_scale: Vec<f32>,
    }
    impl BatchNorm {
        fn statistics(input: &[f32], width: usize) -> Result<(Vec<f32>, Vec<f32>), &'static str> {
            #[derive(Clone, Copy, Debug)]
            // Прямоугольный батч с минимум двумя строками.
            struct BatchColumns<'a> {
                values: &'a [f32],
                width: usize,
            }
            impl<'a> TryFrom<(&'a [f32], usize)> for BatchColumns<'a> {
                type Error = &'static str;
                fn try_from((values, width): (&'a [f32], usize)) -> Result<Self, Self::Error> {
                    if width == 0 || !values.len().is_multiple_of(width) || values.len() / width < 2
                    {
                        return Err(
                            "BatchNorm требует положительную ширину и минимум две полные строки",
                        );
                    }
                    Ok(Self { values, width })
                }
            }
            let batch: BatchColumns<'_> = BatchColumns::try_from((input, width))?;

            let input: &[f32] = batch.values;
            let width: usize = batch.width;
            let count: usize = batch.values.len() / batch.width;
            let mean: Vec<f32> = {
                let mut mean: Vec<f32> = vec![0.; width];
                for row in input.chunks_exact(width) {
                    for (total, value) in mean.iter_mut().zip(row) {
                        *total += value / count as f32;
                    }
                }

                mean
            };
            let mut variance: Vec<f32> = vec![0.; width];
            for row in input.chunks_exact(width) {
                for index in 0..width {
                    variance[index] += (row[index] - mean[index]).powi(2) / (count - 1) as f32;
                }
            }
            Ok((mean, variance))
        }

        fn new(width: usize, first_part: bool, output: bool) -> Self {
            Self {
                width,
                gain: Parameter::new(vec![if output { 0.1 } else { 1. }; width]),
                bias: Parameter::new(vec![0.; width]),
                running_mean: vec![0.; width],
                running_scale: vec![1.; width],
                momentum: if first_part { 0.001 } else { 0.1 },
                epsilon: if first_part { 0. } else { 1e-5 },
                average_standard_deviation: first_part,
                normalized: vec![],
                inverse_scale: vec![],
            }
        }

        fn forward(&mut self, input: Vec<f32>, training: bool) -> Result<Vec<f32>, &'static str> {
            let (mean, scale): (Vec<f32>, Vec<f32>) = if training {
                let (mean, variance): (Vec<f32>, Vec<f32>) =
                    Self::statistics(input.as_slice(), self.width)?;
                let scale: Vec<f32> = variance
                    .iter()
                    .map(|v| {
                        if self.average_standard_deviation {
                            v.sqrt()
                        } else {
                            *v
                        }
                    })
                    .collect();
                for index in 0..self.width {
                    self.running_mean[index] = (1. - self.momentum) * self.running_mean[index]
                        + self.momentum * mean[index];
                    self.running_scale[index] = (1. - self.momentum) * self.running_scale[index]
                        + self.momentum * scale[index];
                }
                (mean, scale)
            } else {
                (self.running_mean.clone(), self.running_scale.clone())
            };
            self.inverse_scale = scale
                .iter()
                .map(|v| {
                    1. / if self.average_standard_deviation {
                        v.max(1e-12)
                    } else {
                        (v + self.epsilon).sqrt()
                    }
                })
                .collect();
            self.normalized = input
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    (value - mean[index % self.width]) * self.inverse_scale[index % self.width]
                })
                .collect();
            Ok(self
                .normalized
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    self.gain.values[index % self.width] * value
                        + self.bias.values[index % self.width]
                })
                .collect())
        }
        fn backward(&mut self, gradient: Vec<f32>) -> Vec<f32> {
            let count: usize = gradient.len() / self.width;
            self.gain.gradients.fill(0.);
            self.bias.gradients.fill(0.);
            for (index, value) in gradient.iter().enumerate() {
                self.gain.gradients[index % self.width] += value * self.normalized[index];
                self.bias.gradients[index % self.width] += value;
            }
            // Делитель n-1 соответствует torch.var/std с correction=1.
            gradient
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let column: usize = index % self.width;
                    self.gain.values[column]
                        * self.inverse_scale[column]
                        * (value
                            - self.bias.gradients[column] / count as f32
                            - self.normalized[index] * self.gain.gradients[column]
                                / (count - 1) as f32)
                })
                .collect()
        }
    }
    #[derive(Clone)]
    enum Layer {
        Linear(Linear),
        BatchNorm(BatchNorm),
        Tanh {
            output: Vec<f32>,
            gradient: Vec<f32>,
        },
    }

    #[derive(Clone)]
    struct Model {
        embeddings: Parameter,
        layers: Vec<Layer>,
        vocabulary_size: usize,
    }
    impl Model {
        // Оценки букв до softmax превращаются в вероятности и cross-entropy.
        fn cross_entropy(
            character_scores: &[f32],
            target_character_index: usize,
        ) -> (f32, Vec<f32>) {
            // log-sum-exp: exp(logits) напрямую может переполниться.
            let maximum_score: f32 = character_scores
                .iter()
                .copied()
                .fold(f32::NEG_INFINITY, f32::max);
            let (character_probabilities, exponential_sum): (Vec<f32>, f32) = {
                let mut character_probabilities: Vec<f32> = character_scores
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

        fn new(vocabulary_size: usize, deep: bool, rng: &mut RandomGenerator) -> Self {
            Self {
                embeddings: Parameter::new(
                    (0..vocabulary_size * 10).map(|_| rng.normal()).collect(),
                ),
                layers: {
                    let mut layers: Vec<Layer> = vec![];

                    if deep {
                        let mut inputs: usize = 30;
                        for outputs in [100, 100, 100, 100, 100, vocabulary_size] {
                            layers.push(Layer::Linear(Linear::new(inputs, outputs, 1., rng)));
                            layers.push(Layer::BatchNorm(BatchNorm::new(
                                outputs,
                                false,
                                outputs == vocabulary_size,
                            )));
                            if outputs != vocabulary_size {
                                layers.push(Layer::Tanh {
                                    output: vec![],
                                    gradient: vec![],
                                });
                            }
                            inputs = outputs;
                        }
                    } else {
                        layers.push(Layer::Linear(Linear::new(30, 200, 5. / 3., rng)));
                        layers.push(Layer::BatchNorm(BatchNorm::new(200, true, false)));
                        layers.push(Layer::Tanh {
                            output: vec![],
                            gradient: vec![],
                        });
                        // notebook W2 = randn * 0.01; Linear обычно делит на sqrt(fan_in).
                        layers.push(Layer::Linear(Linear::new(
                            200,
                            vocabulary_size,
                            0.01 * 200f32.sqrt(),
                            rng,
                        )));
                        // Выходное смещение b2 хранится отдельно в основном цикле.
                    }
                    layers
                },
                vocabulary_size,
            }
        }
        fn forward(
            &mut self,
            batch: &[TrainingExample],
            training: bool,
        ) -> Result<Vec<f32>, &'static str> {
            let values: Vec<f32> = {
                let mut values: Vec<f32> = Vec::with_capacity(batch.len() * 30);
                for example in batch {
                    for &character in &example.context_character_indices {
                        values.extend_from_slice(
                            &self.embeddings.values[character * 10..character * 10 + 10],
                        );
                    }
                }

                values
            };
            self.layers
                .iter_mut()
                .try_fold(values, |values, layer| match layer {
                    Layer::Linear(layer) => Ok(layer.forward(values)),
                    Layer::BatchNorm(layer) => layer.forward(values, training),
                    Layer::Tanh { output, .. } => {
                        *output = values.iter().map(|x| x.tanh()).collect();
                        Ok(output.clone())
                    }
                })
        }
        fn parameters(&mut self) -> Vec<&mut Parameter> {
            let mut result: Vec<&mut Parameter> = vec![&mut self.embeddings];
            for layer in &mut self.layers {
                match layer {
                    Layer::Linear(layer) => result.push(&mut layer.weights),
                    Layer::BatchNorm(layer) => {
                        result.push(&mut layer.gain);
                        result.push(&mut layer.bias);
                    }
                    _ => {}
                }
            }
            result
        }
        fn gradients(
            &mut self,
            batch: &[TrainingExample],
            output_bias: &mut Parameter,
        ) -> Result<f32, &'static str> {
            let (gradient, loss): (Vec<f32>, f32) = {
                let mut gradient: Vec<f32> = vec![];
                let mut loss: f32 = 0.;
                output_bias.gradients.fill(0.);
                for (row, example) in self
                    .forward(batch, true)?
                    .chunks_exact(self.vocabulary_size)
                    .zip(batch)
                {
                    let probabilities: Vec<f32> = {
                        let (value, mut probabilities): (f32, Vec<f32>) = Model::cross_entropy(
                            &row.iter()
                                .zip(&output_bias.values)
                                .map(|(a, b)| a + b)
                                .collect::<Vec<f32>>(),
                            example.target_character_index,
                        );
                        loss += value / batch.len() as f32;
                        probabilities[example.target_character_index] -= 1.;
                        for (index, p) in probabilities.iter_mut().enumerate() {
                            *p /= batch.len() as f32;
                            output_bias.gradients[index] += *p;
                        }

                        probabilities
                    };
                    gradient.extend(probabilities);
                }

                (gradient, loss)
            };
            let gradient: Vec<f32> =
                self.layers
                    .iter_mut()
                    .rev()
                    .fold(gradient, |upstream, layer| match layer {
                        Layer::Linear(layer) => layer.backward(upstream),
                        Layer::BatchNorm(layer) => layer.backward(upstream),
                        Layer::Tanh { output, gradient } => {
                            *gradient = upstream.clone();
                            upstream
                                .iter()
                                .zip(output)
                                .map(|(g, h)| g * (1. - *h * *h))
                                .collect()
                        }
                    });
            self.embeddings.gradients.fill(0.);
            for (example, row) in batch.iter().zip(gradient.as_chunks::<30>().0.iter()) {
                for (position, character) in example.context_character_indices.iter().enumerate() {
                    for coordinate in 0..10 {
                        self.embeddings.gradients[character * 10 + coordinate] +=
                            row[position * 10 + coordinate];
                    }
                }
            }
            Ok(loss)
        }
    }
    fn standard_deviation(values: &[f32]) -> f32 {
        let mean: f32 = values.iter().sum::<f32>() / values.len() as f32;
        (values.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / (values.len() - 1) as f32).sqrt()
    }

    #[cfg(test)]
    if cfg!(test) {
        // invalid_batch_is_returned_without_changing_running_statistics
        {
            let mut layer: BatchNorm = BatchNorm::new(2, false, false);
            let mean: Vec<f32> = layer.running_mean.clone();
            let scale: Vec<f32> = layer.running_scale.clone();
            for input in [vec![], vec![1., 2.], vec![1., 2., 3.]] {
                assert!(layer.forward(input, true).is_err());
            }
            assert_eq!(layer.running_mean, mean);
            assert_eq!(layer.running_scale, scale);
            let mut model: Model = Model::new(3, false, &mut RandomGenerator::new(42));
            let mut bias: Parameter = Parameter::new(vec![0.; 3]);
            assert!(
                model
                    .gradients(
                        &[TrainingExample {
                            context_character_indices: [0; 3],
                            target_character_index: 1,
                        }],
                        &mut bias
                    )
                    .is_err()
            );
        }
        // batch_norm_backward_matches_finite_differences
        {
            for first_part in [false, true] {
                let input: Vec<f32> = vec![0.2, -1.1, 0.7, 0.3, -0.4, 1.2, 2.1, -0.7];
                let upstream: Vec<f32> = vec![0.3, -0.2, 0.4, 0.8, -0.1, 0.5, 0.2, -0.6];
                let (layer, analytic): (BatchNorm, Vec<f32>) = {
                    let mut layer: BatchNorm = BatchNorm::new(2, first_part, false);
                    layer.gain.values = vec![1.3, 0.6];
                    layer.forward(input.clone(), true)?;
                    let analytic: Vec<f32> = layer.backward(upstream.clone());

                    (layer, analytic)
                };
                for index in 0..input.len() {
                    let plus: Vec<f32> = {
                        let mut plus: Vec<f32> = input.clone();
                        plus[index] += 0.001;
                        plus
                    };
                    let minus: Vec<f32> = {
                        let mut minus: Vec<f32> = input.clone();
                        minus[index] -= 0.001;
                        minus
                    };
                    let objective = |values: Vec<f32>| -> Result<f32, &'static str> {
                        let mut copy: BatchNorm = layer.clone();
                        Ok(copy
                            .forward(values, true)?
                            .iter()
                            .zip(&upstream)
                            .map(|(x, g)| x * g)
                            .sum::<f32>())
                    };
                    let numeric: f32 = (objective(plus)? - objective(minus)?) / 0.002;
                    assert!(
                        (numeric - analytic[index]).abs() < 0.0005,
                        "{first_part} {index}: {numeric} vs {}",
                        analytic[index]
                    );
                }
            }
        }
        // full_network_gradients_include_embeddings_and_all_layer_types
        {
            let batch: Vec<TrainingExample> =
                TrainingExample::dataset(&["aba", "baa"], &['.', 'a', 'b']);
            for deep in [false, true] {
                let (mut model, bias): (Model, Parameter) = {
                    let mut model: Model = Model::new(3, deep, &mut RandomGenerator::new(55));
                    let mut bias: Parameter = Parameter::new(vec![0.; 3]);
                    model.gradients(&batch, &mut bias)?;
                    (model, bias)
                };
                for tensor in 0..model.parameters().len() {
                    let (index, analytic): (usize, f32) = {
                        let p: &&mut Parameter = &model.parameters()[tensor];
                        let index: usize = p
                            .gradients
                            .iter()
                            .enumerate()
                            .max_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
                            .unwrap()
                            .0;
                        (index, p.gradients[index])
                    };
                    let objective = |offset: f32| -> Result<f32, &'static str> {
                        let mut copy: Model = model.clone();
                        copy.parameters()[tensor].values[index] += offset;
                        copy.gradients(&batch, &mut bias.clone())
                    };
                    let numeric: f32 = (objective(0.001)? - objective(-0.001)?) / 0.002;
                    assert!(
                        (numeric - analytic).abs() < 0.003,
                        "deep={deep} tensor={tensor}: {numeric} vs {analytic}"
                    );
                }
            }
        }
        // eval_uses_frozen_statistics_and_is_independent_of_companions
        {
            let mut layer: BatchNorm = BatchNorm::new(2, false, false);
            layer.forward(vec![1., 2., 3., 7., -2., 4.], true)?;
            let mean: Vec<f32> = layer.running_mean.clone();
            let variance: Vec<f32> = layer.running_scale.clone();
            assert_eq!(
                layer.forward(vec![1., 2.], false)?,
                &layer.forward(vec![1., 2., 100., -30.], false)?[..2],
            );
            assert_eq!(mean, layer.running_mean);
            assert_eq!(variance, layer.running_scale);
            // torch correction=1: variance([1,3]) = 2, а не 1.
            assert_eq!(
                BatchNorm::statistics(&[1., 3.][..], 1)?,
                (vec![2.], vec![2.])
            );
            assert!(
                BatchNorm::new(2, false, false)
                    .forward(vec![3.; 8], true)?
                    .iter()
                    .all(|x| x.is_finite())
            );
        }
        // sgd_learns_and_dataset_resets_context
        {
            let batch: Vec<TrainingExample> =
                TrainingExample::dataset(&["ab", "ab", "ab", "ab"], &['.', 'a', 'b']);
            assert_eq!(batch[3].context_character_indices, [0; 3]);
            for deep in [false, true] {
                let mut model: Model = Model::new(3, deep, &mut RandomGenerator::new(42));
                let mut bias: Parameter = Parameter::new(vec![0.; 3]);
                let initial: f32 = model.gradients(&batch, &mut bias)?;
                for _ in 0..100 {
                    model.gradients(&batch, &mut bias)?;
                    for p in model.parameters() {
                        p.update(0.1);
                    }
                    if !deep {
                        bias.update(0.1);
                    }
                }
                let final_loss: f32 = model.gradients(&batch, &mut bias)?;
                assert!(
                    final_loss < initial * 0.5,
                    "{deep}: {initial} -> {final_loss}"
                );
            }
        }

        return Ok(());
    }

    let mut files: std::collections::BTreeMap<String, std::fs::File> = {
        let output_directory: std::path::PathBuf =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("output");
        std::fs::create_dir_all(&output_directory)?;
        let file_names: Vec<String> = {
            let mut file_names: Vec<String> = vec![
                "calibration.csv".to_owned(),
                "batch_coupling.csv".to_owned(),
                "batch_coupling.html".to_owned(),
                "linear_statistics.csv".to_owned(),
            ];
            for prefix in ["shallow", "deep"] {
                for suffix in [
                    "loss.csv",
                    "updates.csv",
                    "loss.svg",
                    "embedding_gradient.svg",
                    "weight_statistics.csv",
                    "diagnostics.csv",
                    "metrics.txt",
                    "samples.txt",
                ] {
                    file_names.push(format!("{prefix}_{suffix}"));
                }
                let (linear_layers, tanh_layers): (&[usize], &[usize]) = if prefix == "deep" {
                    (&[0, 3, 6, 9, 12, 15], &[2, 5, 8, 11, 14])
                } else {
                    (&[0, 3], &[2])
                };
                for index in linear_layers {
                    for extension in ["csv", "svg"] {
                        file_names.push(format!("{prefix}_{index}_weight_gradient.{extension}"));
                    }
                }
                for index in tanh_layers {
                    for kind in ["activation", "activation_gradient"] {
                        for extension in ["csv", "svg"] {
                            file_names.push(format!("{prefix}_{index}_{kind}.{extension}"));
                        }
                    }
                }
                if prefix == "deep" {
                    for index in 0..19 {
                        file_names.push(format!("{prefix}_parameter_{index}_updates.svg"));
                    }
                }
            }
            file_names
        };
        let mut files: std::collections::BTreeMap<String, std::fs::File> =
            std::collections::BTreeMap::new();
        for name in file_names {
            files.insert(
                name.clone(),
                std::fs::File::create(output_directory.join(name))?,
            );
        }
        files
    };
    let (vocabulary, training, validation, test): (
        Vec<char>,
        Vec<TrainingExample>,
        Vec<TrainingExample>,
        Vec<TrainingExample>,
    ) = {
        // Поле закрыто; наружу доступен только неизменяемый срез проверенных имён.
        #[derive(Debug)]
        struct Names<'a>(Vec<&'a str>);
        impl<'a> TryFrom<&'a str> for Names<'a> {
            type Error = &'static str;
            fn try_from(text: &'a str) -> Result<Self, Self::Error> {
                let names: Vec<&str> = text.lines().collect();
                if names.is_empty()
                    || names.iter().any(|name| {
                        name.is_empty()
                            || !name.chars().all(|character| character.is_ascii_lowercase())
                    })
                {
                    return Err("нужен непустой набор непустых имён из букв a–z");
                }
                Ok(Self(names))
            }
        }
        impl<'a> std::ops::Deref for Names<'a> {
            type Target = [&'a str];
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        let text: &str =
            include_str!("../../l247-52-generate-names-with-makemore-mlp/data/names.txt");
        let names: Names<'_> = Names::try_from(text)?;
        let vocabulary: Vec<char> = {
            let mut vocabulary: Vec<char> =
                text.chars().filter(|c| c.is_ascii_lowercase()).collect();
            vocabulary.sort_unstable();
            vocabulary.dedup();
            vocabulary.insert(0, '.');

            vocabulary
        };
        let names: Names<'_> = {
            let mut names: Names<'_> = names;
            let mut generator: RandomGenerator = RandomGenerator::new(42);
            for current_index in (1..names.0.len()).rev() {
                names
                    .0
                    .swap(current_index, generator.index(current_index + 1));
            }
            names
        };
        let training: Vec<TrainingExample> =
            TrainingExample::dataset(&names[..names.len() * 8 / 10], &vocabulary);
        let validation: Vec<TrainingExample> = TrainingExample::dataset(
            &names[names.len() * 8 / 10..names.len() * 9 / 10],
            &vocabulary,
        );
        let test: Vec<TrainingExample> =
            TrainingExample::dataset(&names[names.len() * 9 / 10..], &vocabulary);
        println!(
            "train/dev/test: {}/{}/{} contexts",
            training.len(),
            validation.len(),
            test.len()
        );

        (vocabulary, training, validation, test)
    };
    // Первая сеть: 200 000 шагов; глубокая: 1001 (ячейка 14: break при i >= 1000).
    for (deep, steps, decay) in [
        (false, 200_000usize, 100_000usize),
        (true, 1001usize, 150_000usize),
    ] {
        // Точки графика: горизонтальная и вертикальная координаты.
        type PlotPoints = Vec<(f32, f32)>;

        fn plot(
            mut output_file: std::fs::File,
            points: &[(f32, f32)],
            title: &str,
        ) -> std::io::Result<()> {
            let min_x: f32 = points.iter().map(|p| p.0).fold(f32::INFINITY, f32::min);
            let max_x: f32 = points.iter().map(|p| p.0).fold(f32::NEG_INFINITY, f32::max);
            let (min_y, max_y): (f32, f32) = {
                let min_y: f32 = points.iter().map(|p| p.1).fold(f32::INFINITY, f32::min);
                let max_y: f32 = points.iter().map(|p| p.1).fold(f32::NEG_INFINITY, f32::max);
                if title.contains("reference -3") {
                    (min_y.min(-3.), max_y.max(-3.))
                } else {
                    (min_y, max_y)
                }
            };
            let svg: String = {
                let mut svg: String = format!(
                    "<svg xmlns='http://www.w3.org/2000/svg' width='900' height='500'><rect width='900' height='500' fill='white'/><text x='50' y='25'>{title}; x=[{min_x:.3},{max_x:.3}], y=[{min_y:.3},{max_y:.3}]</text><polyline fill='none' stroke='royalblue' points='"
                );
                for &(x, y) in points {
                    svg.push_str(&format!(
                        "{:.2},{:.2} ",
                        50. + 800. * (x - min_x) / (max_x - min_x).max(1e-12),
                        450. - 400. * (y - min_y) / (max_y - min_y).max(1e-12)
                    ));
                }
                svg.push_str("'/>");
                if title.contains("reference -3") && min_y <= -3. && max_y >= -3. {
                    svg.push_str(&format!(
                        "<path d='M50 {}H850' stroke='black' stroke-dasharray='5 5'/>",
                        450. - 400. * (-3. - min_y) / (max_y - min_y).max(1e-12),
                    ));
                }
                svg.push_str("</svg>");

                svg
            };
            std::io::Write::write_all(&mut output_file, svg.as_bytes())
        }
        let prefix: &str = if deep { "deep" } else { "shallow" };
        let (model, bias, curve, update_curves): (Model, Parameter, PlotPoints, Vec<PlotPoints>) = {
            let mut rng: RandomGenerator = RandomGenerator::new(2147483647);
            let mut model: Model = Model::new(vocabulary.len(), deep, &mut rng);
            let mut bias: Parameter = Parameter::new(vec![0.; vocabulary.len()]);
            println!(
                "{prefix}: {} parameters",
                model
                    .parameters()
                    .iter()
                    .map(|p| p.values.len())
                    .sum::<usize>()
                    + if deep { 0 } else { vocabulary.len() }
            );
            let mut losses: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(
                files
                    .remove(&format!("{prefix}_loss.csv"))
                    .ok_or("файл отсутствует в подготовленном наборе")?,
            );
            let mut ratios: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(
                files
                    .remove(&format!("{prefix}_updates.csv"))
                    .ok_or("файл отсутствует в подготовленном наборе")?,
            );
            std::io::Write::write_fmt(
                &mut losses,
                format_args!("step,learning_rate,loss,log10_loss\n"),
            )?;
            std::io::Write::write_fmt(
                &mut ratios,
                format_args!("step,parameter,log10_update_to_data_std\n"),
            )?;
            let mut curve: PlotPoints = Vec::new();
            let mut update_curves: Vec<PlotPoints> = vec![Vec::new(); model.parameters().len()];
            for step in 0..steps {
                #[derive(Clone, Copy, Debug)]
                struct FiniteLoss(f32);
                impl TryFrom<f32> for FiniteLoss {
                    type Error = &'static str;
                    fn try_from(value: f32) -> Result<Self, Self::Error> {
                        if value.is_finite() {
                            Ok(Self(value))
                        } else {
                            Err("ошибка обучения должна быть конечным числом")
                        }
                    }
                }
                impl FiniteLoss {
                    fn get(self) -> f32 {
                        self.0
                    }
                }
                impl std::fmt::Display for FiniteLoss {
                    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        std::fmt::Display::fmt(&self.0, formatter)
                    }
                }

                let loss: FiniteLoss = FiniteLoss::try_from(
                    model.gradients(
                        &(0..32)
                            .map(|_| training[rng.index(training.len())])
                            .collect::<Vec<TrainingExample>>(),
                        &mut bias,
                    )?,
                )?;
                let rate: f32 = if step < decay { 0.1 } else { 0.01 };
                for (index, parameter) in model.parameters().into_iter().enumerate() {
                    parameter.update(rate);
                    let ratio: f32 = rate * standard_deviation(&parameter.gradients)
                        / standard_deviation(&parameter.values);
                    std::io::Write::write_fmt(
                        &mut ratios,
                        format_args!("{step},{index},{}\n", ratio.log10()),
                    )?;
                    if deep && ratio.is_finite() && ratio > 0. {
                        update_curves[index].push((step as f32, ratio.log10()));
                    }
                }
                if !deep {
                    bias.update(rate);
                }
                std::io::Write::write_fmt(
                    &mut losses,
                    format_args!("{step},{rate},{loss},{}\n", loss.get().log10()),
                )?;
                curve.push((step as f32, loss.get().log10()));
                if step % 10_000 == 0 {
                    println!("{prefix} {step}/{steps}: {loss:.4}");
                }
            }
            std::io::Write::flush(&mut losses)?;
            std::io::Write::flush(&mut ratios)?;

            (model, bias, curve, update_curves)
        };
        plot(
            files
                .remove(&format!("{prefix}_loss.svg"))
                .ok_or("файл отсутствует в подготовленном наборе")?,
            &curve,
            "log10 loss",
        )?;
        for (index, points) in update_curves.iter().enumerate() {
            if !points.is_empty() {
                plot(
                    files
                        .remove(&format!("{prefix}_parameter_{index}_updates.svg"))
                        .ok_or("файл отсутствует в подготовленном наборе")?,
                    points,
                    "log10(std(update)/std(parameter)); reference -3",
                )?;
            }
        }
        // Сохраняем диагностику последнего train-batch ДО eval, который меняет caches.
        {
            fn histogram(values: &[f32]) -> PlotPoints {
                let min: f32 = values.iter().copied().fold(f32::INFINITY, f32::min);
                let width: f32 = ((values.iter().copied().fold(f32::NEG_INFINITY, f32::max) - min)
                    / 50.)
                    .max(1e-12);
                let bins: [usize; 50] = {
                    let mut bins: [usize; 50] = [0usize; 50];
                    for value in values {
                        bins[(((value - min) / width) as usize).min(49)] += 1;
                    }

                    bins
                };
                bins.iter()
                    .enumerate()
                    .map(|(i, count)| {
                        (
                            min + (i as f32 + 0.5) * width,
                            *count as f32 / (values.len() as f32 * width),
                        )
                    })
                    .collect()
            }

            plot(
                files
                    .remove(&format!("{prefix}_embedding_gradient.svg"))
                    .ok_or("файл отсутствует в подготовленном наборе")?,
                &histogram(&model.embeddings.gradients),
                "embedding gradient density",
            )?;
            {
                let mut weight_statistics: std::fs::File = files
                    .remove(&format!("{prefix}_weight_statistics.csv"))
                    .ok_or("файл отсутствует в подготовленном наборе")?;
                std::io::Write::write_fmt(
                    &mut weight_statistics,
                    format_args!("parameter,gradient_std,data_std,gradient_to_data_std\n"),
                )?;
                for (index, parameter) in model.clone().parameters().iter().enumerate() {
                    let gradient_std: f32 = standard_deviation(&parameter.gradients);
                    let data_std: f32 = standard_deviation(&parameter.values);
                    std::io::Write::write_fmt(
                        &mut weight_statistics,
                        format_args!(
                            "{index},{gradient_std},{data_std},{}\n",
                            gradient_std / data_std
                        ),
                    )?;
                }
            }
            let mut file: std::fs::File = files
                .remove(&format!("{prefix}_diagnostics.csv"))
                .ok_or("файл отсутствует в подготовленном наборе")?;
            std::io::Write::write_fmt(
                &mut file,
                format_args!("layer,kind,mean,std,saturated_fraction\n"),
            )?;
            for (index, layer) in model.layers.iter().enumerate() {
                for (kind, values) in match layer {
                    Layer::Tanh { output, gradient } => {
                        vec![("activation", output), ("activation_gradient", gradient)]
                    }
                    Layer::Linear(linear) => vec![("weight_gradient", &linear.weights.gradients)],
                    _ => vec![],
                } {
                    std::io::Write::write_fmt(
                        &mut file,
                        format_args!(
                            "{index},{kind},{},{},{}\n",
                            values.iter().sum::<f32>() / values.len() as f32,
                            standard_deviation(values),
                            values.iter().filter(|v| v.abs() > 0.97).count() as f32
                                / values.len() as f32,
                        ),
                    )?;
                    let points: PlotPoints = histogram(values);
                    plot(
                        files
                            .remove(&format!("{prefix}_{index}_{kind}.svg"))
                            .ok_or("файл отсутствует в подготовленном наборе")?,
                        &points,
                        kind,
                    )?;
                    let mut histogram: std::fs::File = files
                        .remove(&format!("{prefix}_{index}_{kind}.csv"))
                        .ok_or("файл отсутствует в подготовленном наборе")?;
                    std::io::Write::write_fmt(
                        &mut histogram,
                        format_args!("bin_center,density\n"),
                    )?;
                    for (x, y) in points {
                        std::io::Write::write_fmt(&mut histogram, format_args!("{x},{y}\n"))?;
                    }
                }
            }
        }
        {
            // Eval меняет только caches слоёв; параметры и статистики BN не обновляются.
            let mut model: Model = model;
            {
                let mut metrics: std::fs::File = files
                    .remove(&format!("{prefix}_metrics.txt"))
                    .ok_or("файл отсутствует в подготовленном наборе")?;
                for (name, examples) in
                    [("train", &training), ("dev", &validation), ("test", &test)]
                {
                    let loss: f64 = {
                        let mut total: f64 = 0.;
                        for batch in examples.chunks(256) {
                            for (row, example) in model
                                .forward(batch, false)?
                                .chunks_exact(model.vocabulary_size)
                                .zip(batch)
                            {
                                total += Model::cross_entropy(
                                    &row.iter()
                                        .zip(&bias.values)
                                        .map(|(a, b)| a + b)
                                        .collect::<Vec<f32>>(),
                                    example.target_character_index,
                                )
                                .0 as f64;
                            }
                        }
                        total / examples.len() as f64
                    };
                    println!("{prefix} {name}: {loss:.6}");
                    std::io::Write::write_fmt(&mut metrics, format_args!("{name}: {loss:.6}\n"))?;
                }
            }
            if !deep {
                // Ячейка 9 вычисляет full-train статистики, но ячейка 10 их не использует.
                let (sums, squares): (Vec<f64>, Vec<f64>) = {
                    let mut sums: Vec<f64> = vec![0f64; 200];
                    let mut squares: Vec<f64> = vec![0f64; 200];
                    for batch in training.chunks(256) {
                        model.forward(batch, false)?;
                        if let Layer::Linear(first) = &model.layers[0] {
                            for row in first.output.as_chunks::<200>().0 {
                                for i in 0..200 {
                                    sums[i] += row[i] as f64;
                                    squares[i] += (row[i] as f64).powi(2);
                                }
                            }
                        }
                    }

                    (sums, squares)
                };
                let mut calibration: std::fs::File = files
                    .remove("calibration.csv")
                    .ok_or("файл отсутствует в подготовленном наборе")?;
                std::io::Write::write_fmt(&mut calibration, format_args!("neuron,mean,std\n"))?;
                for i in 0..200 {
                    let n: f64 = training.len() as f64;
                    std::io::Write::write_fmt(
                        &mut calibration,
                        format_args!(
                            "{i},{},{}\n",
                            sums[i] / n,
                            ((squares[i] - sums[i] * sums[i] / n) / (n - 1.))
                                .max(0.)
                                .sqrt()
                        ),
                    )?;
                }
            }
            let mut samples: std::fs::File = files
                .remove(&format!("{prefix}_samples.txt"))
                .ok_or("файл отсутствует в подготовленном наборе")?;
            let mut rng: RandomGenerator = RandomGenerator::new(2147483657);
            for _ in 0..20 {
                let name: String = {
                    let mut context: [usize; 3] = [0; 3];
                    let mut name: String = String::new();
                    for _ in 0..100 {
                        let probabilities: Vec<f32> = Model::cross_entropy(
                            &model
                                .forward(
                                    &[TrainingExample {
                                        context_character_indices: context,
                                        target_character_index: 0,
                                    }],
                                    false,
                                )?
                                .iter()
                                .zip(&bias.values)
                                .map(|(a, b)| a + b)
                                .collect::<Vec<f32>>(),
                            0,
                        )
                        .1;
                        let draw: f32 = rng.uniform();
                        let selected: usize = {
                            let mut cumulative: f32 = 0.;
                            let mut selected: usize = vocabulary.len() - 1;
                            for (i, p) in probabilities.iter().enumerate() {
                                cumulative += p;
                                if draw < cumulative {
                                    selected = i;
                                    break;
                                }
                            }

                            selected
                        };
                        name.push(vocabulary[selected]);
                        if selected == 0 {
                            break;
                        }
                        context = [context[1], context[2], selected];
                    }
                    if !name.ends_with('.') {
                        name.push_str(" [лимит 100 символов]");
                    }

                    name
                };
                std::io::Write::write_fmt(&mut samples, format_args!("{name}\n"))?;
            }
        }
    }
    {
        // Локальный HTML slider и таблица всех положений x0 с шагом 0.5.
        {
            let mut file: std::fs::File = files
                .remove("batch_coupling.csv")
                .ok_or("файл отсутствует в подготовленном наборе")?;
            std::io::Write::write_fmt(&mut file, format_args!("x0,example,input,normalized\n"))?;
            let original: Vec<f32> = {
                let mut rng: RandomGenerator = RandomGenerator::new(2147483648);
                let original: Vec<f32> = (0..5).map(|_| rng.normal() * 5.).collect();

                original
            };
            std::io::Write::write_all(&mut files.remove("batch_coupling.html").ok_or("файл отсутствует в подготовленном наборе")?, ((r##"<!doctype html><meta charset="utf-8"><title>BatchNorm: связь примеров</title>
<style>body{font:18px sans-serif;max-width:950px;margin:30px auto}svg{border:1px solid #ccc}label{display:block}</style>
<h1>BatchNorm: изменение одного примера</h1>
<label>x₀: <input id="slider" type="range" min="-30" max="30" step="0.5" value="0"><output id="number"></output></label>
<p>Синие точки — входы, красные — нормализованные значения. Линии связывают один пример до и после нормализации. Кривые — нормальные плотности с исходными mean/std и с mean=0, std=1.</p>
<svg id="chart" viewBox="0 0 900 400"></svg><pre id="values"></pre>
<script>
const original=ORIGINAL;
const slider=document.getElementById('slider'),chart=document.getElementById('chart');
function render(){
 const x=[...original];x[0]=Number(slider.value);
 const mean=x.reduce((a,b)=>a+b)/5;
 const std=Math.sqrt(x.reduce((a,b)=>a+(b-mean)**2,0)/4);
 const z=x.map(a=>(a-mean)/std);
 const px=a=>450+a*12, py=a=>340-a*180;
 let svg='<path d="M30 340H870" stroke="black"/>';
 for(const [mu,sigma,color] of [[mean,std,'blue'],[0,1,'red']]){
   let points=[];for(let a=-35;a<=35;a+=.1)points.push(px(a)+','+py(Math.exp(-.5*((a-mu)/sigma)**2)/(sigma*Math.sqrt(2*Math.PI))));
   svg+='<polyline fill="none" stroke="'+color+'" points="'+points.join(' ')+'"/>';
 }
 for(let i=0;i<5;i++){
   svg+='<path d="M'+px(x[i])+' 90L'+px(z[i])+' 300" stroke="#aaa"/>';
   svg+='<circle cx="'+px(x[i])+'" cy="90" r="6" fill="blue"/><circle cx="'+px(z[i])+'" cy="300" r="6" fill="red"/>';
 }
 chart.innerHTML=svg;
 document.getElementById('number').textContent=x[0];
 document.getElementById('values').textContent='mean='+mean.toFixed(4)+' std='+std.toFixed(4)+'\ninput: '+x.map(a=>a.toFixed(3)).join(', ')+'\nnormalized: '+z.map(a=>a.toFixed(3)).join(', ');
}
slider.addEventListener('input',render);render();
</script>"##).replace("ORIGINAL", &format!("{original:?}"))).as_bytes())?;
            for step in 0..=120 {
                let values: Vec<f32> = {
                    let mut values: Vec<f32> = original.clone();
                    values[0] = -30. + step as f32 * 0.5;

                    values
                };
                let (mean, variance): (Vec<f32>, Vec<f32>) =
                    BatchNorm::statistics(values.as_slice(), 1)?;
                for (i, x) in values.iter().enumerate() {
                    std::io::Write::write_fmt(
                        &mut file,
                        format_args!(
                            "{},{i},{x},{}\n",
                            values[0],
                            (x - mean[0]) / variance[0].sqrt()
                        ),
                    )?;
                }
            }
        }
        let mut file: std::fs::File = files
            .remove("linear_statistics.csv")
            .ok_or("файл отсутствует в подготовленном наборе")?;
        std::io::Write::write_fmt(&mut file, format_args!("batch_norm,tensor,std\n"))?;
        for normalized in [false, true] {
            let (input, mut linear, values, upstream): (Vec<f32>, Linear, Vec<f32>, Vec<f32>) = {
                let mut rng: RandomGenerator = RandomGenerator::new(2147483647);
                let input: Vec<f32> = (0..1000).map(|_| rng.normal()).collect();
                let mut linear: Linear = Linear::new(1000, 1000, 1000f32.sqrt(), &mut rng);
                let values: Vec<f32> = linear.forward(input.clone());
                let upstream: Vec<f32> = (0..1000).map(|_| rng.normal()).collect();
                (input, linear, values, upstream)
            };
            let (out, gradient): (Vec<f32>, Vec<f32>) = if normalized {
                let mut bn: BatchNorm = BatchNorm::new(1, false, false);
                (
                    bn.forward(values.clone(), true)?,
                    bn.backward(upstream.clone()),
                )
            } else {
                (values.clone(), upstream.clone())
            };
            let input_gradient: Vec<f32> = linear.backward(gradient.clone());
            let linear: Linear = linear;
            for (name, values) in [
                ("input", &input),
                ("weight", &linear.weights.values),
                ("linear_output", &values),
                ("output", &out),
                ("output_gradient", &upstream),
                ("linear_gradient", &gradient),
                ("input_gradient", &input_gradient),
                ("weight_gradient", &linear.weights.gradients),
            ] {
                std::io::Write::write_fmt(
                    &mut file,
                    format_args!("{normalized},{name},{}\n", standard_deviation(values)),
                )?;
            }
        }
    }
    println!(
        "Общее время работы программы: {:.2} с",
        program_started_at.elapsed().as_secs_f64()
    );
    Ok(())
}
