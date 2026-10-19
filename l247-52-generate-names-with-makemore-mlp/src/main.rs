// SVG строится самим Rust; ни matplotlib, ни дополнительные программы не нужны.
fn plot(
    output_path: std::path::PathBuf,
    data_points: &[(f32, f32)],
    character_labels: Option<&[char]>,
    title: &str,
) -> std::io::Result<()> {
    let minimum_horizontal_value = data_points
        .iter()
        .map(|data_point| data_point.0)
        .fold(f32::INFINITY, f32::min);
    let maximum_horizontal_value = data_points
        .iter()
        .map(|data_point| data_point.0)
        .fold(f32::NEG_INFINITY, f32::max);
    let minimum_vertical_value = data_points
        .iter()
        .map(|data_point| data_point.1)
        .fold(f32::INFINITY, f32::min);
    let maximum_vertical_value = data_points
        .iter()
        .map(|data_point| data_point.1)
        .fold(f32::NEG_INFINITY, f32::max);
    let to_pixel_coordinates = |data_point: &(f32, f32)| {
        (
            65.0 + 680.0 * (data_point.0 - minimum_horizontal_value)
                / (maximum_horizontal_value - minimum_horizontal_value).max(1e-6),
            535.0
                - 470.0 * (data_point.1 - minimum_vertical_value)
                    / (maximum_vertical_value - minimum_vertical_value).max(1e-6),
        )
    };
    let svg_document = {
        let mut svg_document = format!(
            "<svg xmlns='http://www.w3.org/2000/svg' width='800' height='600' viewBox='0 0 800 600'><rect width='800' height='600' fill='white'/><g font-family='sans-serif' fill='#172554'><text x='65' y='28'>{title}</text><path d='M65 55V535H755' fill='none' stroke='#64748b'/><text x='65' y='565'>{minimum_horizontal_value:.3}</text><text x='690' y='565'>{maximum_horizontal_value:.3}</text><text x='5' y='535'>{minimum_vertical_value:.3}</text><text x='5' y='65'>{maximum_vertical_value:.3}</text>"
        );
        if let Some(character_labels) = character_labels {
            for (data_point, character) in data_points.iter().zip(character_labels) {
                let (horizontal_pixel, vertical_pixel) = to_pixel_coordinates(data_point);
                svg_document.push_str(&format!("<circle cx='{horizontal_pixel}' cy='{vertical_pixel}' r='12' fill='#2563eb'/><text x='{horizontal_pixel}' y='{}' text-anchor='middle' fill='white'>{character}</text>", vertical_pixel+4.0));
            }
        } else {
            svg_document
                .push_str("<polyline fill='none' stroke='#2563eb' stroke-width='0.7' points='");
            for data_point in data_points {
                let (horizontal_pixel, vertical_pixel) = to_pixel_coordinates(data_point);
                svg_document.push_str(&format!("{horizontal_pixel:.2},{vertical_pixel:.2} "));
            }
            svg_document.push_str("'/>");
        }
        svg_document.push_str("</g></svg>");

        svg_document
    };
    std::fs::write(output_path, svg_document)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Настройки урока: меняй здесь и запускай cargo run.
    let training_steps = 200_000usize;
    let output_directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("output");
    assert!(training_steps > 0, "число шагов должно быть положительным");

    std::fs::create_dir_all(&output_directory)?;
    // 1. Данные и словарь: 0 — начало/конец имени, a..z — 1..26.
    let names_text = include_str!("../data/names.txt");
    let names: Vec<_> = names_text.lines().collect();
    let vocabulary = {
        let mut vocabulary: Vec<_> = names_text
            .chars()
            .filter(|character| character.is_ascii_lowercase())
            .collect();
        vocabulary.sort_unstable();
        vocabulary.dedup();
        vocabulary.insert(0, '.');

        vocabulary
    };
    assert!(names.iter().all(|name| {
        !name.is_empty() && name.chars().all(|character| character.is_ascii_lowercase())
    }));
    println!("Имена: {}, первые восемь: {:?}", names.len(), &names[..8]);
    println!("Словарь: {vocabulary:?}");
    let all_examples = dataset(&names, &vocabulary);
    println!(
        "Все X: ({},3), Y: ({},)",
        all_examples.len(),
        all_examples.len()
    );
    for example in dataset(&names[..1], &vocabulary) {
        println!(
            "{} -> {}",
            example
                .context_character_indices
                .map(|character_index| vocabulary[character_index])
                .iter()
                .collect::<String>(),
            vocabulary[example.target_character_index]
        );
    }
    // 2. Маленькая сеть из начала notebook: C(27,2), W1(6,100).
    let demonstration_model = Model::new(
        vocabulary.len(),
        2,
        100,
        &mut RandomGenerator::new(2147483647),
    );
    let (flattened_embeddings, hidden_activations, character_scores) =
        demonstration_model.forward(all_examples[0].context_character_indices);
    let (stable_cross_entropy, character_probabilities) =
        cross_entropy(&character_scores, all_examples[0].target_character_index);
    let exponentiated_scores: Vec<_> = character_scores.iter().map(|score| score.exp()).collect();
    println!(
        "Демо: emb=(1,3,2), flat=(1,{}), h=(1,{}), logits=(1,{}); sum(prob)={:.6}",
        flattened_embeddings.len(),
        hidden_activations.len(),
        character_scores.len(),
        character_probabilities.iter().sum::<f32>()
    );
    println!(
        "Один пример: явный softmax+NLL={:.6}, stable CE={stable_cross_entropy:.6}; loss первых 32={:.6}",
        -(exponentiated_scores[all_examples[0].target_character_index]
            / exponentiated_scores.iter().sum::<f32>())
        .ln(),
        demonstration_model.loss(&all_examples[..32])
    );
    // 3. Разделяем ИМЕНА до построения контекстов: 80/10/10.
    let names = {
        let mut names = names;
        RandomGenerator::new(42).shuffle(&mut names);
        names
    };
    let training_names_end = names.len() * 8 / 10;
    let validation_names_end = names.len() * 9 / 10;
    let training_examples = dataset(&names[..training_names_end], &vocabulary);
    let validation_examples = dataset(
        &names[training_names_end..validation_names_end],
        &vocabulary,
    );
    let test_examples = dataset(&names[validation_names_end..], &vocabulary);
    println!(
        "Имена train/dev/test: {}/{}/{}; примеры: {}/{}/{}",
        training_names_end,
        validation_names_end - training_names_end,
        names.len() - validation_names_end,
        training_examples.len(),
        validation_examples.len(),
        test_examples.len()
    );
    let model = Model::new(
        vocabulary.len(),
        10,
        200,
        &mut RandomGenerator::new(2147483647),
    );
    println!(
        "C=({},{}), W1=({},{}), b1=({}), W2=({},{}), b2=({}); параметров: {}",
        model.vocabulary_size,
        model.embedding_dimensions,
        3 * model.embedding_dimensions,
        model.hidden_neuron_count,
        model.hidden_neuron_count,
        model.hidden_neuron_count,
        model.vocabulary_size,
        model.vocabulary_size,
        model.parameters.iter().map(Vec::len).sum::<usize>()
    );
    // 4. Необязательный опыт с lr: отдельная копия, основной запуск не меняется.
    if false {
        // true — дополнительно построить график выбора learning rate
        let learning_rate_curve = {
            let mut learning_rate_trial_model = model.clone();
            let mut batch_random_generator = RandomGenerator::new(123);
            let mut learning_rate_curve = Vec::new();

            let mut loss_csv_file = std::fs::File::create(output_directory.join("lr_sweep.csv"))?;

            std::io::Write::write_fmt(&mut loss_csv_file, format_args!("log10_lr,lr,loss\n"))?;
            for step_index in 0..1000 {
                let learning_rate_exponent = -3.0 + 3.0 * step_index as f32 / 999.0;
                let learning_rate = 10.0f32.powf(learning_rate_exponent);
                let loss = learning_rate_trial_model.step(
                    &(0..32)
                        .map(|_| {
                            training_examples[batch_random_generator.index(training_examples.len())]
                        })
                        .collect::<Vec<_>>(),
                    learning_rate,
                );
                std::io::Write::write_fmt(
                    &mut loss_csv_file,
                    format_args!("{learning_rate_exponent},{learning_rate},{loss}\n"),
                )?;
                learning_rate_curve.push((learning_rate_exponent, loss));
            }

            learning_rate_curve
        };
        plot(
            output_directory.join("lr_sweep.svg"),
            &learning_rate_curve,
            None,
            "Learning-rate sweep: x=log10(lr), y=loss",
        )?;
    }
    // 5. SGD: выборка с возвращением, средняя CE, очистка градиентов каждый шаг.
    let initial_validation_loss = model.loss(&validation_examples);
    println!("До обучения dev loss={initial_validation_loss:.6}");
    let (model, training_loss_curve) = {
        let mut model = model;
        let mut batch_random_generator = RandomGenerator::new(2147483647);

        let mut loss_csv_file =
            std::io::BufWriter::new(std::fs::File::create(output_directory.join("loss.csv"))?);

        std::io::Write::write_fmt(
            &mut loss_csv_file,
            format_args!("step,learning_rate,loss,log10_loss\n"),
        )?;
        let mut training_loss_curve = Vec::with_capacity(training_steps);

        let training_started_at = std::time::Instant::now();
        for step_index in 0..training_steps {
            let learning_rate = if step_index < 100_000 { 0.1 } else { 0.01 };
            let loss = model.step(
                &(0..32)
                    .map(|_| {
                        training_examples[batch_random_generator.index(training_examples.len())]
                    })
                    .collect::<Vec<_>>(),
                learning_rate,
            );
            if !loss.is_finite() {
                return Err(format!("loss не конечен на шаге {step_index}").into());
            }
            std::io::Write::write_fmt(
                &mut loss_csv_file,
                format_args!("{step_index},{learning_rate},{loss},{}\n", loss.log10()),
            )?;
            training_loss_curve.push((step_index as f32, loss.log10()));
            if step_index % 10_000 == 0 || step_index + 1 == training_steps {
                println!(
                    "step={step_index:6} lr={learning_rate} loss={loss:.4} elapsed={:.1}s",
                    training_started_at.elapsed().as_secs_f32()
                );
            }
        }
        std::io::Write::flush(&mut loss_csv_file)?;

        (model, training_loss_curve)
    };
    // 6. Оценка без обновлений и графики, как в notebook.
    // Test — только окончательная оценка, не выбор lr/архитектуры.
    let metrics_text = format!(
        "steps={training_steps}\nparameters={parameter_count}\ninitial_dev_loss={initial_validation_loss:.6}\ntrain_loss={training_loss:.6}\ndev_loss={validation_loss:.6}\ntest_loss={test_loss:.6}\n",
        parameter_count = model.parameters.iter().map(Vec::len).sum::<usize>(),
        training_loss = model.loss(&training_examples),
        validation_loss = model.loss(&validation_examples),
        test_loss = model.loss(&test_examples),
    );
    print!("{metrics_text}");

    std::fs::write(output_directory.join("metrics.txt"), metrics_text)?;
    plot(
        output_directory.join("loss.svg"),
        &training_loss_curve,
        None,
        "Training: x=step, y=log10(minibatch loss)",
    )?;
    let [character_embeddings, _, _, _, _] = &model.parameters;
    let embedding_coordinates: Vec<_> = (0..vocabulary.len())
        .map(|character_index| {
            (
                character_embeddings[character_index * model.embedding_dimensions],
                character_embeddings[character_index * model.embedding_dimensions + 1],
            )
        })
        .collect();
    plot(
        output_directory.join("embeddings.svg"),
        &embedding_coordinates,
        Some(&vocabulary),
        "C: x=dimension 0, y=dimension 1 (of 10)",
    )?;
    {
        let mut embeddings_csv_file =
            std::fs::File::create(output_directory.join("embeddings.csv"))?;
        std::io::Write::write_fmt(&mut embeddings_csv_file, format_args!("character,c0,c1\n"))?;
        for (character, (first_coordinate, second_coordinate)) in
            vocabulary.iter().zip(embedding_coordinates)
        {
            std::io::Write::write_fmt(
                &mut embeddings_csv_file,
                format_args!("{character},{first_coordinate},{second_coordinate}\n"),
            )?;
        }
    }
    // 7. Авторегрессионная генерация с собственным фиксированным seed.
    let generated_names = {
        let mut sampling_random_generator = RandomGenerator::new(2147483647 + 10);
        let mut generated_names = String::new();
        for _ in 0..20 {
            let name = model.sample(&vocabulary, &mut sampling_random_generator);
            println!("{name}");
            generated_names.push_str(&name);
            generated_names.push('\n');
        }

        generated_names
    };
    std::fs::write(output_directory.join("samples.txt"), generated_names)?;
    println!("Артефакты: {}", output_directory.display());
    Ok(())
}

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
    fn normal(&mut self) -> f32 {
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

#[derive(Clone)]
pub struct Model {
    pub vocabulary_size: usize,
    // Сколько обучаемых координат описывает одну букву.
    pub embedding_dimensions: usize,
    pub hidden_neuron_count: usize,
    // Пять тензоров в порядке notebook:
    // C — координаты букв, W1 — веса вход → скрытый слой, b1 — смещения скрытого слоя,
    // W2 — веса скрытый слой → выход, b2 — смещения выходного слоя.
    pub parameters: [Vec<f32>; 5],
}
impl Model {
    pub fn new(
        vocabulary_size: usize,
        embedding_dimensions: usize,
        hidden_neuron_count: usize,
        random_generator: &mut RandomGenerator,
    ) -> Self {
        Self {
            vocabulary_size,
            embedding_dimensions,
            hidden_neuron_count,
            parameters: [
                vocabulary_size * embedding_dimensions,
                3 * embedding_dimensions * hidden_neuron_count,
                hidden_neuron_count,
                hidden_neuron_count * vocabulary_size,
                vocabulary_size,
            ]
            .map(|tensor_size| {
                (0..tensor_size)
                    .map(|_| random_generator.normal())
                    .collect()
            }),
        }
    }
    pub fn forward(&self, context_character_indices: [usize; 3]) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        let [
            character_embeddings,
            input_to_hidden_weights,
            hidden_biases,
            hidden_to_output_weights,
            output_biases,
        ] = &self.parameters;
        // Берём координаты трёх букв и объединяем в один входной вектор.
        // Его длина: 3 * embedding_dimensions.
        let flattened_embeddings: Vec<_> = context_character_indices
            .iter()
            .flat_map(|&character_index| {
                character_embeddings[character_index * self.embedding_dimensions
                    ..(character_index + 1) * self.embedding_dimensions]
                    .iter()
                    .copied()
            })
            .collect();
        let hidden_activations = {
            let mut hidden_activations = hidden_biases.clone();
            for (input_index, &input_value) in flattened_embeddings.iter().enumerate() {
                for (hidden_activation, &connection_weight) in hidden_activations.iter_mut().zip(
                    &input_to_hidden_weights[input_index * self.hidden_neuron_count
                        ..(input_index + 1) * self.hidden_neuron_count],
                ) {
                    *hidden_activation += input_value * connection_weight;
                }
            }
            for hidden_activation in &mut hidden_activations {
                *hidden_activation = hidden_activation.tanh();
            }

            hidden_activations
        };
        let character_scores = {
            let mut character_scores = output_biases.clone();
            for (hidden_neuron_index, &hidden_activation) in hidden_activations.iter().enumerate() {
                for (character_score, &connection_weight) in character_scores.iter_mut().zip(
                    &hidden_to_output_weights[hidden_neuron_index * self.vocabulary_size
                        ..(hidden_neuron_index + 1) * self.vocabulary_size],
                ) {
                    *character_score += hidden_activation * connection_weight;
                }
            }

            character_scores
        };
        (flattened_embeddings, hidden_activations, character_scores)
    }
    pub fn loss(&self, examples: &[TrainingExample]) -> f32 {
        assert!(!examples.is_empty());
        examples
            .iter()
            .map(|example| {
                cross_entropy(
                    &self.forward(example.context_character_indices).2,
                    example.target_character_index,
                )
                .0 as f64
            })
            .sum::<f64>() as f32
            / examples.len() as f32
    }
    pub fn gradients(&self, training_batch: &[TrainingExample]) -> (f32, [Vec<f32>; 5]) {
        assert!(!training_batch.is_empty());
        let mut parameter_gradients = self
            .parameters
            .each_ref()
            .map(|parameter_tensor| vec![0.0; parameter_tensor.len()]);
        let [
            embedding_gradients,
            input_to_hidden_weight_gradients,
            hidden_bias_gradients,
            hidden_to_output_weight_gradients,
            output_bias_gradients,
        ] = &mut parameter_gradients;
        let [_, input_to_hidden_weights, _, hidden_to_output_weights, _] = &self.parameters;
        let mut total_loss = 0.0;
        for example in training_batch {
            let (flattened_embeddings, hidden_activations, character_scores) =
                self.forward(example.context_character_indices);
            let score_gradients = {
                let (example_loss, mut score_gradients) =
                    cross_entropy(&character_scores, example.target_character_index);
                total_loss += example_loss;
                // Вероятности превращаем в производные ошибки по выходным оценкам.
                // Для правильной буквы вычитаем 1, затем усредняем по мини-батчу.
                score_gradients[example.target_character_index] -= 1.0;
                for score_gradient in &mut score_gradients {
                    *score_gradient /= training_batch.len() as f32;
                }

                score_gradients
            };
            for (output_bias_gradient, &score_gradient) in
                output_bias_gradients.iter_mut().zip(&score_gradients)
            {
                *output_bias_gradient += score_gradient;
            }
            // Производные ошибки по суммам скрытого слоя ДО применения tanh.
            let hidden_pre_activation_gradients = {
                let mut hidden_pre_activation_gradients = vec![0.0; self.hidden_neuron_count];
                for hidden_neuron_index in 0..self.hidden_neuron_count {
                    for output_character_index in 0..self.vocabulary_size {
                        hidden_to_output_weight_gradients[hidden_neuron_index
                            * self.vocabulary_size
                            + output_character_index] += hidden_activations[hidden_neuron_index]
                            * score_gradients[output_character_index];
                        hidden_pre_activation_gradients[hidden_neuron_index] +=
                            hidden_to_output_weights[hidden_neuron_index * self.vocabulary_size
                                + output_character_index]
                                * score_gradients[output_character_index];
                    }
                    hidden_pre_activation_gradients[hidden_neuron_index] *= 1.0
                        - hidden_activations[hidden_neuron_index]
                            * hidden_activations[hidden_neuron_index]; // производная tanh
                    hidden_bias_gradients[hidden_neuron_index] +=
                        hidden_pre_activation_gradients[hidden_neuron_index];
                }

                hidden_pre_activation_gradients
            };
            for input_coordinate_index in 0..flattened_embeddings.len() {
                let input_coordinate_gradient = {
                    let mut input_coordinate_gradient = 0.0;
                    for hidden_neuron_index in 0..self.hidden_neuron_count {
                        input_to_hidden_weight_gradients[input_coordinate_index
                            * self.hidden_neuron_count
                            + hidden_neuron_index] += flattened_embeddings[input_coordinate_index]
                            * hidden_pre_activation_gradients[hidden_neuron_index];
                        input_coordinate_gradient += input_to_hidden_weights[input_coordinate_index
                            * self.hidden_neuron_count
                            + hidden_neuron_index]
                            * hidden_pre_activation_gradients[hidden_neuron_index];
                    }

                    input_coordinate_gradient
                };
                // Scatter-add: повторяющиеся символы суммируют градиенты одной строки C.
                embedding_gradients[example.context_character_indices
                    [input_coordinate_index / self.embedding_dimensions]
                    * self.embedding_dimensions
                    + input_coordinate_index % self.embedding_dimensions] +=
                    input_coordinate_gradient;
            }
        }
        (
            total_loss / training_batch.len() as f32,
            parameter_gradients,
        )
    }
    pub fn step(&mut self, training_batch: &[TrainingExample], learning_rate: f32) -> f32 {
        let (loss, gradients) = self.gradients(training_batch);
        for (parameter_tensor, gradient_tensor) in self.parameters.iter_mut().zip(gradients) {
            for (parameter, gradient) in parameter_tensor.iter_mut().zip(gradient_tensor) {
                *parameter -= learning_rate * gradient;
            }
        }
        loss
    }
    pub fn sample(&self, vocabulary: &[char], random_generator: &mut RandomGenerator) -> String {
        let mut context_character_indices = [0; 3];
        let mut generated_name = String::new();
        for _ in 0..100 {
            let (_, character_probabilities) =
                cross_entropy(&self.forward(context_character_indices).2, 0);
            let random_probability = random_generator.uniform();
            let sampled_character_index = {
                let mut cumulative_probability = 0.0;
                let mut sampled_character_index = self.vocabulary_size - 1;
                for (character_index, character_probability) in
                    character_probabilities.iter().enumerate()
                {
                    cumulative_probability += character_probability;
                    if random_probability < cumulative_probability {
                        sampled_character_index = character_index;
                        break;
                    }
                }

                sampled_character_index
            };
            generated_name.push(vocabulary[sampled_character_index]);
            if sampled_character_index == 0 {
                return generated_name;
            }
            context_character_indices = [
                context_character_indices[1],
                context_character_indices[2],
                sampled_character_index,
            ];
        }
        generated_name.push_str(" [лимит 100 символов]");
        generated_name
    }
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
