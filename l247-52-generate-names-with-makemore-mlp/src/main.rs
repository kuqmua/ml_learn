fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_started_at: std::time::Instant = std::time::Instant::now();
    /// Владеющий вектор, содержащий хотя бы один элемент.
    /// Создаётся через TryFrom; доступ через срез не позволяет удалить все элементы.
    #[derive(Clone, Debug, derive_more::Deref, derive_more::IntoIterator)]
    #[deref(forward)]
    struct NonEmptyVec<T>(Vec<T>);
    impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
        type Error = &'static str;
        fn try_from(values: Vec<T>) -> Result<Self, Self::Error> {
            if values.is_empty() {
                Err("вектор не должен быть пустым")
            } else {
                Ok(Self(values))
            }
        }
    }
    impl<T> NonEmptyVec<T> {
        // Преобразует элементы, сохраняя их количество и непустоту.
        fn map<U>(&self, transform: impl FnMut(&T) -> U) -> NonEmptyVec<U> {
            NonEmptyVec(self.iter().map(transform).collect())
        }
    }
    impl<T> NonEmptyVec<T> {
        // Проверяет преобразование каждого элемента, сохраняя непустую форму.
        fn try_map<U>(
            &self,
            mut transform: impl FnMut(&T) -> Result<U, &'static str>,
        ) -> Result<NonEmptyVec<U>, &'static str> {
            Ok(NonEmptyVec(
                self.iter()
                    .map(&mut transform)
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
    // Поэлементное проверяемое преобразование без изменения длины массива.
    fn try_map_array<T, U, const N: usize>(
        values: [T; N],
        transform: impl FnMut(T) -> Result<U, &'static str>,
    ) -> Result<[U; N], &'static str> {
        values
            .into_iter()
            .map(transform)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| "при преобразовании изменилась длина массива")
    }
    // Словарь символов: точка под индексом 0, затем уникальные буквы a–z по алфавиту.
    struct Vocabulary(
        // Символы в порядке их индексов; создаются методом new.
        NonEmptyVec<char>,
    );
    impl Vocabulary {
        // Отбирает из текста буквы a–z, сортирует, удаляет повторы и добавляет точку под индексом 0.
        fn new(text: &str) -> Result<Self, &'static str> {
            let mut characters: Vec<char> = text
                .chars()
                .filter(|character| character.is_ascii_lowercase())
                .collect();
            characters.sort_unstable();
            characters.dedup();
            characters.insert(0, '.');
            Ok(Self(NonEmptyVec::try_from(characters)?))
        }
        // Возвращает количество символов, включая точку.
        fn len(&self) -> usize {
            self.0.len()
        }
        // Возвращает индекс точки: она обозначает границу имени и заполняет начальный контекст.
        // Метод new всегда помещает точку под индексом 0.
        fn dot_index(&self) -> VocabularyCharacterIndexAsUsize {
            VocabularyCharacterIndexAsUsize(0)
        }
        // Находит индекс символа; для отсутствующего символа возвращает ошибку.
        fn index_of(
            &self,
            character: char,
        ) -> Result<VocabularyCharacterIndexAsUsize, &'static str> {
            let value: usize = self
                .0
                .iter()
                .position(|&candidate| candidate == character)
                .ok_or("символ отсутствует в словаре")?;
            VocabularyCharacterIndexAsUsize::try_from((value, self))
        }
        // Возвращает символ по числовой позиции, проверяя границы; принадлежность индекса словарю не проверяется.
        fn character(&self, index: VocabularyCharacterIndexAsUsize) -> Result<char, &'static str> {
            self.0
                .get(index.0)
                .copied()
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
    }
    // Позиция символа, проверенная относительно размера Vocabulary при создании.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct VocabularyCharacterIndexAsUsize(
        // Позиция проверена при создании; принадлежность конкретному экземпляру Vocabulary не хранится.
        usize,
    );
    impl TryFrom<(usize, &Vocabulary)> for VocabularyCharacterIndexAsUsize {
        type Error = &'static str;
        // Создаёт индекс, только если value < vocabulary.len().
        fn try_from((value, vocabulary): (usize, &Vocabulary)) -> Result<Self, Self::Error> {
            vocabulary
                .0
                .get(value)
                .map(|_| Self(value))
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
    }
    // Сколько предыдущих символов получает модель; значение должно быть положительным.
    const CONTEXT_LENGTH: usize = 3;
    // Обучающий пример в индексах Vocabulary: контекст длины CONTEXT_LENGTH и правильный следующий символ.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct VocabularyContextAndTargetIndices {
        // Индексы предыдущих CONTEXT_LENGTH символов в Vocabulary; индекс точки заполняет начало имени.
        vocabulary_context_indices: [VocabularyCharacterIndexAsUsize; CONTEXT_LENGTH],
        // Индекс правильного следующего символа в Vocabulary; индекс точки обозначает конец имени.
        vocabulary_character_index_target: VocabularyCharacterIndexAsUsize,
    }
    // Нулевое значение для координат, градиентов и накопительных сумм.
    const ZERO_F32: f32 = 0.0;
    // Фиксированные размеры модели, известные при компиляции.
    const EMBEDDING_DIMENSIONS: usize = 10;
    const FIRST_LAYER_SIZE: usize = 200;
    // Число примеров, по которым усредняются градиенты одного шага.
    const BATCH_SIZE: usize = 32;
    const _: () = assert!(
        CONTEXT_LENGTH > 0 && EMBEDDING_DIMENSIONS > 0 && FIRST_LAYER_SIZE > 0 && BATCH_SIZE > 0
    );
    // Положительное число обновлений параметров; для нуля возвращается ошибка.
    let training_steps: std::num::NonZeroUsize =
        std::num::NonZeroUsize::new(200_000).ok_or("число шагов должно быть положительным")?;
    // Один seed и общий генератор для всех случайных операций программы.
    let mut random_generator: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(42);
    let (vocabulary, training_examples, validation_examples, test_examples) = {
        // 1. Проверяем записи; повторения сохраняем, чтобы не менять их частоты в наборе.
        let names_text: &str = include_str!("../data/names.txt");
        // Одно непустое имя из строчных латинских букв; заимствует исходный текст.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        struct Name<'a>(&'a str);
        impl<'a> TryFrom<&'a str> for Name<'a> {
            type Error = &'static str;
            fn try_from(text: &'a str) -> Result<Self, Self::Error> {
                if text.is_empty() || !text.chars().all(|character| character.is_ascii_lowercase())
                {
                    Err("имя должно быть непустым и содержать только буквы a–z")
                } else {
                    Ok(Self(text))
                }
            }
        }
        impl std::fmt::Debug for Name<'_> {
            // Выводит исходную строку имени без дополнительной обёртки.
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(self.0, formatter)
            }
        }
        let names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            names_text
                .lines()
                .map(Name::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        println!(
            "Имена: {}, первые восемь: {:?}",
            names.len(),
            &names[..names.len().min(8)]
        );
        // 2. Объединяем одинаковые имена, чтобы их копии не оказались одновременно в train/dev/test.
        // BTreeMap задаёт стабильный порядок групп перед воспроизводимым перемешиванием с seed 42.
        let mut name_groups: NonEmptyVec<NonEmptyVec<Name<'_>>> = {
            let mut groups: std::collections::BTreeMap<Name<'_>, Vec<Name<'_>>> =
                std::collections::BTreeMap::new();
            for &name in names.iter() {
                groups.entry(name).or_default().push(name);
            }
            NonEmptyVec::try_from(
                groups
                    .into_values()
                    .map(NonEmptyVec::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )?
        };
        rand::seq::SliceRandom::shuffle(name_groups.0.as_mut_slice(), &mut random_generator);
        // Доли 80/10/10 относятся к уникальным именам; число записей зависит от размеров групп.
        let (training_name_groups, remaining_name_groups): (
            &[NonEmptyVec<Name<'_>>],
            &[NonEmptyVec<Name<'_>>],
        ) = name_groups.split_at(name_groups.len() * 8 / 10);
        let (validation_name_groups, test_name_groups): (
            &[NonEmptyVec<Name<'_>>],
            &[NonEmptyVec<Name<'_>>],
        ) = remaining_name_groups.split_at(name_groups.len() * 9 / 10 - training_name_groups.len());
        let training_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            training_name_groups
                .iter()
                .flat_map(|group| group.iter())
                .copied()
                .collect::<Vec<_>>(),
        )?;
        let validation_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            validation_name_groups
                .iter()
                .flat_map(|group| group.iter())
                .copied()
                .collect::<Vec<_>>(),
        )?;
        let test_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            test_name_groups
                .iter()
                .flat_map(|group| group.iter())
                .copied()
                .collect::<Vec<_>>(),
        )?;
        // Состав Vocabulary определяем только по train. Неизвестный символ в dev/test вернёт ошибку.
        let vocabulary: Vocabulary =
            Vocabulary::new(&training_names.map(|name| name.0).join("\n"))?;
        println!("Словарь (только train): {:?}", vocabulary.0);
        // Захватывает Vocabulary: для каждого имени сбрасывает контекст и строит пары «контекст → следующий символ».
        // Вход и результат непустые; точка в конце учит завершать имя, неизвестный символ возвращает ошибку.
        let build_vocabulary_context_and_target_indices = |names: NonEmptySlice<'_, Name<'_>>| -> Result<
            NonEmptyVec<VocabularyContextAndTargetIndices>,
            &'static str,
        > {
            let mut examples: Vec<VocabularyContextAndTargetIndices> = Vec::new();
            for name in names.iter() {
                let mut vocabulary_context_indices: [VocabularyCharacterIndexAsUsize;
                    CONTEXT_LENGTH] = [vocabulary.dot_index(); CONTEXT_LENGTH];
                for next_character in name.0.chars().chain(std::iter::once('.')) {
                    let vocabulary_character_index_target: VocabularyCharacterIndexAsUsize =
                        vocabulary.index_of(next_character)?;
                    examples.push(VocabularyContextAndTargetIndices {
                        vocabulary_context_indices,
                        vocabulary_character_index_target,
                    });
                    vocabulary_context_indices.rotate_left(1);
                    vocabulary_context_indices[CONTEXT_LENGTH - 1] =
                        vocabulary_character_index_target;
                }
            }
            NonEmptyVec::try_from(examples)
        };
        let training_examples: NonEmptyVec<VocabularyContextAndTargetIndices> =
            build_vocabulary_context_and_target_indices(NonEmptySlice::from(&training_names))?;
        let validation_examples: NonEmptyVec<VocabularyContextAndTargetIndices> =
            build_vocabulary_context_and_target_indices(NonEmptySlice::from(&validation_names))?;
        let test_examples: NonEmptyVec<VocabularyContextAndTargetIndices> =
            build_vocabulary_context_and_target_indices(NonEmptySlice::from(&test_names))?;
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
    let vocabulary_len: usize = vocabulary.len();
    // Обёртка над срезом, для которого уже проверено наличие хотя бы одного элемента.
    #[derive(Debug, derive_more::Deref)]
    #[deref(forward)]
    #[derive_where::derive_where(Clone, Copy)]
    struct NonEmptySlice<'a, T>(
        // Заимствованный срез: проверка не копирует его элементы.
        &'a [T],
    );
    impl<'a, T> TryFrom<&'a [T]> for NonEmptySlice<'a, T> {
        type Error = &'static str;
        // Отклоняет пустой срез и оборачивает непустой без копирования элементов.
        fn try_from(values: &'a [T]) -> Result<Self, Self::Error> {
            if values.is_empty() {
                Err("срез не должен быть пустым")
            } else {
                Ok(Self(values))
            }
        }
    }
    impl<'a, T> From<&'a NonEmptyVec<T>> for NonEmptySlice<'a, T> {
        // Заимствует непустой вектор без копирования и повторной проверки.
        fn from(values: &'a NonEmptyVec<T>) -> Self {
            Self(&values.0)
        }
    }
    /// Вероятность символа в диапазоне [0, 1]; NaN и бесконечности запрещены.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct CharacterProbabilityAsF32From0To1(f32);
    impl TryFrom<f32> for CharacterProbabilityAsF32From0To1 {
        type Error = &'static str;
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            if (0.0..=1.0).contains(&value) {
                Ok(Self(value))
            } else {
                Err("вероятность символа должна быть от 0 до 1")
            }
        }
    }
    /// Конечная неотрицательная ошибка предсказания; NaN и бесконечности запрещены.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct CrossEntropyLossAsNonNegativeF32(f32);
    impl TryFrom<f32> for CrossEntropyLossAsNonNegativeF32 {
        type Error = &'static str;
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            if value.is_finite() && value >= 0.0 {
                Ok(Self(value))
            } else {
                Err("cross-entropy loss должен быть конечным и неотрицательным")
            }
        }
    }
    // Конечное число f32: NaN и обе бесконечности отклоняются при создании.
    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
    struct FiniteF32(f32);
    impl TryFrom<f32> for FiniteF32 {
        type Error = &'static str;
        // Проверяет конечность значения, не ограничивая его знак и величину.
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            if value.is_finite() {
                Ok(Self(value))
            } else {
                Err("значение должно быть конечным числом")
            }
        }
    }
    // Положительная конечная скорость обучения; ноль, отрицательные значения и бесконечности запрещены.
    #[derive(Clone, Copy, Debug)]
    struct LearningRateAsPositiveFiniteF32(f32);
    impl TryFrom<f32> for LearningRateAsPositiveFiniteF32 {
        type Error = &'static str;
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            if value.is_finite() && value > ZERO_F32 {
                Ok(Self(value))
            } else {
                Err("скорость обучения должна быть положительным конечным числом")
            }
        }
    }
    // Для одного примера возвращает (loss, character_probabilities).
    // loss — ошибка предсказания: -ln(вероятность правильного следующего символа).
    let cross_entropy = |character_scores: NonEmptySlice<'_, FiniteF32>,
                         vocabulary_character_index_target: VocabularyCharacterIndexAsUsize|
     -> Result<
        (
            CrossEntropyLossAsNonNegativeF32,
            NonEmptyVec<CharacterProbabilityAsF32From0To1>,
        ),
        &'static str,
    > {
        // Вычитаем максимальную оценку перед exp; cross-entropy считаем через log-sum-exp без log(вероятности).
        let maximum_score: f32 = character_scores
            .iter()
            .map(|score| score.0)
            .fold(f32::NEG_INFINITY, f32::max);
        let character_exponentials: NonEmptyVec<f32> = NonEmptyVec::try_from(
            character_scores
                .iter()
                .map(|character_score| (character_score.0 - maximum_score).exp())
                .collect::<Vec<_>>(),
        )?;
        let exponential_sum: f32 = character_exponentials.iter().sum();
        let character_probabilities: NonEmptyVec<CharacterProbabilityAsF32From0To1> =
            NonEmptyVec::try_from(
                character_exponentials
                    .into_iter()
                    .map(|value| {
                        CharacterProbabilityAsF32From0To1::try_from(value / exponential_sum)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )?;
        let target_score: f32 = character_scores
            .get(vocabulary_character_index_target.0)
            .ok_or("индекс целевого символа выходит за границы оценок")?
            .0;
        let loss: f32 = (maximum_score - target_score) + exponential_sum.ln();
        Ok((
            CrossEntropyLossAsNonNegativeF32::try_from(loss)?,
            character_probabilities,
        ))
    };
    let (model, forward, loss) = {
        // Параметры MLP: таблица эмбеддингов и два полносвязных слоя, между которыми применяется tanh.
        // Прямой проход и loss — замыкания ниже; размер словаря хранится в локальной переменной vocabulary_len.
        #[derive(Clone)]
        struct Model {
            // C: одна строка из EMBEDDING_DIMENSIONS координат на каждый символ Vocabulary.
            character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
            // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, FIRST_LAYER_SIZE) — веса первого слоя.
            first_layer_weights:
                [[FiniteF32; FIRST_LAYER_SIZE]; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
            // b1: (FIRST_LAYER_SIZE,) — смещения нейронов первого слоя.
            first_layer_biases: [FiniteF32; FIRST_LAYER_SIZE],
            // W2: для каждого символа — веса от всех нейронов первого слоя.
            second_layer_weights: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
            // b2: (vocabulary_len,) — смещения оценок следующих символов.
            second_layer_biases: NonEmptyVec<FiniteF32>,
        }
        let model: Model = {
            // Возвращает следующее начальное значение параметра из N(0, 1).
            let mut initialize_embeddings_sample_standard_normal = || -> f32 {
                rand_distr::Distribution::sample(&rand_distr::StandardNormal, &mut random_generator)
            };
            // Для 27 символов создаёт решётку 3 × 3 × 3 в первых трёх координатах.
            // Остальные координаты сначала нулевые, затем обучаются вместе со всеми параметрами.
            let initialize_embeddings_regular_cube =
                || -> NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> {
                    if vocabulary_len < 2 {
                        return vocabulary.0.map(|_| [ZERO_F32; EMBEDDING_DIMENSIONS]);
                    }
                    let grid_dimensions: usize = EMBEDDING_DIMENSIONS.min(3);
                    let mut levels_per_axis: usize = 1;
                    while levels_per_axis.pow(grid_dimensions as u32) < vocabulary_len {
                        levels_per_axis += 1;
                    }
                    let node_count: usize = levels_per_axis.pow(grid_dimensions as u32);
                    let half_width: f32 = 3.0f32.sqrt();
                    let mut symbol_index: usize = 0;
                    vocabulary.0.map(|_| {
                        // Если число символов не заполняет решётку, выбираем узлы равномерно
                        // по её линейной нумерации, включая оба крайних узла; случайности нет.
                        let mut node_index: usize =
                            symbol_index * (node_count - 1) / (vocabulary_len - 1);
                        let mut embedding: [f32; EMBEDDING_DIMENSIONS] =
                            [ZERO_F32; EMBEDDING_DIMENSIONS];
                        for coordinate in &mut embedding[..grid_dimensions] {
                            let level: usize = node_index % levels_per_axis;
                            node_index /= levels_per_axis;
                            *coordinate = -half_width
                                + 2.0 * half_width * level as f32 / (levels_per_axis - 1) as f32;
                        }
                        symbol_index += 1;
                        embedding
                    })
                };
            // Создаём нормальную C в обоих режимах, чтобы остальные веса получали те же случайные числа.
            // C: (vocabulary_len, EMBEDDING_DIMENSIONS) — обучаемые координаты символов.
            let normal_character_embeddings: NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> = vocabulary
                .0
                .map(|_| std::array::from_fn(|_| initialize_embeddings_sample_standard_normal()));
            let character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]> = {
                let embeddings: NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> = if false {
                    initialize_embeddings_regular_cube()
                } else {
                    normal_character_embeddings
                };
                embeddings.try_map(|row| try_map_array(*row, FiniteF32::try_from))?
            };
            // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, FIRST_LAYER_SIZE) — массив фиксированной длины.
            let first_layer_weights: [[FiniteF32; FIRST_LAYER_SIZE];
                CONTEXT_LENGTH * EMBEDDING_DIMENSIONS] = try_map_array(
                std::array::from_fn(|_| {
                    std::array::from_fn(|_| initialize_embeddings_sample_standard_normal())
                }),
                |row| try_map_array(row, FiniteF32::try_from),
            )?;
            // b1: (FIRST_LAYER_SIZE,) — смещения нейронов первого слоя, также фиксированный массив.
            let first_layer_biases: [FiniteF32; FIRST_LAYER_SIZE] = try_map_array(
                std::array::from_fn(|_| initialize_embeddings_sample_standard_normal()),
                FiniteF32::try_from,
            )?;
            // W2: для каждого символа — FIRST_LAYER_SIZE связей с первым слоем.
            let mut second_layer_weights: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]> = {
                let zero_weight: FiniteF32 = FiniteF32::try_from(ZERO_F32)?;
                vocabulary.0.map(|_| [zero_weight; FIRST_LAYER_SIZE])
            };
            // Сначала перебираем компоненты первого слоя, затем символы: порядок случайных чисел сохраняется.
            for first_layer_index in 0..FIRST_LAYER_SIZE {
                for character_weights in second_layer_weights.0.iter_mut() {
                    character_weights[first_layer_index] =
                        FiniteF32::try_from(initialize_embeddings_sample_standard_normal())?;
                }
            }
            // b2: (vocabulary_len,) — смещения оценок символов, размер зависит от словаря.
            let second_layer_biases: NonEmptyVec<FiniteF32> = NonEmptyVec::try_from(
                (0..vocabulary_len)
                    .map(|_| FiniteF32::try_from(initialize_embeddings_sample_standard_normal()))
                    .collect::<Result<Vec<_>, _>>()?,
            )?;
            Model {
                character_embeddings,
                first_layer_weights,
                first_layer_biases,
                second_layer_weights,
                second_layer_biases,
            }
        };
        // Результат прямого прохода; промежуточные значения нужны для расчёта градиентов.
        struct ForwardPassResult {
            // Эмбеддинги символов контекста, объединённые в один входной вектор.
            character_embeddings_flattened: [FiniteF32; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
            // Выходы нейронов первого слоя после применения tanh.
            first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE],
            // Оценки следующих символов до softmax, в порядке Vocabulary.
            character_scores: NonEmptyVec<FiniteF32>,
        }
        // Вычисляет оценки следующего символа и сохраняет промежуточные значения прямого прохода.
        let forward = |model: &Model,
                       vocabulary_context_indices: [VocabularyCharacterIndexAsUsize;
                           CONTEXT_LENGTH]|
         -> Result<ForwardPassResult, &'static str> {
            // Берём векторы символов контекста из C и объединяем в исходном порядке.
            // Длина входного вектора: CONTEXT_LENGTH * EMBEDDING_DIMENSIONS.
            let character_embeddings_flattened: [FiniteF32; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS] = {
                let context_embeddings: [[FiniteF32; EMBEDDING_DIMENSIONS]; CONTEXT_LENGTH] =
                    vocabulary_context_indices.map(|index| model.character_embeddings[index.0]);
                context_embeddings
                    .as_flattened()
                    .try_into()
                    .map_err(|_| "неверная длина объединённых эмбеддингов")?
            };
            let first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE] = {
                let mut first_layer_activations: [f32; FIRST_LAYER_SIZE] =
                    model.first_layer_biases.map(|value| value.0);
                for (context_embedding_coordinate_index, &context_embedding_coordinate) in
                    character_embeddings_flattened.iter().enumerate()
                {
                    for (first_layer_activation, &first_layer_weight) in first_layer_activations
                        .iter_mut()
                        .zip(&model.first_layer_weights[context_embedding_coordinate_index])
                    {
                        *first_layer_activation +=
                            context_embedding_coordinate.0 * first_layer_weight.0;
                    }
                }
                for first_layer_activation in &mut first_layer_activations {
                    *first_layer_activation =
                        FiniteF32::try_from(*first_layer_activation)?.0.tanh();
                }
                try_map_array(first_layer_activations, FiniteF32::try_from)?
            };
            let character_scores: NonEmptyVec<f32> = {
                let mut character_scores: NonEmptyVec<f32> =
                    model.second_layer_biases.map(|value| value.0);
                for (first_layer_index, &first_layer_activation) in
                    first_layer_activations.iter().enumerate()
                {
                    for (character_score, character_weights) in character_scores
                        .0
                        .iter_mut()
                        .zip(model.second_layer_weights.iter())
                    {
                        *character_score +=
                            first_layer_activation.0 * character_weights[first_layer_index].0;
                    }
                }
                character_scores
            };
            // Проверяем готовые оценки один раз перед передачей в cross_entropy.
            let character_scores: NonEmptyVec<FiniteF32> = NonEmptyVec::try_from(
                character_scores
                    .into_iter()
                    .map(FiniteF32::try_from)
                    .collect::<Result<Vec<_>, _>>()?,
            )?;
            Ok(ForwardPassResult {
                character_embeddings_flattened,
                first_layer_activations,
                character_scores,
            })
        };
        // Усредняет cross-entropy по непустому набору; захватывает forward и cross_entropy, модель не изменяет.
        let loss = move |model: &Model,
                         examples: NonEmptySlice<'_, VocabularyContextAndTargetIndices>|
              -> Result<CrossEntropyLossAsNonNegativeF32, &'static str> {
            let mut total_loss: f64 = 0.0;
            for example in examples.iter() {
                let (example_loss, _) = cross_entropy(
                    NonEmptySlice::from(
                        &forward(model, example.vocabulary_context_indices)?.character_scores,
                    ),
                    example.vocabulary_character_index_target,
                )?;
                total_loss += f64::from(example_loss.0);
            }
            CrossEntropyLossAsNonNegativeF32::try_from((total_loss / examples.len() as f64) as f32)
        };
        (model, forward, loss)
    };
    let trainable_scalar_count: usize = model.character_embeddings.len() * EMBEDDING_DIMENSIONS
        + model.first_layer_weights.len() * FIRST_LAYER_SIZE
        + model.first_layer_biases.len()
        + model.second_layer_weights.len() * FIRST_LAYER_SIZE
        + model.second_layer_biases.len();
    println!(
        "C=({},{}), W1=({},{}), b1=({}), W2=({},{}), b2=({}); параметров: {}",
        vocabulary_len,
        EMBEDDING_DIMENSIONS,
        CONTEXT_LENGTH * EMBEDDING_DIMENSIONS,
        FIRST_LAYER_SIZE,
        FIRST_LAYER_SIZE,
        FIRST_LAYER_SIZE,
        vocabulary_len,
        vocabulary_len,
        trainable_scalar_count
    );
    let initial_validation_loss: CrossEntropyLossAsNonNegativeF32 =
        loss(&model, NonEmptySlice::from(&validation_examples))?;
    println!("До обучения dev loss={:.6}", initial_validation_loss.0);
    // 3. SGD: выборка с возвращением, средняя cross-entropy и новые нулевые градиенты на каждом шаге.
    let model = {
        let mut model = model;
        let training_started_at: std::time::Instant = std::time::Instant::now();
        for step_index in 0..training_steps.get() {
            let iteration_started_at: std::time::Instant = std::time::Instant::now();
            let learning_rate: LearningRateAsPositiveFiniteF32 =
                LearningRateAsPositiveFiniteF32::try_from(if step_index < 100_000 {
                    0.1
                } else {
                    0.01
                })?;
            let loss: CrossEntropyLossAsNonNegativeF32 = {
                let (loss, gradients) = {
                    // Производные loss по каждому тензору Model; формы и порядок элементов совпадают с соответствующим полем.
                    struct ModelGradients<T> {
                        // Градиенты character_embeddings.
                        character_embeddings: NonEmptyVec<[T; EMBEDDING_DIMENSIONS]>,
                        // Градиенты first_layer_weights.
                        first_layer_weights:
                            [[T; FIRST_LAYER_SIZE]; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
                        // Градиенты first_layer_biases.
                        first_layer_biases: [T; FIRST_LAYER_SIZE],
                        // Градиенты second_layer_weights.
                        second_layer_weights: NonEmptyVec<[T; FIRST_LAYER_SIZE]>,
                        // Градиенты second_layer_biases.
                        second_layer_biases: NonEmptyVec<T>,
                    }
                    let batch: [VocabularyContextAndTargetIndices; BATCH_SIZE] =
                        std::array::from_fn(|_| {
                            training_examples[rand::RngExt::random_range(
                                &mut random_generator,
                                0..training_examples.len(),
                            )]
                        });
                    let training_batch: NonEmptySlice<'_, VocabularyContextAndTargetIndices> =
                        NonEmptySlice::try_from(batch.as_slice())?;
                    let mut gradients: ModelGradients<f32> = ModelGradients {
                        character_embeddings: model
                            .character_embeddings
                            .map(|_| [ZERO_F32; EMBEDDING_DIMENSIONS]),
                        first_layer_weights: [[ZERO_F32; FIRST_LAYER_SIZE];
                            CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
                        first_layer_biases: [ZERO_F32; FIRST_LAYER_SIZE],
                        second_layer_weights: model
                            .second_layer_weights
                            .map(|_| [ZERO_F32; FIRST_LAYER_SIZE]),
                        second_layer_biases: model.second_layer_biases.map(|_| ZERO_F32),
                    };
                    let mut total_loss: f32 = ZERO_F32;
                    for example in training_batch.iter() {
                        let forward_result = forward(&model, example.vocabulary_context_indices)?;
                        let score_gradients: NonEmptyVec<f32> = {
                            let (example_loss, character_probabilities) = cross_entropy(
                                NonEmptySlice::from(&forward_result.character_scores),
                                example.vocabulary_character_index_target,
                            )?;
                            total_loss += example_loss.0;
                            let mut score_gradients: NonEmptyVec<f32> =
                                character_probabilities.map(|probability| probability.0);
                            // Вероятности превращаем в производные ошибки по выходным оценкам.
                            // Для правильной буквы вычитаем 1, затем усредняем по мини-батчу.
                            score_gradients.0[example.vocabulary_character_index_target.0] -= 1.0;
                            for score_gradient in score_gradients.0.iter_mut() {
                                *score_gradient /= training_batch.len() as f32;
                            }
                            score_gradients
                        };
                        for (second_layer_bias_gradient, &score_gradient) in gradients
                            .second_layer_biases
                            .0
                            .iter_mut()
                            .zip(score_gradients.iter())
                        {
                            *second_layer_bias_gradient += score_gradient;
                        }
                        // Производные ошибки по суммам первого слоя ДО применения tanh.
                        let first_layer_pre_activation_gradients: [f32; FIRST_LAYER_SIZE] = {
                            let mut first_layer_pre_activation_gradients: [f32; FIRST_LAYER_SIZE] =
                                [ZERO_F32; FIRST_LAYER_SIZE];
                            for first_layer_index in 0..FIRST_LAYER_SIZE {
                                for vocabulary_character_index in 0..vocabulary_len {
                                    gradients.second_layer_weights.0[vocabulary_character_index]
                                        [first_layer_index] +=
                                        forward_result.first_layer_activations[first_layer_index].0
                                            * score_gradients[vocabulary_character_index];
                                    first_layer_pre_activation_gradients[first_layer_index] += model
                                        .second_layer_weights[vocabulary_character_index]
                                        [first_layer_index]
                                        .0
                                        * score_gradients[vocabulary_character_index];
                                }
                                // Производная tanh равна 1 - tanh².
                                first_layer_pre_activation_gradients[first_layer_index] *= 1.0
                                    - forward_result.first_layer_activations[first_layer_index].0
                                        * forward_result.first_layer_activations[first_layer_index]
                                            .0;
                                gradients.first_layer_biases[first_layer_index] +=
                                    first_layer_pre_activation_gradients[first_layer_index];
                            }
                            first_layer_pre_activation_gradients
                        };
                        for input_coordinate_index in
                            0..forward_result.character_embeddings_flattened.len()
                        {
                            let input_coordinate_gradient: f32 = {
                                let mut input_coordinate_gradient: f32 = ZERO_F32;
                                for first_layer_index in 0..FIRST_LAYER_SIZE {
                                    gradients.first_layer_weights[input_coordinate_index]
                                        [first_layer_index] += forward_result
                                        .character_embeddings_flattened[input_coordinate_index]
                                        .0
                                        * first_layer_pre_activation_gradients[first_layer_index];
                                    input_coordinate_gradient += model.first_layer_weights
                                        [input_coordinate_index][first_layer_index]
                                        .0
                                        * first_layer_pre_activation_gradients[first_layer_index];
                                }
                                input_coordinate_gradient
                            };
                            // Scatter-add: повторяющиеся символы суммируют градиенты одной строки C.
                            gradients.character_embeddings.0[example.vocabulary_context_indices
                                [input_coordinate_index / EMBEDDING_DIMENSIONS]
                                .0][input_coordinate_index % EMBEDDING_DIMENSIONS] +=
                                input_coordinate_gradient;
                        }
                    }
                    // Готовые градиенты проверяем до изменения любого параметра модели.
                    let gradients: ModelGradients<FiniteF32> = ModelGradients {
                        character_embeddings: gradients
                            .character_embeddings
                            .try_map(|row| try_map_array(*row, FiniteF32::try_from))?,
                        first_layer_weights: try_map_array(gradients.first_layer_weights, |row| {
                            try_map_array(row, FiniteF32::try_from)
                        })?,
                        first_layer_biases: try_map_array(
                            gradients.first_layer_biases,
                            FiniteF32::try_from,
                        )?,
                        second_layer_weights: gradients
                            .second_layer_weights
                            .try_map(|row| try_map_array(*row, FiniteF32::try_from))?,
                        second_layer_biases: gradients
                            .second_layer_biases
                            .try_map(|&value| FiniteF32::try_from(value))?,
                    };
                    (total_loss / training_batch.len() as f32, gradients)
                };
                for (parameter_tensor, gradient_tensor) in [
                    (
                        model.character_embeddings.0.as_flattened_mut(),
                        gradients.character_embeddings.as_flattened(),
                    ),
                    (
                        model.first_layer_weights.as_flattened_mut(),
                        gradients.first_layer_weights.as_flattened(),
                    ),
                    (
                        &mut model.first_layer_biases[..],
                        &gradients.first_layer_biases[..],
                    ),
                    (
                        model.second_layer_weights.0.as_flattened_mut(),
                        gradients.second_layer_weights.as_flattened(),
                    ),
                    (
                        &mut model.second_layer_biases.0[..],
                        &gradients.second_layer_biases[..],
                    ),
                ] {
                    for (parameter, &gradient) in parameter_tensor.iter_mut().zip(gradient_tensor) {
                        *parameter =
                            FiniteF32::try_from(parameter.0 - learning_rate.0 * gradient.0)?;
                    }
                }
                CrossEntropyLossAsNonNegativeF32::try_from(loss)?
            };
            if step_index % 10_000 == 0 || step_index + 1 == training_steps.get() {
                println!(
                    "step={step_index:6} lr={learning_rate} loss={loss:.4} elapsed={:.1}s iteration_ms={:.3}",
                    training_started_at.elapsed().as_secs_f32(),
                    iteration_started_at.elapsed().as_secs_f64() * 1000.0,
                    loss = loss.0,
                    learning_rate = learning_rate.0,
                );
            }
        }
        model
    };
    // 4. Считаем средние loss на train/dev/test без обновления параметров и печатаем метрики.
    // Dev служит для выбора настроек, test — для окончательной оценки после их выбора.
    {
        let metrics_text: String = format!(
            "steps={training_steps}\nparameters={trainable_scalar_count}\ninitial_dev_loss={initial_validation_loss:.6}\ntrain_loss={training_loss:.6}\ndev_loss={validation_loss:.6}\ntest_loss={test_loss:.6}\n",
            initial_validation_loss = initial_validation_loss.0,
            training_loss = loss(&model, NonEmptySlice::from(&training_examples))?.0,
            validation_loss = loss(&model, NonEmptySlice::from(&validation_examples))?.0,
            test_loss = loss(&model, NonEmptySlice::from(&test_examples))?.0,
        );
        print!("{metrics_text}");
    }
    // 5. Генерируем имена с общим генератором: каждый выбранный символ добавляем в контекст до точки или лимита.
    for _ in 0..20 {
        let name: String = 'sample: {
            let mut vocabulary_context_indices: [VocabularyCharacterIndexAsUsize; CONTEXT_LENGTH] =
                [vocabulary.dot_index(); CONTEXT_LENGTH];
            let mut generated_name: String = String::new();
            for _ in 0..100 {
                let (_, character_probabilities) = cross_entropy(
                    NonEmptySlice::from(
                        &forward(&model, vocabulary_context_indices)?.character_scores,
                    ),
                    vocabulary.dot_index(),
                )?;
                let random_probability: CharacterProbabilityAsF32From0To1 =
                    CharacterProbabilityAsF32From0To1::try_from(rand::RngExt::random::<f32>(
                        &mut random_generator,
                    ))?;
                let sampled_character_index: usize = {
                    let mut cumulative_probability: f32 = ZERO_F32;
                    let mut sampled_character_index: usize = vocabulary_len - 1;
                    for (character_index, character_probability) in
                        character_probabilities.iter().enumerate()
                    {
                        cumulative_probability += character_probability.0;
                        if random_probability.0 < cumulative_probability {
                            sampled_character_index = character_index;
                            break;
                        }
                    }
                    sampled_character_index
                };
                let sampled_character_index: VocabularyCharacterIndexAsUsize =
                    VocabularyCharacterIndexAsUsize::try_from((
                        sampled_character_index,
                        &vocabulary,
                    ))?;
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
    }
    println!(
        "Общее время работы программы: {:.2} с",
        program_started_at.elapsed().as_secs_f64()
    );
    Ok(())
}
