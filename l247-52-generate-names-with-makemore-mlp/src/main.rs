fn main() -> Result<(), Box<dyn std::error::Error>> {
    let program_started_at: std::time::Instant = std::time::Instant::now();
    /// Владеющий вектор, содержащий хотя бы один элемент.
    /// TryFrom отклоняет пустой вектор; map и try_map сохраняют длину. Deref предоставляет только срез.
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
            .map_err(|_: Vec<U>| -> &'static str {
                "при преобразовании изменилась длина массива"
            })
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
                .filter(|character: &char| -> bool { character.is_ascii_lowercase() })
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
                .position(|&candidate: &char| -> bool { candidate == character })
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
    // Числовой индекс символа Vocabulary; TryFrom проверяет границы конкретного словаря.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct VocabularyCharacterIndexAsUsize(
        // Значение от 0 до vocabulary.len() - 1 после TryFrom; сам usize не хранит связь со словарём.
        usize,
    );
    impl TryFrom<(usize, &Vocabulary)> for VocabularyCharacterIndexAsUsize {
        type Error = &'static str;
        // Создаёт индекс, только если value < vocabulary.len().
        fn try_from((value, vocabulary): (usize, &Vocabulary)) -> Result<Self, Self::Error> {
            vocabulary
                .0
                .get(value)
                .map(|_: &char| -> Self { Self(value) })
                .ok_or("индекс символа должен быть меньше размера словаря")
        }
    }
    // Сколько предыдущих символов получает модель; значение должно быть положительным.
    const CONTEXT_LENGTH: usize = 3;
    // Обучающий пример в индексах Vocabulary: контекст длины CONTEXT_LENGTH и правильный следующий символ.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct PreviousVocabularyCharacterIndexesAndNextCharacterIndex {
        // Индексы предыдущих CONTEXT_LENGTH символов в Vocabulary; индекс точки заполняет начало имени.
        previous_vocabulary_character_usize_indexes:
            [VocabularyCharacterIndexAsUsize; CONTEXT_LENGTH],
        // Индекс правильного следующего символа в Vocabulary; индекс точки обозначает конец имени.
        next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize,
    }
    // Нулевое значение для координат, градиентов и накопительных сумм.
    const ZERO_F32: f32 = 0.0;
    // Число обучаемых координат одного символа; определяет длину каждой строки C.
    const EMBEDDING_DIMENSIONS: usize = 10;
    // Число компонентов результата первого слоя и длина b1.
    const FIRST_LAYER_SIZE: usize = 200;
    // Число примеров, по которым усредняются градиенты одного шага.
    const BATCH_SIZE: usize = 32;
    // Проверяем положительность размеров при компиляции.
    const _: () = assert!(
        CONTEXT_LENGTH > 0 && EMBEDDING_DIMENSIONS > 0 && FIRST_LAYER_SIZE > 0 && BATCH_SIZE > 0
    );
    // Положительное число обновлений параметров; для нуля возвращается ошибка.
    let training_steps: std::num::NonZeroUsize =
        std::num::NonZeroUsize::new(200_000).ok_or("число шагов должно быть положительным")?;
    // Один seed и общий генератор для всех случайных операций программы.
    let mut random_generator: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(42);
    let (
        vocabulary,
        training_character_index_pairs,
        validation_character_index_pairs,
        test_character_index_pairs,
    ): (
        Vocabulary,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
    ) = {
        // 1. Проверяем записи; повторения сохраняем, чтобы не менять их частоты в наборе.
        let names_text: &str = include_str!("../data/names.txt");
        // Одно непустое имя из строчных латинских букв; заимствует исходный текст.
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        struct Name<'a>(&'a str);
        impl<'a> TryFrom<&'a str> for Name<'a> {
            type Error = &'static str;
            fn try_from(text: &'a str) -> Result<Self, Self::Error> {
                if text.is_empty()
                    || !text
                        .chars()
                        .all(|character: char| -> bool { character.is_ascii_lowercase() })
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
            for iteration_value in names.iter() {
                let &name: &Name<'_> = iteration_value;
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
                .flat_map(
                    |group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> {
                        group.iter()
                    },
                )
                .copied()
                .collect::<Vec<_>>(),
        )?;
        let validation_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            validation_name_groups
                .iter()
                .flat_map(
                    |group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> {
                        group.iter()
                    },
                )
                .copied()
                .collect::<Vec<_>>(),
        )?;
        let test_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            test_name_groups
                .iter()
                .flat_map(
                    |group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> {
                        group.iter()
                    },
                )
                .copied()
                .collect::<Vec<_>>(),
        )?;
        // Состав Vocabulary определяем только по train. Неизвестный символ в dev/test вернёт ошибку.
        let vocabulary: Vocabulary = Vocabulary::new(
            &training_names
                .map(|name: &Name<'_>| -> &str { name.0 })
                .join("\n"),
        )?;
        println!("Словарь (только train): {:?}", vocabulary.0);
        // Захватывает Vocabulary: для каждого имени сбрасывает контекст и строит пары «контекст → следующий символ».
        // Вход и результат непустые; точка в конце учит завершать имя, неизвестный символ возвращает ошибку.
        let build_previous_vocabulary_character_indexes_and_next_character_indexes =
            |names: NonEmptySlice<'_, Name<'_>>| -> Result<
                NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
                &'static str,
            > {
                let mut character_index_pairs: Vec<
                    PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
                > = Vec::new();
                for iteration_value in names.iter() {
                    let name: &Name<'_> = iteration_value;
                    let mut previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize;
                    CONTEXT_LENGTH] = [vocabulary.dot_index(); CONTEXT_LENGTH];
                    for iteration_value in name.0.chars().chain(std::iter::once('.')) {
                        let next_character: char = iteration_value;
                        let next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize =
                            vocabulary.index_of(next_character)?;
                        character_index_pairs.push(
                            PreviousVocabularyCharacterIndexesAndNextCharacterIndex {
                                previous_vocabulary_character_usize_indexes,
                                next_vocabulary_character_usize_index,
                            },
                        );
                        previous_vocabulary_character_usize_indexes.rotate_left(1);
                        previous_vocabulary_character_usize_indexes[CONTEXT_LENGTH - 1] =
                            next_vocabulary_character_usize_index;
                    }
                }
                NonEmptyVec::try_from(character_index_pairs)
            };
        let training_character_index_pairs: NonEmptyVec<
            PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
        > = build_previous_vocabulary_character_indexes_and_next_character_indexes(
            NonEmptySlice::from(&training_names),
        )?;
        let validation_character_index_pairs: NonEmptyVec<
            PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
        > = build_previous_vocabulary_character_indexes_and_next_character_indexes(
            NonEmptySlice::from(&validation_names),
        )?;
        let test_character_index_pairs: NonEmptyVec<
            PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
        > = build_previous_vocabulary_character_indexes_and_next_character_indexes(
            NonEmptySlice::from(&test_names),
        )?;
        println!(
            "Имена train/dev/test: {}/{}/{}; примеры: {}/{}/{}",
            training_names.len(),
            validation_names.len(),
            test_names.len(),
            training_character_index_pairs.len(),
            validation_character_index_pairs.len(),
            test_character_index_pairs.len()
        );
        (
            vocabulary,
            training_character_index_pairs,
            validation_character_index_pairs,
            test_character_index_pairs,
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
        // Разрешает отрицательные числа, ноль и положительные числа; отклоняет NaN и ±∞.
        fn try_from(value: f32) -> Result<Self, Self::Error> {
            if value.is_finite() {
                Ok(Self(value))
            } else {
                Err("значение должно быть конечным числом")
            }
        }
    }
    // Множитель градиента при обновлении параметров; допускается только конечное число строго больше нуля.
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
    // Результат cross-entropy для одного правильного следующего символа.
    struct NextCharacterProbabilitiesAndNegativeLogLikelihood {
        // Конечная неотрицательная ошибка: −ln(вероятности правильного символа).
        negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32,
        // Вероятности всех символов в порядке Vocabulary.
        next_character_probabilities: NonEmptyVec<CharacterProbabilityAsF32From0To1>,
    }
    // Для одного примера возвращает ошибку и вероятности символов.
    // loss — ошибка предсказания: -ln(вероятность правильного следующего символа).
    let cross_entropy = |character_scores: NonEmptySlice<'_, FiniteF32>,
                         next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize|
     -> Result<
        NextCharacterProbabilitiesAndNegativeLogLikelihood,
        &'static str,
    > {
        // Softmax: превращаем оценки символов в вероятности.
        // Максимум и сумму экспонент передаём в NLL для численно устойчивого расчёта.
        let (
            next_character_probabilities,
            maximum_character_score,
            exponentials_from_character_scores_minus_maximum_sum,
        ): (NonEmptyVec<CharacterProbabilityAsF32From0To1>, f32, f32) = {
            // Находим максимум непустого набора конечных оценок; −∞ служит только начальным накопителем fold.
            let maximum_character_score: f32 = character_scores
                .iter()
                .map(|score: &FiniteF32| -> f32 { score.0 })
                .fold(f32::NEG_INFINITY, f32::max);
            // Из каждой оценки вычитаем максимум: получаем значения ≤ 0, у максимумов — 0.
            // exp(оценка − максимум) даёт значения от 0 до 1; очень малые могут округлиться до 0.
            let exponentials_from_character_scores_minus_maximum: NonEmptyVec<f32> =
                NonEmptyVec::try_from(
                    character_scores
                        .iter()
                        .map(|character_score: &FiniteF32| -> f32 {
                            (character_score.0 - maximum_character_score).exp()
                        })
                        .collect::<Vec<_>>(),
                )?;
            // Сумма не меньше 1: хотя бы одна максимальная оценка даёт exp(0) = 1.
            let exponentials_from_character_scores_minus_maximum_sum: f32 =
                exponentials_from_character_scores_minus_maximum
                    .iter()
                    .sum();
            // Softmax: делим каждую экспоненту на общую сумму, сохраняя порядок символов Vocabulary.
            let next_character_probabilities: NonEmptyVec<CharacterProbabilityAsF32From0To1> =
                NonEmptyVec::try_from(
                    exponentials_from_character_scores_minus_maximum
                        .into_iter()
                        .map(|value: f32| -> Result<CharacterProbabilityAsF32From0To1, &'static str> {
                            CharacterProbabilityAsF32From0To1::try_from(
                                value / exponentials_from_character_scores_minus_maximum_sum,
                            )
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )?;
            (
                next_character_probabilities,
                maximum_character_score,
                exponentials_from_character_scores_minus_maximum_sum,
            )
        };
        // Negative log-likelihood (NLL): −ln вероятности правильного следующего символа.
        let negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32 = {
            // Берём исходную оценку правильного следующего символа, до вычитания максимума.
            let target_character_score: f32 = character_scores
                .get(next_vocabulary_character_usize_index.0)
                .ok_or("индекс целевого символа выходит за границы оценок")?
                .0;
            // negative_log_likelihood_loss = −ln(p правильного символа).
            // p = exp(target_character_score − maximum_character_score) / сумма экспонент.
            // Раскрывая −ln(p), получаем максимум − целевую оценку + ln(сумма).
            // Считаем напрямую: крошечная вероятность может округлиться до 0, и ln(0) дал бы −∞.
            let negative_log_likelihood_loss: f32 = (maximum_character_score
                - target_character_score)
                + exponentials_from_character_scores_minus_maximum_sum.ln();
            CrossEntropyLossAsNonNegativeF32::try_from(negative_log_likelihood_loss)?
        };
        Ok(NextCharacterProbabilitiesAndNegativeLogLikelihood {
            negative_log_likelihood_loss,
            next_character_probabilities,
        })
    };
    // Параметры MLP: таблица эмбеддингов и два полносвязных слоя, между которыми применяется tanh.
    // Прямой проход и loss — замыкания ниже; размер словаря хранится в локальной переменной vocabulary_len.
    #[derive(Clone)]
    struct Model {
        // C: одна строка из EMBEDDING_DIMENSIONS координат на каждый символ Vocabulary.
        character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
        // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, FIRST_LAYER_SIZE) — веса первого слоя.
        first_layer_weights: [[FiniteF32; FIRST_LAYER_SIZE]; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
        // b1: одно обучаемое смещение на каждую компоненту первого слоя.
        first_layer_biases: [FiniteF32; FIRST_LAYER_SIZE],
        // W2: [индекс символа][индекс компоненты первого слоя]; одна строка на символ Vocabulary.
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
        let initialize_embeddings_regular_cube = || -> NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> {
            if vocabulary_len < 2 {
                return vocabulary.0.map(|_: &char| -> [f32; EMBEDDING_DIMENSIONS] {
                    [ZERO_F32; EMBEDDING_DIMENSIONS]
                });
            }
            let grid_dimensions: usize = EMBEDDING_DIMENSIONS.min(3);
            let mut levels_per_axis: usize = 1;
            while levels_per_axis.pow(grid_dimensions as u32) < vocabulary_len {
                levels_per_axis += 1;
            }
            let node_count: usize = levels_per_axis.pow(grid_dimensions as u32);
            let half_width: f32 = 3.0f32.sqrt();
            let mut symbol_index: usize = 0;
            vocabulary.0.map(|_: &char| -> [f32; EMBEDDING_DIMENSIONS] {
                // Если число символов не заполняет решётку, выбираем узлы равномерно
                // по её линейной нумерации, включая оба крайних узла; случайности нет.
                let mut node_index: usize = symbol_index * (node_count - 1) / (vocabulary_len - 1);
                let mut embedding: [f32; EMBEDDING_DIMENSIONS] = [ZERO_F32; EMBEDDING_DIMENSIONS];
                for iteration_value in &mut embedding[..grid_dimensions] {
                    let coordinate: &mut f32 = iteration_value;
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
        let normal_character_embeddings: NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> =
            vocabulary.0.map(|_: &char| -> [f32; EMBEDDING_DIMENSIONS] {
                std::array::from_fn(|_: usize| -> f32 {
                    initialize_embeddings_sample_standard_normal()
                })
            });
        let character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]> = {
            let embeddings: NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> = if false {
                initialize_embeddings_regular_cube()
            } else {
                normal_character_embeddings
            };
            embeddings.try_map(|row: &[f32; EMBEDDING_DIMENSIONS]| -> Result<[FiniteF32; EMBEDDING_DIMENSIONS], &'static str> { try_map_array(*row, FiniteF32::try_from) })?
        };
        // W1: (CONTEXT_LENGTH * EMBEDDING_DIMENSIONS, FIRST_LAYER_SIZE) — массив фиксированной длины.
        let first_layer_weights: [[FiniteF32; FIRST_LAYER_SIZE];
            CONTEXT_LENGTH * EMBEDDING_DIMENSIONS] = try_map_array(
            std::array::from_fn(|_: usize| -> [f32; FIRST_LAYER_SIZE] {
                std::array::from_fn(|_: usize| -> f32 {
                    initialize_embeddings_sample_standard_normal()
                })
            }),
            |row: [f32; FIRST_LAYER_SIZE]| -> Result<[FiniteF32; FIRST_LAYER_SIZE], &'static str> {
                try_map_array(row, FiniteF32::try_from)
            },
        )?;
        // b1: FIRST_LAYER_SIZE случайных смещений первого слоя, проверенных на конечность.
        let first_layer_biases: [FiniteF32; FIRST_LAYER_SIZE] = try_map_array(
            std::array::from_fn(|_: usize| -> f32 {
                initialize_embeddings_sample_standard_normal()
            }),
            FiniteF32::try_from,
        )?;
        // W2: для каждого символа — FIRST_LAYER_SIZE связей с первым слоем.
        let mut second_layer_weights: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]> = {
            let zero_weight: FiniteF32 = FiniteF32::try_from(ZERO_F32)?;
            vocabulary
                .0
                .map(|_: &char| -> [FiniteF32; FIRST_LAYER_SIZE] {
                    [zero_weight; FIRST_LAYER_SIZE]
                })
        };
        // Сначала перебираем компоненты первого слоя, затем символы: порядок случайных чисел сохраняется.
        for first_layer_index in 0..FIRST_LAYER_SIZE {
            for iteration_value in second_layer_weights.0.iter_mut() {
                let character_weights: &mut [FiniteF32; FIRST_LAYER_SIZE] = iteration_value;
                character_weights[first_layer_index] =
                    FiniteF32::try_from(initialize_embeddings_sample_standard_normal())?;
            }
        }
        // b2: (vocabulary_len,) — смещения оценок символов, размер зависит от словаря.
        let second_layer_biases: NonEmptyVec<FiniteF32> = NonEmptyVec::try_from(
            (0..vocabulary_len)
                .map(|_: usize| -> Result<FiniteF32, &'static str> {
                    FiniteF32::try_from(initialize_embeddings_sample_standard_normal())
                })
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
        // FIRST_LAYER_SIZE результатов tanh; каждый находится в диапазоне от −1 до 1.
        first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE],
        // Оценки следующих символов до softmax, в порядке Vocabulary.
        character_scores: NonEmptyVec<FiniteF32>,
    }
    // Вычисляет оценки следующего символа и сохраняет промежуточные значения прямого прохода.
    let forward_pass = |model: &Model,
                        previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize;
                            CONTEXT_LENGTH]|
     -> Result<ForwardPassResult, &'static str> {
        // Берём векторы символов контекста из C и объединяем в исходном порядке.
        // Длина входного вектора: CONTEXT_LENGTH * EMBEDDING_DIMENSIONS.
        let character_embeddings_flattened: [FiniteF32; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS] = {
            let context_embeddings: [[FiniteF32; EMBEDDING_DIMENSIONS]; CONTEXT_LENGTH] =
                previous_vocabulary_character_usize_indexes.map(
                    |index: VocabularyCharacterIndexAsUsize| -> [FiniteF32; EMBEDDING_DIMENSIONS] {
                        model.character_embeddings[index.0]
                    },
                );
            context_embeddings.as_flattened().try_into().map_err(
                |_: std::array::TryFromSliceError| -> &'static str {
                    "неверная длина объединённых эмбеддингов"
                },
            )?
        };
        // Первый слой: начинаем с b1, накапливаем вход × W1, затем применяем tanh.
        let first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE] = {
            let mut first_layer_activations: [f32; FIRST_LAYER_SIZE] = model
                .first_layer_biases
                .map(|value: FiniteF32| -> f32 { value.0 });
            for iteration_value in character_embeddings_flattened.iter().enumerate() {
                let (context_embedding_coordinate_index, &context_embedding_coordinate): (
                    usize,
                    &FiniteF32,
                ) = iteration_value;
                for iteration_value in first_layer_activations
                    .iter_mut()
                    .zip(&model.first_layer_weights[context_embedding_coordinate_index])
                {
                    let (first_layer_activation, &first_layer_weight): (&mut f32, &FiniteF32) =
                        iteration_value;
                    *first_layer_activation +=
                        context_embedding_coordinate.0 * first_layer_weight.0;
                }
            }
            for iteration_value in &mut first_layer_activations {
                let first_layer_activation: &mut f32 = iteration_value;
                *first_layer_activation = FiniteF32::try_from(*first_layer_activation)?.0.tanh();
            }
            try_map_array(first_layer_activations, FiniteF32::try_from)?
        };
        // Второй слой: b2 + сумма(активация первого слоя × W2); здесь ещё нет softmax.
        let character_scores: NonEmptyVec<f32> = {
            let mut character_scores: NonEmptyVec<f32> = model
                .second_layer_biases
                .map(|value: &FiniteF32| -> f32 { value.0 });
            for iteration_value in first_layer_activations.iter().enumerate() {
                let (first_layer_index, &first_layer_activation): (usize, &FiniteF32) =
                    iteration_value;
                for iteration_value in character_scores
                    .0
                    .iter_mut()
                    .zip(model.second_layer_weights.iter())
                {
                    let (character_score, character_weights): (
                        &mut f32,
                        &[FiniteF32; FIRST_LAYER_SIZE],
                    ) = iteration_value;
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
    // Усредняет cross-entropy по непустому набору; захватывает forward_pass и cross_entropy, модель не изменяет.
    let loss = move |model: &Model,
                     character_index_pairs: NonEmptySlice<
        '_,
        PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
    >|
          -> Result<CrossEntropyLossAsNonNegativeF32, &'static str> {
        let mut total_negative_log_likelihood_loss: f64 = 0.0;
        for iteration_value in character_index_pairs.iter() {
            let character_index_pair: &PreviousVocabularyCharacterIndexesAndNextCharacterIndex =
                iteration_value;
            let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood = cross_entropy(
                NonEmptySlice::from(
                    &forward_pass(
                        model,
                        character_index_pair.previous_vocabulary_character_usize_indexes,
                    )?
                    .character_scores,
                ),
                character_index_pair.next_vocabulary_character_usize_index,
            )?;
            total_negative_log_likelihood_loss += f64::from(
                next_character_probabilities_and_negative_log_likelihood
                    .negative_log_likelihood_loss
                    .0,
            );
        }
        CrossEntropyLossAsNonNegativeF32::try_from(
            (total_negative_log_likelihood_loss / character_index_pairs.len() as f64) as f32,
        )
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
    let initial_validation_loss: CrossEntropyLossAsNonNegativeF32 = loss(
        &model,
        NonEmptySlice::from(&validation_character_index_pairs),
    )?;
    println!("До обучения dev loss={:.6}", initial_validation_loss.0);
    // 3. Последовательно берём непересекающиеся батчи train; после конца начинаем новый проход в том же порядке.
    let model: Model = {
        let mut model: Model = model;
        let mut batch_start: usize = 0;
        let training_started_at: std::time::Instant = std::time::Instant::now();
        for training_step in 0..training_steps.get() {
            let iteration_started_at: std::time::Instant = std::time::Instant::now();
            let learning_rate: LearningRateAsPositiveFiniteF32 =
                LearningRateAsPositiveFiniteF32::try_from(if training_step < 100_000 {
                    0.1
                } else {
                    0.01
                })?;
            let loss: CrossEntropyLossAsNonNegativeF32 = {
                // Формы совпадают с Model; каждый градиент проверяется при изменении.
                struct ModelGradients {
                    // Производные среднего loss по координатам символов C.
                    character_embeddings_gradients: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
                    // Производные по W1: [координата объединённых эмбеддингов][компонента первого слоя].
                    first_layer_weights_gradients:
                        [[FiniteF32; FIRST_LAYER_SIZE]; CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
                    // Производные по каждому смещению первого слоя b1.
                    first_layer_biases_gradients: [FiniteF32; FIRST_LAYER_SIZE],
                    // Производные по W2: [индекс символа Vocabulary][компонента первого слоя].
                    second_layer_weights_gradients: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
                    // Производные по каждому смещению оценок символов b2.
                    second_layer_biases_gradients: NonEmptyVec<FiniteF32>,
                }
                let (loss, model_gradients): (f32, ModelGradients) = {
                    // Последний батч эпохи может быть короче BATCH_SIZE: используем все примеры.
                    let batch_end: usize =
                        (batch_start + BATCH_SIZE).min(training_character_index_pairs.len());
                    let previous_vocabulary_character_indexes_and_next_character_index_batch: NonEmptySlice<
                        '_,
                        PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
                    > = NonEmptySlice::try_from(
                        &training_character_index_pairs[batch_start..batch_end],
                    )?;
                    batch_start = if batch_end == training_character_index_pairs.len() {
                        0
                    } else {
                        batch_end
                    };
                    let zero_gradient: FiniteF32 = FiniteF32::try_from(ZERO_F32)?;
                    let mut model_gradients: ModelGradients = ModelGradients {
                        character_embeddings_gradients: model
                            .character_embeddings
                            .map(|_: &[FiniteF32; EMBEDDING_DIMENSIONS]| -> [FiniteF32; EMBEDDING_DIMENSIONS] { [zero_gradient; EMBEDDING_DIMENSIONS] }),
                        first_layer_weights_gradients: [[zero_gradient; FIRST_LAYER_SIZE];
                            CONTEXT_LENGTH * EMBEDDING_DIMENSIONS],
                        first_layer_biases_gradients: [zero_gradient; FIRST_LAYER_SIZE],
                        second_layer_weights_gradients: model
                            .second_layer_weights
                            .map(|_: &[FiniteF32; FIRST_LAYER_SIZE]| -> [FiniteF32; FIRST_LAYER_SIZE] { [zero_gradient; FIRST_LAYER_SIZE] }),
                        second_layer_biases_gradients: model
                            .second_layer_biases
                            .map(|_: &FiniteF32| -> FiniteF32 { zero_gradient }),
                    };
                    let mut total_negative_log_likelihood_loss: FiniteF32 = zero_gradient;
                    for previous_vocabulary_character_indexes_and_next_character_index_batch_element in
                        previous_vocabulary_character_indexes_and_next_character_index_batch.iter()
                    {
                        // Производная по оценке правильного символа: p − 1, затем деление на размер батча.
                        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
                        struct TargetScoreGradientAsF32FromMinus1To0(f32);
                        impl TryFrom<f32> for TargetScoreGradientAsF32FromMinus1To0 {
                            type Error = &'static str;
                            // Проверяет замкнутый диапазон [−1, 0], отклоняя NaN и бесконечности.
                            fn try_from(value: f32) -> Result<Self, Self::Error> {
                                if (-1.0..=ZERO_F32).contains(&value) {
                                    Ok(Self(value))
                                } else {
                                    Err("градиент правильного символа должен быть от −1 до 0")
                                }
                            }
                        }
                        // Различает градиент правильного символа и неотрицательные градиенты остальных.
                        #[derive(Clone, Copy, Debug)]
                        enum NextCharacterScoreGradientAsF32FromMinus1To1 {
                            Target(TargetScoreGradientAsF32FromMinus1To0),
                            // Используем проверенный диапазон [0, 1]; здесь значение уже является градиентом.
                            Other(CharacterProbabilityAsF32From0To1),
                        }
                        impl From<NextCharacterScoreGradientAsF32FromMinus1To1> for f32 {
                            // Извлекает числовую производную для расчёта градиентов параметров модели.
                            fn from(
                                gradient: NextCharacterScoreGradientAsF32FromMinus1To1,
                            ) -> Self {
                                match gradient {
                                    NextCharacterScoreGradientAsF32FromMinus1To1::Target(value) => {
                                        value.0
                                    }
                                    NextCharacterScoreGradientAsF32FromMinus1To1::Other(value) => {
                                        value.0
                                    }
                                }
                            }
                        }
                        let forward_pass_result: ForwardPassResult = forward_pass(
                            &model,
                            previous_vocabulary_character_indexes_and_next_character_index_batch_element
                                .previous_vocabulary_character_usize_indexes,
                        )?;
                        let next_character_score_gradients_as_f32_from_minus1_to1: NonEmptyVec<
                            NextCharacterScoreGradientAsF32FromMinus1To1,
                        > = {
                            let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood = cross_entropy(
                                NonEmptySlice::from(&forward_pass_result.character_scores),
                                previous_vocabulary_character_indexes_and_next_character_index_batch_element
                                    .next_vocabulary_character_usize_index,
                            )?;
                            total_negative_log_likelihood_loss = FiniteF32::try_from(
                                total_negative_log_likelihood_loss.0
                                    + next_character_probabilities_and_negative_log_likelihood
                                        .negative_log_likelihood_loss
                                        .0,
                            )?;
                            let previous_vocabulary_character_indexes_and_next_character_index_batch_len: f32 =
                                previous_vocabulary_character_indexes_and_next_character_index_batch
                                    .len() as f32;
                            // Ровно один Target ставим на позицию правильного символа; остальные — Other.
                            // Проверяем диапазон уже усреднённой производной: (p − 1) / B либо p / B.
                            NonEmptyVec::try_from(
                                next_character_probabilities_and_negative_log_likelihood.next_character_probabilities
                                    .iter().enumerate()
                                    .map(|(next_character_probability_index, next_character_probability): (usize, &CharacterProbabilityAsF32From0To1)| -> Result<NextCharacterScoreGradientAsF32FromMinus1To1, &'static str> {
                                        if next_character_probability_index == previous_vocabulary_character_indexes_and_next_character_index_batch_element.next_vocabulary_character_usize_index.0 {
                                            Ok(NextCharacterScoreGradientAsF32FromMinus1To1::Target(
                                                TargetScoreGradientAsF32FromMinus1To0::try_from((next_character_probability.0 - 1.0) / previous_vocabulary_character_indexes_and_next_character_index_batch_len)?,
                                            ))
                                        } else {
                                            Ok(NextCharacterScoreGradientAsF32FromMinus1To1::Other(
                                                CharacterProbabilityAsF32From0To1::try_from(next_character_probability.0 / previous_vocabulary_character_indexes_and_next_character_index_batch_len)?,
                                            ))
                                        }
                                    })
                                    .collect::<Result<Vec<_>, _>>()?,
                            )?
                        };
                        // Добавляет произведение двух множителей: вход × градиент выхода либо вес × градиент выхода.
                        fn accumulate_gradient_product(
                            gradient: &mut FiniteF32,
                            input_or_weight: f32,
                            output_gradient: f32,
                            location: &'static str,
                        ) -> Result<(), &'static str> {
                            let contribution: FiniteF32 = FiniteF32::try_from(input_or_weight * output_gradient)
                                .map_err(|_: &'static str| -> &'static str { location })?;
                            *gradient = FiniteF32::try_from(gradient.0 + contribution.0)
                                .map_err(|_: &'static str| -> &'static str { location })?;
                            Ok(())
                        }
                        // Производная по смещению равна производной по выходной оценке: множитель 1 не нужен.
                        fn accumulate_bias_gradient(
                            bias_gradient: &mut FiniteF32,
                            output_gradient: f32,
                            location: &'static str,
                        ) -> Result<(), &'static str> {
                            *bias_gradient = FiniteF32::try_from(bias_gradient.0 + output_gradient)
                                .map_err(|_: &'static str| -> &'static str { location })?;
                            Ok(())
                        }
                        for (
                            second_layer_bias_gradient,
                            &next_character_score_gradient_as_f32_from_minus1_to1,
                        ) in model_gradients
                            .second_layer_biases_gradients
                            .0
                            .iter_mut()
                            .zip(next_character_score_gradients_as_f32_from_minus1_to1.iter())
                        {
                            accumulate_bias_gradient(
                                second_layer_bias_gradient,
                                f32::from(next_character_score_gradient_as_f32_from_minus1_to1),
                                "переполнение градиента second_layer_biases",
                            )?;
                        }
                        // Производные ошибки по суммам первого слоя ДО применения tanh.
                        let first_layer_pre_activation_gradients: [FiniteF32; FIRST_LAYER_SIZE] = {
                            let mut first_layer_pre_activation_gradients: [FiniteF32;
                                FIRST_LAYER_SIZE] = [zero_gradient; FIRST_LAYER_SIZE];
                            for first_layer_index in 0..FIRST_LAYER_SIZE {
                                for iteration_value in 0..vocabulary_len {
                                    let vocabulary_character_index: usize = iteration_value;
                                    accumulate_gradient_product(
                                        &mut model_gradients.second_layer_weights_gradients.0
                                            [vocabulary_character_index][first_layer_index],
                                        forward_pass_result.first_layer_activations
                                            [first_layer_index]
                                            .0,
                                        f32::from(
                                            next_character_score_gradients_as_f32_from_minus1_to1
                                                [vocabulary_character_index],
                                        ),
                                        "переполнение градиента second_layer_weights",
                                    )?;
                                    accumulate_gradient_product(
                                        &mut first_layer_pre_activation_gradients
                                            [first_layer_index],
                                        model.second_layer_weights[vocabulary_character_index]
                                            [first_layer_index]
                                            .0,
                                        f32::from(
                                            next_character_score_gradients_as_f32_from_minus1_to1
                                                [vocabulary_character_index],
                                        ),
                                        "переполнение градиента входа первого слоя",
                                    )?;
                                }
                                // Производная tanh равна 1 - tanh².
                                first_layer_pre_activation_gradients[first_layer_index] =
                                    FiniteF32::try_from(
                                        first_layer_pre_activation_gradients[first_layer_index].0
                                            * (1.0
                                                - forward_pass_result.first_layer_activations
                                                    [first_layer_index]
                                                    .0
                                                    * forward_pass_result.first_layer_activations
                                                        [first_layer_index]
                                                        .0),
                                    )?;
                                accumulate_bias_gradient(
                                    &mut model_gradients.first_layer_biases_gradients
                                        [first_layer_index],
                                    first_layer_pre_activation_gradients[first_layer_index].0,
                                    "переполнение градиента first_layer_biases",
                                )?;
                            }
                            first_layer_pre_activation_gradients
                        };
                        for character_embeddings_flattened_index in
                            0..forward_pass_result.character_embeddings_flattened.len()
                        {
                            let input_coordinate_gradient: FiniteF32 = {
                                let mut input_coordinate_gradient: FiniteF32 = zero_gradient;
                                for first_layer_index in 0..FIRST_LAYER_SIZE {
                                    accumulate_gradient_product(
                                        &mut model_gradients.first_layer_weights_gradients
                                            [character_embeddings_flattened_index]
                                            [first_layer_index],
                                        forward_pass_result.character_embeddings_flattened
                                            [character_embeddings_flattened_index]
                                            .0,
                                        first_layer_pre_activation_gradients[first_layer_index].0,
                                        "переполнение градиента first_layer_weights",
                                    )?;
                                    accumulate_gradient_product(
                                        &mut input_coordinate_gradient,
                                        model.first_layer_weights
                                            [character_embeddings_flattened_index]
                                            [first_layer_index]
                                            .0,
                                        first_layer_pre_activation_gradients[first_layer_index].0,
                                        "переполнение градиента координаты эмбеддинга",
                                    )?;
                                }
                                input_coordinate_gradient
                            };
                            // Повторяющиеся символы используют одну строку C: складываем вклады всех их появлений.
                            let embedding_gradient: &mut FiniteF32 = &mut model_gradients.character_embeddings_gradients.0
                                    [previous_vocabulary_character_indexes_and_next_character_index_batch_element
                                        .previous_vocabulary_character_usize_indexes
                                        [character_embeddings_flattened_index
                                            / EMBEDDING_DIMENSIONS]
                                        .0]
                                    [character_embeddings_flattened_index % EMBEDDING_DIMENSIONS];
                            *embedding_gradient = FiniteF32::try_from(
                                embedding_gradient.0 + input_coordinate_gradient.0,
                            )
                            .map_err(
                                |_: &'static str| -> &'static str {
                                    "переполнение градиента character_embeddings"
                                },
                            )?;
                        }
                    }
                    (
                        total_negative_log_likelihood_loss.0
                            / previous_vocabulary_character_indexes_and_next_character_index_batch
                                .len() as f32,
                        model_gradients,
                    )
                };
                for iteration_value in [
                    (
                        model.character_embeddings.0.as_flattened_mut(),
                        model_gradients
                            .character_embeddings_gradients
                            .as_flattened(),
                    ),
                    (
                        model.first_layer_weights.as_flattened_mut(),
                        model_gradients.first_layer_weights_gradients.as_flattened(),
                    ),
                    (
                        &mut model.first_layer_biases[..],
                        &model_gradients.first_layer_biases_gradients[..],
                    ),
                    (
                        model.second_layer_weights.0.as_flattened_mut(),
                        model_gradients
                            .second_layer_weights_gradients
                            .as_flattened(),
                    ),
                    (
                        &mut model.second_layer_biases.0[..],
                        &model_gradients.second_layer_biases_gradients[..],
                    ),
                ] {
                    let (parameter_tensor, gradient_tensor): (&mut [FiniteF32], &[FiniteF32]) =
                        iteration_value;
                    for iteration_value in parameter_tensor.iter_mut().zip(gradient_tensor) {
                        let (parameter, &gradient): (&mut FiniteF32, &FiniteF32) = iteration_value;
                        *parameter =
                            FiniteF32::try_from(parameter.0 - learning_rate.0 * gradient.0)?;
                    }
                }
                CrossEntropyLossAsNonNegativeF32::try_from(loss)?
            };
            if training_step % 10_000 == 0 || training_step + 1 == training_steps.get() {
                println!(
                    "step={training_step:6} lr={learning_rate} loss={loss:.4} elapsed={:.1}s iteration_ms={:.3}",
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
            training_loss = loss(&model, NonEmptySlice::from(&training_character_index_pairs))?.0,
            validation_loss = loss(
                &model,
                NonEmptySlice::from(&validation_character_index_pairs)
            )?
            .0,
            test_loss = loss(&model, NonEmptySlice::from(&test_character_index_pairs))?.0,
        );
        print!("{metrics_text}");
    }
    // 5. Печатаем 20 имён: выбираем символ по вероятностям, завершаем на точке или после 100 символов.
    for _ in 0..20 {
        let name: String = 'sample: {
            let mut previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize;
                CONTEXT_LENGTH] = [vocabulary.dot_index(); CONTEXT_LENGTH];
            let mut generated_name: String = String::new();
            for _ in 0..100 {
                let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood = cross_entropy(
                    NonEmptySlice::from(
                        &forward_pass(&model, previous_vocabulary_character_usize_indexes)?
                            .character_scores,
                    ),
                    vocabulary.dot_index(),
                )?;
                let random_probability: CharacterProbabilityAsF32From0To1 =
                    CharacterProbabilityAsF32From0To1::try_from(rand::RngExt::random::<f32>(
                        &mut random_generator,
                    ))?;
                // Выбираем интервал накопленной вероятности; последний индекс — запасной из-за округления суммы.
                let sampled_character_index: usize = {
                    let mut cumulative_probability: f32 = ZERO_F32;
                    let mut sampled_character_index: usize = vocabulary_len - 1;
                    for iteration_value in next_character_probabilities_and_negative_log_likelihood
                        .next_character_probabilities
                        .iter()
                        .enumerate()
                    {
                        let (character_index, character_probability): (
                            usize,
                            &CharacterProbabilityAsF32From0To1,
                        ) = iteration_value;
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
                // Точку конца имени тоже включаем в консольную строку.
                generated_name.push(vocabulary.character(sampled_character_index)?);
                if sampled_character_index.0 == 0 {
                    break 'sample generated_name;
                }
                previous_vocabulary_character_usize_indexes.rotate_left(1);
                previous_vocabulary_character_usize_indexes[CONTEXT_LENGTH - 1] =
                    sampled_character_index;
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
