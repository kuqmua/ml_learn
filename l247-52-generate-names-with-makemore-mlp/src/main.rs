fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_started_at: std::time::Instant = std::time::Instant::now();
    // Обёртка над срезом, для которого уже проверено наличие хотя бы одного элемента.
    #[derive(Debug)]
    struct NonEmptySlice<'a, T>(
        // Заимствованный срез: проверка не копирует его элементы.
        &'a [T],
    );
    impl<T> Copy for NonEmptySlice<'_, T> {}
    impl<T> Clone for NonEmptySlice<'_, T> {
        // Копирует обёртку со ссылкой; элементы исходного среза не клонируются.
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<'a, T> TryFrom<&'a [T]> for NonEmptySlice<'a, T> {
        type Error = &'static str;
        // Отклоняет пустой срез и оборачивает непустой без копирования элементов.
        fn try_from(values: &'a [T]) -> Result<Self, Self::Error> {
            if values.is_empty() {
                Err("набор обучающих примеров не должен быть пустым")
            } else {
                Ok(Self(values))
            }
        }
    }
    impl<T> std::ops::Deref for NonEmptySlice<'_, T> {
        type Target = [T];
        // Возвращает исходный срез, чтобы обращаться к его элементам и методам.
        fn deref(&self) -> &Self::Target {
            self.0
        }
    }

    // Генератор псевдослучайных чисел: одинаковый seed даёт одинаковую последовательность.
    #[derive(Clone)]
    struct RandomGenerator(
        // Текущее внутреннее состояние, изменяемое при получении очередного числа.
        u64,
    );
    impl RandomGenerator {
        // Создаёт генератор с заданным seed для воспроизводимой последовательности.
        fn new(seed: u64) -> Self {
            Self(seed)
        }
        // Обновляет состояние генератора и возвращает следующее псевдослучайное u64.
        fn next(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
            let mixed_bits: u64 = self.0;
            let mixed_bits: u64 =
                (mixed_bits ^ (mixed_bits >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            let mixed_bits: u64 =
                (mixed_bits ^ (mixed_bits >> 27)).wrapping_mul(0x94d049bb133111eb);
            mixed_bits ^ (mixed_bits >> 31)
        }
        // Получает равномерное псевдослучайное число от 0 до 1 для выборки и инициализации.
        fn uniform(&mut self) -> f32 {
            ((self.next() >> 40) as f32 + 0.5) / 16777216.0
        }
        // Выбирает индекс от 0 до upper_bound (не включая границу) без смещения от остатка; upper_bound > 0.
        fn index(&mut self, upper_bound: usize) -> usize {
            let upper_bound: u64 = upper_bound as u64;
            loop {
                let random_bits: u64 = self.next();
                if random_bits >= upper_bound.wrapping_neg() % upper_bound {
                    return (random_bits % upper_bound) as usize;
                }
            }
        }
    }

    // Индекс символа, проверенный относительно размера словаря при создании.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct CharacterIndex(
        // Позиция символа в словаре; создаётся методами Vocabulary, а читается через .0.
        usize,
    );

    // Словарь символов: точка под индексом 0, затем уникальные буквы a–z по алфавиту.
    struct Vocabulary(
        // Символы в порядке их индексов; создаются методом new.
        Vec<char>,
    );
    impl Vocabulary {
        // Собирает уникальные строчные латинские буквы текста и добавляет точку в начало.
        fn new(text: &str) -> Self {
            let mut characters: Vec<char> = text
                .chars()
                .filter(|character| character.is_ascii_lowercase())
                .collect();
            characters.sort_unstable();
            characters.dedup();
            characters.insert(0, '.');
            Self(characters)
        }
        // Возвращает количество символов, включая точку.
        fn len(&self) -> usize {
            self.0.len()
        }
        // Проверяет границы и возвращает индекс, пригодный для этого словаря.
        fn character_index(&self, value: usize) -> Result<CharacterIndex, &'static str> {
            self.0
                .get(value)
                .map(|_| CharacterIndex(value))
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
        // Находит индекс символа; для отсутствующего символа возвращает ошибку.
        fn index_of(&self, character: char) -> Result<CharacterIndex, &'static str> {
            self.0
                .iter()
                .position(|&candidate| candidate == character)
                .map(CharacterIndex)
                .ok_or("символ отсутствует в словаре")
        }
        // Возвращает символ по индексу; проверяет границы и при индексе из другого словаря.
        fn character(&self, index: CharacterIndex) -> Result<char, &'static str> {
            self.0
                .get(index.0)
                .copied()
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
    }

    // Три проверенных индекса предыдущих символов; индекс 0 — точка и заполнение начала.
    type ContextCharacterIndices = [CharacterIndex; 3];

    // Один учебный пример: по трём предыдущим символам предсказываем следующий.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct TrainingExample {
        // Индексы трёх символов в словаре; 0 означает точку, в том числе заполнение начала.
        context_character_indices: ContextCharacterIndices,
        // Индекс правильного следующего символа; 0 также обозначает конец имени.
        target_character_index: CharacterIndex,
    }

    // MLP: векторы трёх символов → скрытый слой с tanh → оценки следующего символа.
    #[derive(Clone)]
    struct Model {
        // Число символов в словаре, включая точку; столько же оценок выдаёт модель.
        vocabulary_size: usize,
        // Сколько обучаемых координат описывает одну букву.
        embedding_dimensions: usize,
        // Число нейронов скрытого слоя.
        hidden_neuron_count: usize,
        // Пять тензоров в порядке notebook:
        // C — координаты букв, W1 — веса вход → скрытый слой, b1 — смещения скрытого слоя,
        // W2 — веса скрытый слой → выход, b2 — смещения выходного слоя.
        parameters: [Vec<f32>; 5],
    }
    impl Model {
        // Возвращает (cross-entropy для правильного символа, вероятности всех символов), устойчиво вычисляя softmax.
        fn cross_entropy(
            character_scores: &[f32],
            target_character_index: CharacterIndex,
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
                (maximum_score - character_scores[target_character_index.0]) + exponential_sum.ln(),
                character_probabilities,
            )
        }

        // Для одного контекста возвращает (объединённые эмбеддинги, активации tanh, оценки символов до softmax).
        fn forward(
            &self,
            context_character_indices: ContextCharacterIndices,
        ) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
            let [
                character_embeddings,
                input_to_hidden_weights,
                hidden_biases,
                hidden_to_output_weights,
                output_biases,
            ]: &[Vec<f32>; 5] = &self.parameters;
            // Берём координаты трёх букв и объединяем в один входной вектор.
            // Его длина: 3 * embedding_dimensions.
            let flattened_embeddings: Vec<f32> = context_character_indices
                .iter()
                .flat_map(|&character_index| {
                    character_embeddings[character_index.0 * self.embedding_dimensions
                        ..(character_index.0 + 1) * self.embedding_dimensions]
                        .iter()
                        .copied()
                })
                .collect();
            let hidden_activations: Vec<f32> = {
                let mut hidden_activations: Vec<f32> = hidden_biases.clone();
                for (input_index, &input_value) in flattened_embeddings.iter().enumerate() {
                    for (hidden_activation, &connection_weight) in
                        hidden_activations.iter_mut().zip(
                            &input_to_hidden_weights[input_index * self.hidden_neuron_count
                                ..(input_index + 1) * self.hidden_neuron_count],
                        )
                    {
                        *hidden_activation += input_value * connection_weight;
                    }
                }
                for hidden_activation in &mut hidden_activations {
                    *hidden_activation = hidden_activation.tanh();
                }

                hidden_activations
            };
            let character_scores: Vec<f32> = {
                let mut character_scores: Vec<f32> = output_biases.clone();
                for (hidden_neuron_index, &hidden_activation) in
                    hidden_activations.iter().enumerate()
                {
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
        // Вычисляет среднюю cross-entropy по непустому набору примеров, не изменяя параметры модели.
        fn loss(&self, examples: NonEmptySlice<'_, TrainingExample>) -> f32 {
            examples
                .iter()
                .map(|example| {
                    Model::cross_entropy(
                        &self.forward(example.context_character_indices).2,
                        example.target_character_index,
                    )
                    .0 as f64
                })
                .sum::<f64>() as f32
                / examples.len() as f32
        }
    }

    // Число шагов обучения задаётся ниже при создании TrainingSteps.
    let training_steps: std::num::NonZeroUsize = {
        // Проверенное положительное число шагов обучения.
        #[derive(Clone, Copy, Debug)]
        struct TrainingSteps(
            // Количество обновлений параметров; ноль исключён самим типом.
            std::num::NonZeroUsize,
        );
        impl TryFrom<usize> for TrainingSteps {
            type Error = &'static str;
            // Проверяет, что число шагов положительно; для нуля возвращает ошибку.
            fn try_from(value: usize) -> Result<Self, Self::Error> {
                std::num::NonZeroUsize::new(value)
                    .map(Self)
                    .ok_or("число шагов должно быть положительным")
            }
        }

        TrainingSteps::try_from(200_000usize)?.0
    };
    let mut files: std::collections::BTreeMap<String, std::fs::File> = {
        let output_directory: std::path::PathBuf =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("output");
        std::fs::create_dir_all(&output_directory)?;
        let mut files: std::collections::BTreeMap<String, std::fs::File> =
            std::collections::BTreeMap::new();
        for name in [
            "loss.csv",
            "loss.svg",
            "metrics.txt",
            "embeddings.svg",
            "embeddings.csv",
            "samples.txt",
        ] {
            files.insert(
                name.to_owned(),
                std::fs::File::create(output_directory.join(name))?,
            );
        }
        files
    };
    let (vocabulary, training_examples, validation_examples, test_examples): (
        Vocabulary,
        Vec<TrainingExample>,
        Vec<TrainingExample>,
        Vec<TrainingExample>,
    ) = {
        // Проверенный непустой список имён из строчных латинских букв.
        #[derive(Debug)]
        struct Names<'a>(
            // Ссылки на строки исходного текста: сами имена не копируются.
            Vec<&'a str>,
        );
        impl<'a> TryFrom<&'a str> for Names<'a> {
            type Error = &'static str;
            // Разбивает текст на имена; возвращает ошибку для пустого списка, пустых имён и символов вне a–z.
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
            // Даёт доступ к проверенным именам как к неизменяемому срезу, без копирования строк.
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // 1. Данные и словарь: 0 — начало/конец имени, a..z — 1..26.
        let names_text: &str = include_str!("../data/names.txt");
        let names: Names<'_> = Names::try_from(names_text)?;
        let vocabulary: Vocabulary = Vocabulary::new(names_text);

        // Строит примеры по именам, заимствуя словарь из окружающей области.
        let dataset = |names: &[&str]| -> Result<Vec<TrainingExample>, &'static str> {
            let mut examples: Vec<TrainingExample> = Vec::new();
            for name in names {
                let mut context_character_indices: ContextCharacterIndices =
                    [vocabulary.character_index(0)?; 3];

                for next_character in name.chars().chain(std::iter::once('.')) {
                    let target_character_index: CharacterIndex =
                        vocabulary.index_of(next_character)?;
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
            Ok(examples)
        };

        println!(
            "Имена: {}, первые восемь: {:?}",
            names.len(),
            &names[..names.len().min(8)]
        );
        println!("Словарь: {:?}", vocabulary.0);
        let all_examples: Vec<TrainingExample> = dataset(&names)?;
        println!(
            "Все X: ({},3), Y: ({},)",
            all_examples.len(),
            all_examples.len()
        );
        for example in dataset(&names[..1])? {
            println!(
                "{} -> {}",
                example
                    .context_character_indices
                    .map(|character_index| vocabulary.character(character_index))
                    .into_iter()
                    .collect::<Result<String, _>>()?,
                vocabulary.character(example.target_character_index)?
            );
        }
        // 2. Перемешиваем имена и делим их на train/dev/test (80/10/10).
        // Контексты каждого набора строятся отдельно, чтобы имена не пересекались.
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
        let training_names_end: usize = names.len() * 8 / 10;
        let validation_names_end: usize = names.len() * 9 / 10;
        let training_examples: Vec<TrainingExample> = dataset(&names[..training_names_end])?;
        let validation_examples: Vec<TrainingExample> =
            dataset(&names[training_names_end..validation_names_end])?;
        let test_examples: Vec<TrainingExample> = dataset(&names[validation_names_end..])?;
        println!(
            "Имена train/dev/test: {}/{}/{}; примеры: {}/{}/{}",
            training_names_end,
            validation_names_end - training_names_end,
            names.len() - validation_names_end,
            training_examples.len(),
            validation_examples.len(),
            test_examples.len()
        );

        (
            vocabulary,
            training_examples,
            validation_examples,
            test_examples,
        )
    };
    // Строит SVG-график и записывает его в заранее созданный файл.
    fn plot(
        mut output_file: std::fs::File,
        data_points: &[(f32, f32)],
        character_labels: Option<&[char]>,
        title: &str,
    ) -> std::io::Result<()> {
        let minimum_horizontal_value: f32 = data_points
            .iter()
            .map(|data_point| data_point.0)
            .fold(f32::INFINITY, f32::min);
        let maximum_horizontal_value: f32 = data_points
            .iter()
            .map(|data_point| data_point.0)
            .fold(f32::NEG_INFINITY, f32::max);
        let minimum_vertical_value: f32 = data_points
            .iter()
            .map(|data_point| data_point.1)
            .fold(f32::INFINITY, f32::min);
        let maximum_vertical_value: f32 = data_points
            .iter()
            .map(|data_point| data_point.1)
            .fold(f32::NEG_INFINITY, f32::max);
        let to_pixel_coordinates = |data_point: &(f32, f32)| -> (f32, f32) {
            (
                65.0 + 680.0 * (data_point.0 - minimum_horizontal_value)
                    / (maximum_horizontal_value - minimum_horizontal_value).max(1e-6),
                535.0
                    - 470.0 * (data_point.1 - minimum_vertical_value)
                        / (maximum_vertical_value - minimum_vertical_value).max(1e-6),
            )
        };
        let svg_document: String = {
            let mut svg_document: String = format!(
                "<svg xmlns='http://www.w3.org/2000/svg' width='800' height='600' viewBox='0 0 800 600'><rect width='800' height='600' fill='white'/><g font-family='sans-serif' fill='#172554'><text x='65' y='28'>{title}</text><path d='M65 55V535H755' fill='none' stroke='#64748b'/><text x='65' y='565'>{minimum_horizontal_value:.3}</text><text x='690' y='565'>{maximum_horizontal_value:.3}</text><text x='5' y='535'>{minimum_vertical_value:.3}</text><text x='5' y='65'>{maximum_vertical_value:.3}</text>"
            );
            if let Some(character_labels) = character_labels {
                for (data_point, character) in data_points.iter().zip(character_labels) {
                    let (horizontal_pixel, vertical_pixel): (f32, f32) =
                        to_pixel_coordinates(data_point);
                    svg_document.push_str(&format!("<circle cx='{horizontal_pixel}' cy='{vertical_pixel}' r='12' fill='#2563eb'/><text x='{horizontal_pixel}' y='{}' text-anchor='middle' fill='white'>{character}</text>", vertical_pixel+4.0));
                }
            } else {
                svg_document
                    .push_str("<polyline fill='none' stroke='#2563eb' stroke-width='0.7' points='");
                for data_point in data_points {
                    let (horizontal_pixel, vertical_pixel): (f32, f32) =
                        to_pixel_coordinates(data_point);
                    svg_document.push_str(&format!("{horizontal_pixel:.2},{vertical_pixel:.2} "));
                }
                svg_document.push_str("'/>");
            }
            svg_document.push_str("</g></svg>");

            svg_document
        };
        std::io::Write::write_all(&mut output_file, svg_document.as_bytes())
    }
    let model: Model = {
        let vocabulary_size: usize = vocabulary.len();
        let embedding_dimensions: usize = 10;
        let hidden_neuron_count: usize = 200;
        let mut random_generator: RandomGenerator = RandomGenerator::new(2147483647);
        // Создаём пять тензоров с независимыми начальными значениями из N(0, 1).

        Model {
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
                    .map(|_| {
                        (-2.0 * random_generator.uniform().ln()).sqrt()
                            * (std::f32::consts::TAU * random_generator.uniform()).cos()
                    })
                    .collect()
            }),
        }
    };
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
    let initial_validation_loss: f32 =
        model.loss(NonEmptySlice::try_from(validation_examples.as_slice())?);
    println!("До обучения dev loss={initial_validation_loss:.6}");
    // 3. SGD: выборка с возвращением, средняя cross-entropy и новые нулевые градиенты на каждом шаге.
    let (model, training_loss_curve): (Model, Vec<(f32, f32)>) = {
        let mut model: Model = model;
        let mut batch_random_generator: RandomGenerator = RandomGenerator::new(2147483647);

        let mut loss_csv_file: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(
            files
                .remove("loss.csv")
                .ok_or("файл отсутствует в подготовленном наборе")?,
        );

        std::io::Write::write_fmt(
            &mut loss_csv_file,
            format_args!("step,learning_rate,loss,log10_loss\n"),
        )?;
        let mut training_loss_curve: Vec<(f32, f32)> = Vec::with_capacity(training_steps.get());

        let training_started_at: std::time::Instant = std::time::Instant::now();
        for step_index in 0..training_steps.get() {
            let iteration_started_at: std::time::Instant = std::time::Instant::now();
            let learning_rate: f32 = if step_index < 100_000 { 0.1 } else { 0.01 };
            let loss: f32 = {
                let (loss, gradients): (f32, [Vec<f32>; 5]) = {
                    let batch: Vec<TrainingExample> = (0..32)
                        .map(|_| {
                            training_examples[batch_random_generator.index(training_examples.len())]
                        })
                        .collect();
                    let training_batch: NonEmptySlice<'_, TrainingExample> =
                        NonEmptySlice::try_from(batch.as_slice())?;

                    let mut parameter_gradients: [Vec<f32>; 5] = model
                        .parameters
                        .each_ref()
                        .map(|parameter_tensor| vec![0.0; parameter_tensor.len()]);
                    let [
                        embedding_gradients,
                        input_to_hidden_weight_gradients,
                        hidden_bias_gradients,
                        hidden_to_output_weight_gradients,
                        output_bias_gradients,
                    ]: &mut [Vec<f32>; 5] = &mut parameter_gradients;
                    let [_, input_to_hidden_weights, _, hidden_to_output_weights, _]: &[Vec<f32>;
                         5] = &model.parameters;
                    let mut total_loss: f32 = 0.0;
                    for example in training_batch.iter() {
                        let (flattened_embeddings, hidden_activations, character_scores): (
                            Vec<f32>,
                            Vec<f32>,
                            Vec<f32>,
                        ) = model.forward(example.context_character_indices);
                        let score_gradients: Vec<f32> = {
                            let (example_loss, mut score_gradients): (f32, Vec<f32>) =
                                Model::cross_entropy(
                                    &character_scores,
                                    example.target_character_index,
                                );
                            total_loss += example_loss;
                            // Вероятности превращаем в производные ошибки по выходным оценкам.
                            // Для правильной буквы вычитаем 1, затем усредняем по мини-батчу.
                            score_gradients[example.target_character_index.0] -= 1.0;
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
                        let hidden_pre_activation_gradients: Vec<f32> = {
                            let mut hidden_pre_activation_gradients: Vec<f32> =
                                vec![0.0; model.hidden_neuron_count];
                            for hidden_neuron_index in 0..model.hidden_neuron_count {
                                for output_character_index in 0..model.vocabulary_size {
                                    hidden_to_output_weight_gradients[hidden_neuron_index
                                        * model.vocabulary_size
                                        + output_character_index] += hidden_activations
                                        [hidden_neuron_index]
                                        * score_gradients[output_character_index];
                                    hidden_pre_activation_gradients[hidden_neuron_index] +=
                                        hidden_to_output_weights[hidden_neuron_index
                                            * model.vocabulary_size
                                            + output_character_index]
                                            * score_gradients[output_character_index];
                                }
                                // Производная tanh равна 1 - tanh².
                                hidden_pre_activation_gradients[hidden_neuron_index] *= 1.0
                                    - hidden_activations[hidden_neuron_index]
                                        * hidden_activations[hidden_neuron_index];
                                hidden_bias_gradients[hidden_neuron_index] +=
                                    hidden_pre_activation_gradients[hidden_neuron_index];
                            }

                            hidden_pre_activation_gradients
                        };
                        for input_coordinate_index in 0..flattened_embeddings.len() {
                            let input_coordinate_gradient: f32 = {
                                let mut input_coordinate_gradient: f32 = 0.0;
                                for hidden_neuron_index in 0..model.hidden_neuron_count {
                                    input_to_hidden_weight_gradients[input_coordinate_index
                                        * model.hidden_neuron_count
                                        + hidden_neuron_index] += flattened_embeddings
                                        [input_coordinate_index]
                                        * hidden_pre_activation_gradients[hidden_neuron_index];
                                    input_coordinate_gradient += input_to_hidden_weights
                                        [input_coordinate_index * model.hidden_neuron_count
                                            + hidden_neuron_index]
                                        * hidden_pre_activation_gradients[hidden_neuron_index];
                                }

                                input_coordinate_gradient
                            };
                            // Scatter-add: повторяющиеся символы суммируют градиенты одной строки C.
                            embedding_gradients[example.context_character_indices
                                [input_coordinate_index / model.embedding_dimensions]
                                .0
                                * model.embedding_dimensions
                                + input_coordinate_index % model.embedding_dimensions] +=
                                input_coordinate_gradient;
                        }
                    }
                    (
                        total_loss / training_batch.len() as f32,
                        parameter_gradients,
                    )
                };
                for (parameter_tensor, gradient_tensor) in
                    model.parameters.iter_mut().zip(gradients)
                {
                    for (parameter, gradient) in parameter_tensor.iter_mut().zip(gradient_tensor) {
                        *parameter -= learning_rate * gradient;
                    }
                }
                loss
            };
            if !loss.is_finite() {
                return Err(format!("loss не конечен на шаге {step_index}").into());
            }
            std::io::Write::write_fmt(
                &mut loss_csv_file,
                format_args!("{step_index},{learning_rate},{loss},{}\n", loss.log10()),
            )?;
            training_loss_curve.push((step_index as f32, loss.log10()));
            if step_index % 10_000 == 0 || step_index + 1 == training_steps.get() {
                println!(
                    "step={step_index:6} lr={learning_rate} loss={loss:.4} elapsed={:.1}s iteration_ms={:.3}",
                    training_started_at.elapsed().as_secs_f32(),
                    iteration_started_at.elapsed().as_secs_f64() * 1000.0
                );
            }
        }
        std::io::Write::flush(&mut loss_csv_file)?;

        (model, training_loss_curve)
    };
    // 4. Оценка без обновлений и графики, как в notebook.
    // Test — только окончательная оценка, не выбор lr/архитектуры.
    {
        let metrics_text: String = format!(
            "steps={training_steps}\nparameters={parameter_count}\ninitial_dev_loss={initial_validation_loss:.6}\ntrain_loss={training_loss:.6}\ndev_loss={validation_loss:.6}\ntest_loss={test_loss:.6}\n",
            parameter_count = model.parameters.iter().map(Vec::len).sum::<usize>(),
            training_loss = model.loss(NonEmptySlice::try_from(training_examples.as_slice())?),
            validation_loss = model.loss(NonEmptySlice::try_from(validation_examples.as_slice())?),
            test_loss = model.loss(NonEmptySlice::try_from(test_examples.as_slice())?),
        );
        print!("{metrics_text}");

        std::io::Write::write_all(
            &mut files
                .remove("metrics.txt")
                .ok_or("файл отсутствует в подготовленном наборе")?,
            (metrics_text).as_bytes(),
        )?;
    }
    plot(
        files
            .remove("loss.svg")
            .ok_or("файл отсутствует в подготовленном наборе")?,
        &training_loss_curve,
        None,
        "Training: x=step, y=log10(minibatch loss)",
    )?;
    let [character_embeddings, _, _, _, _]: &[Vec<f32>; 5] = &model.parameters;
    let embedding_coordinates: Vec<(f32, f32)> = (0..vocabulary.len())
        .map(|character_index| {
            (
                character_embeddings[character_index * model.embedding_dimensions],
                character_embeddings[character_index * model.embedding_dimensions + 1],
            )
        })
        .collect();
    plot(
        files
            .remove("embeddings.svg")
            .ok_or("файл отсутствует в подготовленном наборе")?,
        &embedding_coordinates,
        Some(&vocabulary.0),
        "C: x=dimension 0, y=dimension 1 (of 10)",
    )?;
    {
        let mut embeddings_csv_file: std::fs::File = files
            .remove("embeddings.csv")
            .ok_or("файл отсутствует в подготовленном наборе")?;
        std::io::Write::write_fmt(&mut embeddings_csv_file, format_args!("character,c0,c1\n"))?;
        for (character, (first_coordinate, second_coordinate)) in
            vocabulary.0.iter().zip(embedding_coordinates)
        {
            std::io::Write::write_fmt(
                &mut embeddings_csv_file,
                format_args!("{character},{first_coordinate},{second_coordinate}\n"),
            )?;
        }
    }
    // 5. Авторегрессионная генерация с собственным фиксированным seed.
    let generated_names: String = {
        let mut sampling_random_generator: RandomGenerator = RandomGenerator::new(2147483647 + 10);
        let mut generated_names: String = String::new();
        for _ in 0..20 {
            let name: String = 'sample: {
                let mut context_character_indices: ContextCharacterIndices =
                    [vocabulary.character_index(0)?; 3];
                let mut generated_name: String = String::new();
                for _ in 0..100 {
                    let (_, character_probabilities): (f32, Vec<f32>) = Model::cross_entropy(
                        &model.forward(context_character_indices).2,
                        vocabulary.character_index(0)?,
                    );
                    let random_probability: f32 = sampling_random_generator.uniform();
                    let sampled_character_index: usize = {
                        let mut cumulative_probability: f32 = 0.0;
                        let mut sampled_character_index: usize = model.vocabulary_size - 1;
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
                    let sampled_character_index: CharacterIndex =
                        vocabulary.character_index(sampled_character_index)?;
                    generated_name.push(vocabulary.character(sampled_character_index)?);
                    if sampled_character_index.0 == 0 {
                        break 'sample generated_name;
                    }
                    context_character_indices = [
                        context_character_indices[1],
                        context_character_indices[2],
                        sampled_character_index,
                    ];
                }
                generated_name.push_str(" [лимит 100 символов]");
                generated_name
            };
            println!("{name}");
            generated_names.push_str(&name);
            generated_names.push('\n');
        }

        generated_names
    };
    std::io::Write::write_all(
        &mut files
            .remove("samples.txt")
            .ok_or("файл отсутствует в подготовленном наборе")?,
        (generated_names).as_bytes(),
    )?;
    println!(
        "Артефакты: {}",
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("output")
            .display()
    );
    println!(
        "Общее время работы программы: {:.2} с",
        program_started_at.elapsed().as_secs_f64()
    );
    Ok(())
}
