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
        // Масштабирует старшие 24 случайных бита в f32 для выборки и инициализации; округление может дать 1.
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

    // Позиция символа в Vocabulary; методы словаря создают её с проверкой границ.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct VocabularyCharacterIndex(
        // Числовая позиция; .0 нужен для тензоров. Сам индекс не хранит, какому Vocabulary принадлежит.
        usize,
    );

    // Словарь символов: точка под индексом 0, затем уникальные буквы a–z по алфавиту.
    struct Vocabulary(
        // Символы в порядке их индексов; создаются методом new.
        Vec<char>,
    );
    impl Vocabulary {
        // Отбирает из текста буквы a–z, сортирует, удаляет повторы и добавляет точку под индексом 0.
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
        // Возвращает индекс точки: она обозначает границу имени и заполняет начальный контекст.
        // Метод new всегда помещает точку под индексом 0.
        fn dot_index(&self) -> VocabularyCharacterIndex {
            VocabularyCharacterIndex(0)
        }
        // Проверяет value < self.len() и оборачивает числовую позицию в VocabularyCharacterIndex.
        fn character_index(&self, value: usize) -> Result<VocabularyCharacterIndex, &'static str> {
            self.0
                .get(value)
                .map(|_| VocabularyCharacterIndex(value))
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
        // Находит индекс символа; для отсутствующего символа возвращает ошибку.
        fn index_of(&self, character: char) -> Result<VocabularyCharacterIndex, &'static str> {
            self.0
                .iter()
                .position(|&candidate| candidate == character)
                .map(VocabularyCharacterIndex)
                .ok_or("символ отсутствует в словаре")
        }
        // Возвращает символ по числовой позиции, проверяя границы; принадлежность индекса словарю не проверяется.
        fn character(&self, index: VocabularyCharacterIndex) -> Result<char, &'static str> {
            self.0
                .get(index.0)
                .copied()
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
    }

    // Сколько предыдущих символов получает модель; значение должно быть положительным.
    const CONTEXT_LENGTH: usize = 3;
    // Контекст из CONTEXT_LENGTH позиций в Vocabulary; dot_index() заполняет начало имени.
    type VocabularyContextIndices = [VocabularyCharacterIndex; CONTEXT_LENGTH];

    // Обучающий пример в индексах Vocabulary: контекст длины CONTEXT_LENGTH и правильный следующий символ.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct VocabularyTrainingExample {
        // Индексы предыдущих CONTEXT_LENGTH символов в Vocabulary; индекс точки заполняет начало имени.
        vocabulary_context_indices: VocabularyContextIndices,
        // Индекс правильного следующего символа в Vocabulary; индекс точки обозначает конец имени.
        vocabulary_character_index_target: VocabularyCharacterIndex,
    }

    // Фиксированные размеры модели, известные при компиляции.
    const EMBEDDING_DIMENSIONS: usize = 10;
    const HIDDEN_NEURON_COUNT: usize = 200;

    // Параметры MLP: таблица эмбеддингов и два полносвязных слоя, между которыми применяется tanh.
    // Прямой проход и loss — замыкания ниже; размер словаря хранится в локальной переменной vocabulary_size.
    #[derive(Clone)]
    struct Model {
        // C: одна строка из EMBEDDING_DIMENSIONS координат на каждый символ Vocabulary.
        character_embeddings: Vec<[f32; EMBEDDING_DIMENSIONS]>,
        // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, HIDDEN_NEURON_COUNT) — веса первого слоя.
        input_to_hidden_weights: [f32; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS * HIDDEN_NEURON_COUNT],
        // b1: (HIDDEN_NEURON_COUNT,) — смещения скрытых нейронов.
        hidden_biases: [f32; HIDDEN_NEURON_COUNT],
        // W2: (HIDDEN_NEURON_COUNT, vocabulary_size) — веса выходного слоя.
        hidden_to_output_weights: Vec<f32>,
        // b2: (vocabulary_size,) — смещения оценок следующих символов.
        output_biases: Vec<f32>,
    }

    // Производные loss по каждому тензору Model; формы и порядок элементов совпадают с соответствующим полем.
    struct ModelGradients {
        // Градиенты character_embeddings.
        character_embeddings: Vec<[f32; EMBEDDING_DIMENSIONS]>,
        // Градиенты input_to_hidden_weights.
        input_to_hidden_weights: [f32; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS * HIDDEN_NEURON_COUNT],
        // Градиенты hidden_biases.
        hidden_biases: [f32; HIDDEN_NEURON_COUNT],
        // Градиенты hidden_to_output_weights.
        hidden_to_output_weights: Vec<f32>,
        // Градиенты output_biases.
        output_biases: Vec<f32>,
    }

    // Положительное число обновлений параметров; для нуля возвращается ошибка.
    let training_steps: std::num::NonZeroUsize =
        std::num::NonZeroUsize::new(200_000).ok_or("число шагов должно быть положительным")?;
    // Имена выходных файлов, общие для их создания и последующей записи.
    const LOSS_CSV: &str = "loss.csv";
    const LOSS_SVG: &str = "loss.svg";
    const METRICS_TXT: &str = "metrics.txt";
    const EMBEDDINGS_SVG: &str = "embeddings.svg";
    const EMBEDDINGS_CSV: &str = "embeddings.csv";
    const SAMPLES_TXT: &str = "samples.txt";

    let mut files: std::collections::BTreeMap<String, std::fs::File> = {
        let output_directory: std::path::PathBuf =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("output");
        std::fs::create_dir_all(&output_directory)?;
        let mut files: std::collections::BTreeMap<String, std::fs::File> =
            std::collections::BTreeMap::new();
        for name in [
            LOSS_CSV,
            LOSS_SVG,
            METRICS_TXT,
            EMBEDDINGS_SVG,
            EMBEDDINGS_CSV,
            SAMPLES_TXT,
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
        Vec<VocabularyTrainingExample>,
        Vec<VocabularyTrainingExample>,
        Vec<VocabularyTrainingExample>,
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

        // 1. Проверяем записи; повторения сохраняем, чтобы не менять их частоты в наборе.
        let names_text: &str = include_str!("../data/names.txt");
        let names: Names<'_> = Names::try_from(names_text)?;
        println!(
            "Имена: {}, первые восемь: {:?}",
            names.len(),
            &names[..names.len().min(8)]
        );
        // 2. Объединяем одинаковые имена, чтобы их копии не оказались одновременно в train/dev/test.
        // BTreeMap задаёт стабильный порядок групп перед воспроизводимым перемешиванием с seed 42.
        let mut name_groups: Vec<Vec<&str>> = {
            let mut groups: std::collections::BTreeMap<&str, Vec<&str>> =
                std::collections::BTreeMap::new();
            for &name in names.iter() {
                groups.entry(name).or_default().push(name);
            }
            groups.into_values().collect()
        };
        let mut generator: RandomGenerator = RandomGenerator::new(42);
        for current_index in (1..name_groups.len()).rev() {
            name_groups.swap(current_index, generator.index(current_index + 1));
        }
        // Доли 80/10/10 относятся к уникальным именам; число записей зависит от размеров групп.
        let (training_name_groups, remaining_name_groups): (&[Vec<&str>], &[Vec<&str>]) =
            name_groups.split_at(name_groups.len() * 8 / 10);
        let (validation_name_groups, test_name_groups): (&[Vec<&str>], &[Vec<&str>]) =
            remaining_name_groups.split_at(name_groups.len() * 9 / 10 - training_name_groups.len());
        let training_names: Vec<&str> = training_name_groups.iter().flatten().copied().collect();
        let validation_names: Vec<&str> =
            validation_name_groups.iter().flatten().copied().collect();
        let test_names: Vec<&str> = test_name_groups.iter().flatten().copied().collect();
        // Состав Vocabulary определяем только по train. Неизвестный символ в dev/test вернёт ошибку.
        let vocabulary: Vocabulary = Vocabulary::new(&training_names.join("\n"));
        println!("Словарь (только train): {:?}", vocabulary.0);

        // Захватывает Vocabulary: для каждого имени сбрасывает контекст и строит пары «контекст → следующий символ».
        // Добавленная в конец точка учит завершать имя; неизвестный символ возвращает ошибку.
        let dataset = |names: &[&str]| -> Result<Vec<VocabularyTrainingExample>, &'static str> {
            let mut examples: Vec<VocabularyTrainingExample> = Vec::new();
            for name in names {
                let mut vocabulary_context_indices: VocabularyContextIndices =
                    [vocabulary.dot_index(); CONTEXT_LENGTH];
                for next_character in name.chars().chain(std::iter::once('.')) {
                    let vocabulary_character_index_target: VocabularyCharacterIndex =
                        vocabulary.index_of(next_character)?;
                    examples.push(VocabularyTrainingExample {
                        vocabulary_context_indices,
                        vocabulary_character_index_target,
                    });
                    vocabulary_context_indices.rotate_left(1);
                    vocabulary_context_indices[CONTEXT_LENGTH - 1] =
                        vocabulary_character_index_target;
                }
            }
            Ok(examples)
        };
        let training_examples: Vec<VocabularyTrainingExample> = dataset(&training_names)?;
        let validation_examples: Vec<VocabularyTrainingExample> = dataset(&validation_names)?;
        let test_examples: Vec<VocabularyTrainingExample> = dataset(&test_names)?;
        println!(
            "Имена train/dev/test: {}/{}/{}; примеры: {}/{}/{}",
            training_names.len(),
            validation_names.len(),
            test_names.len(),
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
    let vocabulary_size: usize = vocabulary.len();
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
        // Область размещения данных внутри SVG 800 × 600; поля оставлены под оси и подписи.
        const DATA_AREA_LEFT: f32 = 65.0; // Левая граница в пикселях SVG.
        const DATA_AREA_BOTTOM: f32 = 535.0; // Нижняя граница в пикселях SVG.
        const DATA_AREA_WIDTH: f32 = 680.0; // Данные занимают x от 65 до 745.
        const DATA_AREA_HEIGHT: f32 = 470.0; // Данные занимают y от 65 до 535.
        // Не даём делителю стать нулём, если все значения по одной оси одинаковы.
        const MINIMUM_VALUE_SPAN: f32 = 1e-6;
        let to_pixel_coordinates = |data_point: &(f32, f32)| -> (f32, f32) {
            (
                DATA_AREA_LEFT
                    + DATA_AREA_WIDTH * (data_point.0 - minimum_horizontal_value)
                        / (maximum_horizontal_value - minimum_horizontal_value)
                            .max(MINIMUM_VALUE_SPAN),
                // В SVG y растёт вниз, поэтому рост значения вычитается из нижней границы.
                DATA_AREA_BOTTOM
                    - DATA_AREA_HEIGHT * (data_point.1 - minimum_vertical_value)
                        / (maximum_vertical_value - minimum_vertical_value).max(MINIMUM_VALUE_SPAN),
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
    // Для одного примера возвращает (cross-entropy правильного символа, вероятности всех символов).
    let cross_entropy = |character_scores: &[f32],
                         vocabulary_character_index_target: VocabularyCharacterIndex|
     -> (f32, Vec<f32>) {
        // Вычитаем максимальную оценку перед exp; cross-entropy считаем через log-sum-exp без log(вероятности).
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
            (maximum_score - character_scores[vocabulary_character_index_target.0])
                + exponential_sum.ln(),
            character_probabilities,
        )
    };
    // Для одного контекста возвращает (объединённый вектор эмбеддингов, активации tanh, оценки до softmax).
    // Модель передаётся аргументом; размер выхода берётся из захваченной переменной vocabulary_size.
    let forward = |model: &Model,
                   vocabulary_context_indices: VocabularyContextIndices|
     -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        // Берём векторы символов контекста из C и объединяем в исходном порядке.
        // Длина входного вектора: CONTEXT_LENGTH * EMBEDDING_DIMENSIONS.
        let flattened_embeddings: Vec<f32> = vocabulary_context_indices
            .iter()
            .flat_map(|&character_index| {
                model.character_embeddings[character_index.0]
                    .iter()
                    .copied()
            })
            .collect();
        let hidden_activations: Vec<f32> = {
            let mut hidden_activations: Vec<f32> = model.hidden_biases.to_vec();
            for (input_index, &input_value) in flattened_embeddings.iter().enumerate() {
                for (hidden_activation, &connection_weight) in hidden_activations.iter_mut().zip(
                    &model.input_to_hidden_weights[input_index * HIDDEN_NEURON_COUNT
                        ..(input_index + 1) * HIDDEN_NEURON_COUNT],
                ) {
                    *hidden_activation += input_value * connection_weight;
                }
            }
            for hidden_activation in &mut hidden_activations {
                *hidden_activation = hidden_activation.tanh();
            }
            hidden_activations
        };
        let character_scores: Vec<f32> = {
            let mut character_scores: Vec<f32> = model.output_biases.clone();
            for (hidden_neuron_index, &hidden_activation) in hidden_activations.iter().enumerate() {
                for (character_score, &connection_weight) in character_scores.iter_mut().zip(
                    &model.hidden_to_output_weights[hidden_neuron_index * vocabulary_size
                        ..(hidden_neuron_index + 1) * vocabulary_size],
                ) {
                    *character_score += hidden_activation * connection_weight;
                }
            }
            character_scores
        };
        (flattened_embeddings, hidden_activations, character_scores)
    };
    // Усредняет cross-entropy по непустому набору; захватывает forward и cross_entropy, модель не изменяет.
    let loss = |model: &Model, examples: NonEmptySlice<'_, VocabularyTrainingExample>| -> f32 {
        examples
            .iter()
            .map(|example| {
                cross_entropy(
                    &forward(model, example.vocabulary_context_indices).2,
                    example.vocabulary_character_index_target,
                )
                .0 as f64
            })
            .sum::<f64>() as f32
            / examples.len() as f32
    };
    let model: Model = {
        let mut random_generator: RandomGenerator = RandomGenerator::new(2147483647);
        // Возвращает следующее начальное значение параметра из N(0, 1).
        let mut initialize_value = || -> f32 {
            (-2.0 * random_generator.uniform().ln()).sqrt()
                * (2.0 * std::f32::consts::PI * random_generator.uniform()).cos()
        };
        // Для 27 символов создаёт решётку 3 × 3 × 3 в первых трёх координатах.
        // Остальные координаты сначала нулевые, затем обучаются вместе со всеми параметрами.
        let initialize_embeddings_regular_cube = || -> Vec<[f32; EMBEDDING_DIMENSIONS]> {
            if vocabulary_size < 2 {
                return vec![[0.0; EMBEDDING_DIMENSIONS]; vocabulary_size];
            }
            let grid_dimensions: usize = EMBEDDING_DIMENSIONS.min(3);
            let mut levels_per_axis: usize = 1;
            while levels_per_axis.pow(grid_dimensions as u32) < vocabulary_size {
                levels_per_axis += 1;
            }
            let node_count: usize = levels_per_axis.pow(grid_dimensions as u32);
            let half_width: f32 = 3.0f32.sqrt();
            (0..vocabulary_size)
                .map(|symbol_index| {
                    // Если число символов не заполняет решётку, выбираем узлы равномерно
                    // по её линейной нумерации, включая оба крайних узла; случайности нет.
                    let mut node_index: usize =
                        symbol_index * (node_count - 1) / (vocabulary_size - 1);
                    let mut embedding: [f32; EMBEDDING_DIMENSIONS] = [0.0; EMBEDDING_DIMENSIONS];
                    for coordinate in &mut embedding[..grid_dimensions] {
                        let level: usize = node_index % levels_per_axis;
                        node_index /= levels_per_axis;
                        *coordinate = -half_width
                            + 2.0 * half_width * level as f32 / (levels_per_axis - 1) as f32;
                    }
                    embedding
                })
                .collect()
        };
        // true выбирает регулярный куб для C; false — нормальное распределение.
        const USE_REGULAR_CUBE_EMBEDDINGS: bool = false;
        // Создаём нормальную C в обоих режимах, чтобы остальные веса получали те же случайные числа.
        // C: (vocabulary_size, EMBEDDING_DIMENSIONS) — обучаемые координаты символов.
        let normal_character_embeddings: Vec<[f32; EMBEDDING_DIMENSIONS]> = (0..vocabulary_size)
            .map(|_| std::array::from_fn(|_| initialize_value()))
            .collect();
        let character_embeddings: Vec<[f32; EMBEDDING_DIMENSIONS]> = if USE_REGULAR_CUBE_EMBEDDINGS
        {
            initialize_embeddings_regular_cube()
        } else {
            normal_character_embeddings
        };
        // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, HIDDEN_NEURON_COUNT) — массив фиксированной длины.
        let input_to_hidden_weights: [f32; CONTEXT_LENGTH
            * EMBEDDING_DIMENSIONS
            * HIDDEN_NEURON_COUNT] = std::array::from_fn(|_| initialize_value());
        // b1: (HIDDEN_NEURON_COUNT,) — смещения скрытых нейронов, также фиксированный массив.
        let hidden_biases: [f32; HIDDEN_NEURON_COUNT] = std::array::from_fn(|_| initialize_value());
        // W2: (HIDDEN_NEURON_COUNT, vocabulary_size) — размер зависит от словаря.
        let hidden_to_output_weights: Vec<f32> = (0..HIDDEN_NEURON_COUNT * vocabulary_size)
            .map(|_| initialize_value())
            .collect();
        // b2: (vocabulary_size,) — смещения оценок символов, размер зависит от словаря.
        let output_biases: Vec<f32> = (0..vocabulary_size).map(|_| initialize_value()).collect();
        Model {
            character_embeddings,
            input_to_hidden_weights,
            hidden_biases,
            hidden_to_output_weights,
            output_biases,
        }
    };
    let parameter_count: usize = model.character_embeddings.len() * EMBEDDING_DIMENSIONS
        + model.input_to_hidden_weights.len()
        + model.hidden_biases.len()
        + model.hidden_to_output_weights.len()
        + model.output_biases.len();
    println!(
        "C=({},{}), W1=({},{}), b1=({}), W2=({},{}), b2=({}); параметров: {}",
        vocabulary_size,
        EMBEDDING_DIMENSIONS,
        CONTEXT_LENGTH * EMBEDDING_DIMENSIONS,
        HIDDEN_NEURON_COUNT,
        HIDDEN_NEURON_COUNT,
        HIDDEN_NEURON_COUNT,
        vocabulary_size,
        vocabulary_size,
        parameter_count
    );
    let initial_validation_loss: f32 = loss(
        &model,
        NonEmptySlice::try_from(validation_examples.as_slice())?,
    );
    println!("До обучения dev loss={initial_validation_loss:.6}");
    // 3. SGD: выборка с возвращением, средняя cross-entropy и новые нулевые градиенты на каждом шаге.
    let (model, training_loss_curve): (Model, Vec<(f32, f32)>) = {
        let mut model: Model = model;
        let mut batch_random_generator: RandomGenerator = RandomGenerator::new(2147483647);
        let mut loss_csv_file: std::io::BufWriter<std::fs::File> = std::io::BufWriter::new(
            files
                .remove(LOSS_CSV)
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
                let (loss, gradients): (f32, ModelGradients) = {
                    let batch: Vec<VocabularyTrainingExample> = (0..32)
                        .map(|_| {
                            training_examples[batch_random_generator.index(training_examples.len())]
                        })
                        .collect();
                    let training_batch: NonEmptySlice<'_, VocabularyTrainingExample> =
                        NonEmptySlice::try_from(batch.as_slice())?;
                    let mut gradients: ModelGradients = ModelGradients {
                        character_embeddings: vec![
                            [0.0; EMBEDDING_DIMENSIONS];
                            model.character_embeddings.len()
                        ],
                        input_to_hidden_weights: [0.0; CONTEXT_LENGTH
                            * EMBEDDING_DIMENSIONS
                            * HIDDEN_NEURON_COUNT],
                        hidden_biases: [0.0; HIDDEN_NEURON_COUNT],
                        hidden_to_output_weights: vec![0.0; model.hidden_to_output_weights.len()],
                        output_biases: vec![0.0; model.output_biases.len()],
                    };
                    let mut total_loss: f32 = 0.0;
                    for example in training_batch.iter() {
                        let (flattened_embeddings, hidden_activations, character_scores): (
                            Vec<f32>,
                            Vec<f32>,
                            Vec<f32>,
                        ) = forward(&model, example.vocabulary_context_indices);
                        let score_gradients: Vec<f32> = {
                            let (example_loss, mut score_gradients): (f32, Vec<f32>) =
                                cross_entropy(
                                    &character_scores,
                                    example.vocabulary_character_index_target,
                                );
                            total_loss += example_loss;
                            // Вероятности превращаем в производные ошибки по выходным оценкам.
                            // Для правильной буквы вычитаем 1, затем усредняем по мини-батчу.
                            score_gradients[example.vocabulary_character_index_target.0] -= 1.0;
                            for score_gradient in &mut score_gradients {
                                *score_gradient /= training_batch.len() as f32;
                            }
                            score_gradients
                        };
                        for (output_bias_gradient, &score_gradient) in
                            gradients.output_biases.iter_mut().zip(&score_gradients)
                        {
                            *output_bias_gradient += score_gradient;
                        }
                        // Производные ошибки по суммам скрытого слоя ДО применения tanh.
                        let hidden_pre_activation_gradients: Vec<f32> = {
                            let mut hidden_pre_activation_gradients: Vec<f32> =
                                vec![0.0; HIDDEN_NEURON_COUNT];
                            for hidden_neuron_index in 0..HIDDEN_NEURON_COUNT {
                                for output_character_index in 0..vocabulary_size {
                                    gradients.hidden_to_output_weights[hidden_neuron_index
                                        * vocabulary_size
                                        + output_character_index] += hidden_activations
                                        [hidden_neuron_index]
                                        * score_gradients[output_character_index];
                                    hidden_pre_activation_gradients[hidden_neuron_index] += model
                                        .hidden_to_output_weights[hidden_neuron_index
                                        * vocabulary_size
                                        + output_character_index]
                                        * score_gradients[output_character_index];
                                }
                                // Производная tanh равна 1 - tanh².
                                hidden_pre_activation_gradients[hidden_neuron_index] *= 1.0
                                    - hidden_activations[hidden_neuron_index]
                                        * hidden_activations[hidden_neuron_index];
                                gradients.hidden_biases[hidden_neuron_index] +=
                                    hidden_pre_activation_gradients[hidden_neuron_index];
                            }
                            hidden_pre_activation_gradients
                        };
                        for input_coordinate_index in 0..flattened_embeddings.len() {
                            let input_coordinate_gradient: f32 = {
                                let mut input_coordinate_gradient: f32 = 0.0;
                                for hidden_neuron_index in 0..HIDDEN_NEURON_COUNT {
                                    gradients.input_to_hidden_weights[input_coordinate_index
                                        * HIDDEN_NEURON_COUNT
                                        + hidden_neuron_index] += flattened_embeddings
                                        [input_coordinate_index]
                                        * hidden_pre_activation_gradients[hidden_neuron_index];
                                    input_coordinate_gradient += model.input_to_hidden_weights
                                        [input_coordinate_index * HIDDEN_NEURON_COUNT
                                            + hidden_neuron_index]
                                        * hidden_pre_activation_gradients[hidden_neuron_index];
                                }
                                input_coordinate_gradient
                            };
                            // Scatter-add: повторяющиеся символы суммируют градиенты одной строки C.
                            gradients.character_embeddings[example.vocabulary_context_indices
                                [input_coordinate_index / EMBEDDING_DIMENSIONS]
                                .0][input_coordinate_index % EMBEDDING_DIMENSIONS] +=
                                input_coordinate_gradient;
                        }
                    }
                    (total_loss / training_batch.len() as f32, gradients)
                };
                for (parameter_tensor, gradient_tensor) in [
                    (
                        model.character_embeddings.as_flattened_mut(),
                        gradients.character_embeddings.as_flattened(),
                    ),
                    (
                        &mut model.input_to_hidden_weights[..],
                        &gradients.input_to_hidden_weights[..],
                    ),
                    (&mut model.hidden_biases[..], &gradients.hidden_biases[..]),
                    (
                        &mut model.hidden_to_output_weights[..],
                        &gradients.hidden_to_output_weights[..],
                    ),
                    (&mut model.output_biases[..], &gradients.output_biases[..]),
                ] {
                    for (parameter, &gradient) in parameter_tensor.iter_mut().zip(gradient_tensor) {
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
    // 4. Считаем средние loss на train/dev/test без обновления параметров и сохраняем графики.
    // Dev служит для выбора настроек, test — для окончательной оценки после их выбора.
    {
        let metrics_text: String = format!(
            "steps={training_steps}\nparameters={parameter_count}\ninitial_dev_loss={initial_validation_loss:.6}\ntrain_loss={training_loss:.6}\ndev_loss={validation_loss:.6}\ntest_loss={test_loss:.6}\n",
            training_loss = loss(
                &model,
                NonEmptySlice::try_from(training_examples.as_slice())?
            ),
            validation_loss = loss(
                &model,
                NonEmptySlice::try_from(validation_examples.as_slice())?
            ),
            test_loss = loss(&model, NonEmptySlice::try_from(test_examples.as_slice())?),
        );
        print!("{metrics_text}");
        std::io::Write::write_all(
            &mut files
                .remove(METRICS_TXT)
                .ok_or("файл отсутствует в подготовленном наборе")?,
            (metrics_text).as_bytes(),
        )?;
    }
    plot(
        files
            .remove(LOSS_SVG)
            .ok_or("файл отсутствует в подготовленном наборе")?,
        &training_loss_curve,
        None,
        "Training: x=step, y=log10(minibatch loss)",
    )?;
    let embedding_coordinates: Vec<(f32, f32)> = model
        .character_embeddings
        .iter()
        .map(|embedding| (embedding[0], embedding[1]))
        .collect();
    plot(
        files
            .remove(EMBEDDINGS_SVG)
            .ok_or("файл отсутствует в подготовленном наборе")?,
        &embedding_coordinates,
        Some(&vocabulary.0),
        &format!("C: x=dimension 0, y=dimension 1 (of {EMBEDDING_DIMENSIONS})"),
    )?;
    {
        let mut embeddings_csv_file: std::fs::File = files
            .remove(EMBEDDINGS_CSV)
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
    // 5. Генерируем имена с отдельным seed: каждый выбранный символ добавляем в контекст до точки или лимита.
    let generated_names: String = {
        let mut sampling_random_generator: RandomGenerator = RandomGenerator::new(2147483647 + 10);
        let mut generated_names: String = String::new();
        for _ in 0..20 {
            let name: String = 'sample: {
                let mut vocabulary_context_indices: VocabularyContextIndices =
                    [vocabulary.dot_index(); CONTEXT_LENGTH];
                let mut generated_name: String = String::new();
                for _ in 0..100 {
                    let (_, character_probabilities): (f32, Vec<f32>) = cross_entropy(
                        &forward(&model, vocabulary_context_indices).2,
                        vocabulary.dot_index(),
                    );
                    let random_probability: f32 = sampling_random_generator.uniform();
                    let sampled_character_index: usize = {
                        let mut cumulative_probability: f32 = 0.0;
                        let mut sampled_character_index: usize = vocabulary_size - 1;
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
                    let sampled_character_index: VocabularyCharacterIndex =
                        vocabulary.character_index(sampled_character_index)?;
                    generated_name.push(vocabulary.character(sampled_character_index)?);
                    if sampled_character_index.0 == 0 {
                        break 'sample generated_name;
                    }
                    vocabulary_context_indices.rotate_left(1);
                    vocabulary_context_indices[CONTEXT_LENGTH - 1] = sampled_character_index;
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
            .remove(SAMPLES_TXT)
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
