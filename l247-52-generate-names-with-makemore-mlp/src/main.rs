const PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION: usize = 3;
const EMBEDDING_DIMENSIONS: usize = 8;
const FIRST_LAYER_SIZE: usize = 64;
const BATCH_SIZE: usize = 32;
const TRAINING_STEPS: usize = 125_000;
const RANDOM_SEED: u64 = 42;
const TRAINING_NAMES_PERCENT: usize = 80;
const VALIDATION_NAMES_PERCENT: usize = 10;
const INITIAL_LEARNING_RATE: f32 = 0.1;
const FINAL_LEARNING_RATE: f32 = 0.01;
const LEARNING_RATE_CHANGE_STEP: usize = 100_000;
const USE_REGULAR_CUBE_EMBEDDINGS: bool = true;
const FREEZE_EMBEDDINGS: bool = true;
const REGULAR_CUBE_HALF_WIDTH_SQUARED: f32 = 1.0;
const GENERATED_NAMES_COUNT: usize = 20;
const MAX_GENERATED_NAME_LENGTH: usize = 100;
const NAMES_TEXT: &str = include_str!("../data/names.txt");
const ZERO_F32: f32 = 0.0;
const TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION: usize = PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION * EMBEDDING_DIMENSIONS;
const _: () = assert!(PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION > 0 && EMBEDDING_DIMENSIONS > 0 && FIRST_LAYER_SIZE > 0 && BATCH_SIZE > 0);
const _: () = assert!(
    TRAINING_NAMES_PERCENT > 0
        && TRAINING_NAMES_PERCENT < 100
        && VALIDATION_NAMES_PERCENT > 0
        && VALIDATION_NAMES_PERCENT < 100
        && TRAINING_NAMES_PERCENT + VALIDATION_NAMES_PERCENT < 100
);
const _: () = assert!(REGULAR_CUBE_HALF_WIDTH_SQUARED.is_finite() && REGULAR_CUBE_HALF_WIDTH_SQUARED > ZERO_F32);
const _: () =
    assert!(INITIAL_LEARNING_RATE.is_finite() && INITIAL_LEARNING_RATE > ZERO_F32 && FINAL_LEARNING_RATE.is_finite() && FINAL_LEARNING_RATE > ZERO_F32);
const _: () = assert!(GENERATED_NAMES_COUNT > 0 && MAX_GENERATED_NAME_LENGTH > 0);
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let training_steps: std::num::NonZeroUsize = std::num::NonZeroUsize::new(TRAINING_STEPS).ok_or("число шагов должно быть положительным")?;
    let mut random_generator: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(RANDOM_SEED);
    // Для текущего names.txt и RANDOM_SEED = 42:
    // {"vocabulary_len": 27, "training_example_count": 169100, "validation_example_count": 20950, "test_example_count": 21278}
    // Размеры и индексы ниже соответствуют реальным данным и константам урока.
    // Имена в примерах, веса, вероятности, loss и градиенты иллюстративные, если явно не указан замер.
    let (vocabulary, training_previous_vocabulary_character_indexes_and_next_character_index_pairs, validation_previous_vocabulary_character_indexes_and_next_character_index_pairs, test_previous_vocabulary_character_indexes_and_next_character_index_pairs): (
        Vocabulary,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
    ) = {
        // Пример обработки короткого списка (реальный файл содержит 32033 строки).
        // {"names_text": "emma\nanna\nemma\nolivia\namelia\neva\nmila\nnina\nnoel\nmia\nava\n"}
        let names_text: &str = NAMES_TEXT;
        // {"names": ["emma", "anna", "emma", "olivia", "amelia", "eva", "mila", "nina", "noel", "mia", "ava"]}
        let names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(names_text.lines().map(Name::try_from).collect::<Result<Vec<Name<'_>>, &'static str>>()?)?;
        // Удаляем копии до перемешивания и разделения на train/dev/test.
        // BTreeSet сохраняет стабильный алфавитный порядок перед shuffle.
        // В полном датасете 29494 уникальных имени; ниже результат для короткого примера.
        // {"unique_names": ["amelia", "anna", "ava", "emma", "eva", "mia", "mila", "nina", "noel", "olivia"]}
        let mut unique_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            names
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<Name<'_>>>()
                .into_iter()
                .collect::<Vec<Name<'_>>>(),
        )?;
        println!("Имена: {} строк, {} уникальных", names.len(), unique_names.len());
        rand::seq::SliceRandom::shuffle(unique_names.0.as_mut_slice(), &mut random_generator);
        let (training_names, remaining_names): (&[Name<'_>], &[Name<'_>]) = unique_names.split_at(unique_names.len() * TRAINING_NAMES_PERCENT / 100);
        let (validation_names, test_names): (&[Name<'_>], &[Name<'_>]) =
            remaining_names.split_at(unique_names.len() * (TRAINING_NAMES_PERCENT + VALIDATION_NAMES_PERCENT) / 100 - training_names.len());
        let training_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(training_names.to_vec())?;
        let validation_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(validation_names.to_vec())?;
        let test_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(test_names.to_vec())?;
        // Реальный словарь: точка и все буквы a–z.
        // {"vocabulary": [
        //   {"character": ".", "index": 0},
        //   {"character": "a", "index": 1},
        //   {"character": "b", "index": 2},
        //   {"character": "c", "index": 3},
        //   {"character": "d", "index": 4},
        //   {"character": "e", "index": 5},
        //   {"character": "f", "index": 6},
        //   {"character": "g", "index": 7},
        //   {"character": "h", "index": 8},
        //   {"character": "i", "index": 9},
        //   {"character": "j", "index": 10},
        //   {"character": "k", "index": 11},
        //   {"character": "l", "index": 12},
        //   {"character": "m", "index": 13},
        //   {"character": "n", "index": 14},
        //   {"character": "o", "index": 15},
        //   {"character": "p", "index": 16},
        //   {"character": "q", "index": 17},
        //   {"character": "r", "index": 18},
        //   {"character": "s", "index": 19},
        //   {"character": "t", "index": 20},
        //   {"character": "u", "index": 21},
        //   {"character": "v", "index": 22},
        //   {"character": "w", "index": 23},
        //   {"character": "x", "index": 24},
        //   {"character": "y", "index": 25},
        //   {"character": "z", "index": 26}
        // ]}
        let vocabulary: Vocabulary = Vocabulary::new(&training_names.map(|name: &Name<'_>| -> &str { name.0 }).join("\n"))?;
        // {
        //   "names": ["emma"],
        //   "return_value": [
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 5},
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 5], "next_vocabulary_character_usize_index": 13},
        //     {"previous_vocabulary_character_usize_indexes": [0, 5, 13], "next_vocabulary_character_usize_index": 13},
        //     {"previous_vocabulary_character_usize_indexes": [5, 13, 13], "next_vocabulary_character_usize_index": 1}
        //   ],
        //   "omitted_elements": 1
        // }
        let build_previous_vocabulary_character_indexes_and_next_character_indexes =
            |names: NonEmptySlice<'_, Name<'_>>| -> Result<NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>, &'static str> {
                // {"character_index_pairs": []}
                let mut character_index_pairs: Vec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex> = Vec::new();
                // {"name": "emma"}
                for name in names.iter() {
                    // {"previous_vocabulary_character_usize_indexes": [0, 0, 0]}
                    let mut previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize;
                        PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION] =
                        [vocabulary.dot_index(); PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION];
                    // {"name": "emma", "next_character_sequence": ["e", "m", "m", "a", "."]}
                    for next_character in name.0.chars().chain(std::iter::once('.')) {
                        // {"next_character": "e", "next_vocabulary_character_usize_index": 5}
                        let next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize = vocabulary.index_of(next_character)?;
                        // {
                        //   "before": {"character_index_pairs": []},
                        //   "after": {
                        //     "character_index_pairs": [{"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 5}]
                        //   }
                        // }
                        character_index_pairs.push(PreviousVocabularyCharacterIndexesAndNextCharacterIndex {
                            previous_vocabulary_character_usize_indexes,
                            next_vocabulary_character_usize_index,
                        });
                        // Сдвигаем оставшиеся индексы влево, отбрасывая самый старый.
                        // Пример на более позднем шаге обработки "emma":
                        // {
                        //   "before": {"previous_vocabulary_character_usize_indexes": [5, 13, 13]},
                        //   "after": {"previous_vocabulary_character_usize_indexes": [13, 13, 13]}
                        // }
                        for previous_character_position in 0..PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION - 1 {
                            previous_vocabulary_character_usize_indexes[previous_character_position] =
                                previous_vocabulary_character_usize_indexes[previous_character_position + 1];
                        }
                        // Записываем новую букву в последнюю ячейку: контекст "emm" становится "mma".
                        // {
                        //   "next_vocabulary_character_usize_index": 1,
                        //   "before": {"previous_vocabulary_character_usize_indexes": [13, 13, 13]},
                        //   "after": {"previous_vocabulary_character_usize_indexes": [13, 13, 1]}
                        // }
                        previous_vocabulary_character_usize_indexes[PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION - 1] =
                            next_vocabulary_character_usize_index;
                    }
                }
                NonEmptyVec::try_from(character_index_pairs)
            };
        // {"training_example_count": 169100}
        let training_previous_vocabulary_character_indexes_and_next_character_index_pairs: NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex> =
            build_previous_vocabulary_character_indexes_and_next_character_indexes(NonEmptySlice::from(&training_names))?;
        // {"validation_example_count": 20950}
        let validation_previous_vocabulary_character_indexes_and_next_character_index_pairs: NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex> =
            build_previous_vocabulary_character_indexes_and_next_character_indexes(NonEmptySlice::from(&validation_names))?;
        // {"test_example_count": 21278}
        let test_previous_vocabulary_character_indexes_and_next_character_index_pairs: NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex> =
            build_previous_vocabulary_character_indexes_and_next_character_indexes(NonEmptySlice::from(&test_names))?;
        (
            vocabulary,
            training_previous_vocabulary_character_indexes_and_next_character_index_pairs,
            validation_previous_vocabulary_character_indexes_and_next_character_index_pairs,
            test_previous_vocabulary_character_indexes_and_next_character_index_pairs,
        )
    };
    // 27 символов = 26 букв a–z + точка (начальный контекст и конец имени).
    // {"vocabulary_len": 27}
    let vocabulary_len: usize = vocabulary.len();
    // Откуда берутся размеры параметров:
    // 3 — число предыдущих символов (PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION).
    // 8 — число координат одного символа (EMBEDDING_DIMENSIONS).
    // 24 = 3 × 8: объединяем три эмбеддинга в один входной вектор.
    // 64 — число элементов first_layer_outputs (FIRST_LAYER_SIZE).
    // 3, 8 и 64 выбраны как настройки модели; из датасета определяется размер словаря 27.
    //
    // character_embeddings_parameters [27, 8]: строка для каждого символа, 8 координат в строке.
    // Всего 27 × 8 = 216 чисел; индекс символа выбирает его строку.
    // first_layer_weights_parameters [24, 64]: строка для каждой входной координаты,
    // столбец для каждого элемента first_layer_outputs. В каждом из 64 столбцов 24 веса: всего 1536.
    // first_layer_biases_parameters [64]: по одному начальному значению для каждого элемента first_layer_outputs.
    // second_layer_weights_parameters [27, 64]: строка для каждого возможного следующего символа,
    // 64 веса в строке — по одному на каждый элемент first_layer_outputs. Всего 1728 весов.
    // second_layer_biases_parameters [27]: по одному смещению для оценки каждого символа.
    // Элемент массива смещений задаёт начальное значение соответствующей накапливаемой суммы.
    // Всего хранимых чисел: 216 + 1536 + 64 + 1728 + 27 = 3571.
    // При FREEZE_EMBEDDINGS = true обучаются 3355; 216 координат эмбеддингов фиксированы.
    // Градиенты имеют те же размеры: для каждого параметра нужна своя производная loss.
    // {
    //   "parameter_shapes": {
    //     "character_embeddings_parameters": [27, 8],
    //     "first_layer_weights_parameters": [24, 64],
    //     "first_layer_biases_parameters": [64],
    //     "second_layer_weights_parameters": [27, 64],
    //     "second_layer_biases_parameters": [27]
    //   },
    //   "gradient_shapes": {
    //     "character_embeddings_gradients": [27, 8],
    //     "first_layer_weights_gradients": [24, 64],
    //     "first_layer_biases_gradients": [64],
    //     "second_layer_weights_gradients": [27, 64],
    //     "second_layer_biases_gradients": [27]
    //   },
    //   "initial_gradient_value": 0.0
    // }
    // Начальная модель использует отдельную последовательность с тем же seed, как в бенчмарке.
    random_generator = rand::SeedableRng::seed_from_u64(RANDOM_SEED);
    let mut model: Model = {
        // {"distribution": "StandardNormal", "mean": 0.0, "standard_deviation": 1.0, "return_value": -0.37}
        let mut initialize_parameter_standard_normal = || -> Result<FiniteF32, &'static str> {
            FiniteF32::try_from(rand_distr::Distribution::<f32>::sample(&rand_distr::StandardNormal, &mut random_generator))
        };
        // {
        //   "vocabulary_len": 27,
        //   "return_shape": [27, 8],
        //   "first_row": [-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0],
        //   "last_row": [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        //   "branch_enabled": true
        // }
        let initialize_embeddings_regular_cube = || -> NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> {
            // {"alternative_input": {"vocabulary_len": 1}, "return_value": [[0, 0, 0, 0, 0, 0, 0, 0]]}
            if vocabulary_len < 2 {
                return vocabulary
                    .0
                    .map(|_: &VocabularyCharacter| -> [f32; EMBEDDING_DIMENSIONS] { [ZERO_F32; EMBEDDING_DIMENSIONS] });
            }
            // Используем все координаты эмбеддинга: при размере 8 строим восьмимерную решётку.
            // {"grid_dimensions": 8}
            let grid_dimensions: usize = EMBEDDING_DIMENSIONS;
            // {"levels_per_axis": 1}
            let mut levels_per_axis: usize = 1;
            // {"vocabulary_len": 27, "levels_per_axis_sequence": [1, 2], "node_counts": [1, 256]}
            while levels_per_axis.pow(grid_dimensions as u32) < vocabulary_len {
                // {"before": {"levels_per_axis": 1}, "after": {"levels_per_axis": 2}}
                levels_per_axis += 1;
            }
            // {"node_count": 256}
            // При 27 символах выбираем 27 различных вершин из 2^8 = 256.
            let node_count: usize = levels_per_axis.pow(grid_dimensions as u32);
            // {"half_width": 1.0}
            let half_width: f32 = REGULAR_CUBE_HALF_WIDTH_SQUARED.sqrt();
            // {"symbol_index": 0}
            let mut symbol_index: usize = 0;
            vocabulary.0.map(|_: &VocabularyCharacter| -> [f32; EMBEDDING_DIMENSIONS] {
                // Равномерно распределяем номера вершин, а не расстояния между точками.
                // Это не максимизация минимального расстояния: некоторые вершины отличаются одной координатой.
                // {"symbol_index": 0, "node_count": 256, "vocabulary_len": 27, "node_index": 0}
                let mut node_index: usize = symbol_index * (node_count - 1) / (vocabulary_len - 1);
                // {"embedding": [0, 0, 0, 0, 0, 0, 0, 0]}
                let mut embedding: [f32; EMBEDDING_DIMENSIONS] = [ZERO_F32; EMBEDDING_DIMENSIONS];
                // {"embedding_after_loop": [-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]}
                for coordinate in &mut embedding {
                    // {"node_index": 0, "levels_per_axis": 2, "level": 0}
                    let level: usize = node_index % levels_per_axis;
                    // {"before": {"node_index": 0}, "after": {"node_index": 0}}
                    node_index /= levels_per_axis;
                    // {"level": 0, "levels_per_axis": 2, "half_width": 1.0, "coordinate": -1.0}
                    *coordinate = -half_width + 2.0 * half_width * level as f32 / (levels_per_axis - 1) as f32;
                }
                // {"before": {"symbol_index": 0}, "after": {"symbol_index": 1}}
                symbol_index += 1;
                embedding
            })
        };
        let zero_as_finite_f32 = FiniteF32::zero();
        Model {
            character_embeddings_parameters: {
                // {"normal_character_embeddings_shape": [27, 8], "normal_character_embeddings_row_0_prefix": [0.4, -0.5, 0.1]}
                let normal_character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]> =
                    vocabulary
                        .0
                        .try_map(|_: &VocabularyCharacter| -> Result<[FiniteF32; EMBEDDING_DIMENSIONS], &'static str> {
                            try_map_array([(); EMBEDDING_DIMENSIONS], |_: ()| -> Result<FiniteF32, &'static str> {
                                initialize_parameter_standard_normal()
                            })
                        })?;
                // {"regular_cube_enabled": false, "selected_initialization": "StandardNormal"}
                if USE_REGULAR_CUBE_EMBEDDINGS {
                    initialize_embeddings_regular_cube().try_map(
                        |row: &[f32; EMBEDDING_DIMENSIONS]| -> Result<[FiniteF32; EMBEDDING_DIMENSIONS], &'static str> {
                            try_map_array(*row, FiniteF32::try_from)
                        },
                    )?
                } else {
                    normal_character_embeddings
                }
            },
            first_layer_weights_parameters: try_map_array(
                [(); TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
                |_: ()| -> Result<[FiniteF32; FIRST_LAYER_SIZE], &'static str> {
                    try_map_array([(); FIRST_LAYER_SIZE], |_: ()| -> Result<FiniteF32, &'static str> {
                        initialize_parameter_standard_normal()
                    })
                },
            )?,
            first_layer_biases_parameters: try_map_array([(); FIRST_LAYER_SIZE], |_: ()| -> Result<FiniteF32, &'static str> {
                initialize_parameter_standard_normal()
            })?,
            second_layer_weights_parameters: {
                // {"second_layer_weights_parameters_shape": [27, 64], "initial_value": 0.0}
                let mut second_layer_weights_parameters: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]> = {
                    // {"zero_weight": 0.0}
                    let zero_weight: FiniteF32 = FiniteF32::zero();
                    vocabulary
                        .0
                        .map(|_: &VocabularyCharacter| -> [FiniteF32; FIRST_LAYER_SIZE] { [zero_weight; FIRST_LAYER_SIZE] })
                };
                for first_layer_index in 0..FIRST_LAYER_SIZE {
                    for character_weights in second_layer_weights_parameters.0.iter_mut() {
                        // {"first_layer_index": 0, "before": {"character_weights_0": 0.0}, "after": {"character_weights_0": -0.37}}
                        character_weights[first_layer_index] = initialize_parameter_standard_normal()?;
                    }
                }
                second_layer_weights_parameters
            },
            second_layer_biases_parameters: NonEmptyVec::try_from(
                (0..vocabulary_len)
                    .map(|_: usize| -> Result<FiniteF32, &'static str> { initialize_parameter_standard_normal() })
                    .collect::<Result<Vec<FiniteF32>, &'static str>>()?,
            )?,
            character_embeddings_gradients: NonEmptyVec::try_from(vec![[zero_as_finite_f32; EMBEDDING_DIMENSIONS]; vocabulary_len])?,
            first_layer_weights_gradients: [[zero_as_finite_f32; FIRST_LAYER_SIZE]; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
            first_layer_biases_gradients: [zero_as_finite_f32; FIRST_LAYER_SIZE],
            second_layer_weights_gradients: NonEmptyVec::try_from(vec![[zero_as_finite_f32; FIRST_LAYER_SIZE]; vocabulary_len])?,
            second_layer_biases_gradients: NonEmptyVec::try_from(vec![SecondLayerBiasGradientAsF32FromMinus1To1::zero(); vocabulary_len])?,
        }
    };
    // Пример входных аргументов calculate_average_negative_log_likelihood_loss. Для model показаны начала массивов параметров;
    // остальные элементы и поля градиентов опущены. Веса иллюстративные.
    // {
    //   "model": {
    //     "character_embeddings_parameters_row_prefixes": [[0.2, -0.1], [0.4, 0.3]],
    //     "first_layer_weights_parameters_row_prefixes": [[0.5, -0.2], [0.1, 0.6]],
    //     "first_layer_biases_parameters_prefix": [0.1, -0.2],
    //     "second_layer_weights_parameters_row_prefixes": [[0.4, -0.3], [0.2, 0.5]],
    //     "second_layer_biases_parameters_prefix": [0.05, -0.1]
    //   },
    //   "character_index_pairs": [
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 5},
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 5], "next_vocabulary_character_usize_index": 13},
    //     {"previous_vocabulary_character_usize_indexes": [0, 5, 13], "next_vocabulary_character_usize_index": 13},
    //     {"previous_vocabulary_character_usize_indexes": [5, 13, 13], "next_vocabulary_character_usize_index": 1},
    //     {"previous_vocabulary_character_usize_indexes": [13, 13, 1], "next_vocabulary_character_usize_index": 0}
    //   ]
    // }
    // Пары выше соответствуют имени "emma": "..." -> "e", "..e" -> "m",
    // ".em" -> "m", "emm" -> "a", "mma" -> "." (конец имени).
    // calculate_average_negative_log_likelihood_loss — замыкание для оценки средней ошибки модели на переданном наборе примеров.
    // model: &Model — общая ссылка: используем текущие веса без изменения параметров и градиентов.
    // character_index_pairs — непустой срез пар «три предыдущих символа -> следующий символ».
    // '_ означает выводимое время жизни заимствования среза; данные не копируются.
    // move захватывает внешние значения по значению, но здесь захватываемых значений нет:
    // модель и примеры передаются аргументами при каждом вызове.
    // Для каждой пары считаем -ln(p правильного символа), затем усредняем по числу пар.
    // Чем выше вероятность правильного символа, тем меньше ошибка; это не доля неверных ответов.
    // {"example": {"previous_characters": "emm", "target": "a", "previous_vocabulary_character_usize_indexes": [5, 13, 13], "next_vocabulary_character_usize_index": 1}, "target_probability": 0.7, "negative_log_likelihood_loss": 0.356675}
    // Для иллюстрации усреднения предположим одинаковую ошибку на всех пяти парах имени "emma".
    // {"per_example_losses": [0.356675, 0.356675, 0.356675, 0.356675, 0.356675], "return_value": 0.356675}
    // Result возвращает проверенную неотрицательную ошибку либо сообщение об ошибке вычисления.
    let calculate_average_negative_log_likelihood_loss = move |model: &Model,
                                                               character_index_pairs: NonEmptySlice<
        '_,
        PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
    >|
          -> Result<CrossEntropyLossAsNonNegativeF32, &'static str> {
        // {"total_negative_log_likelihood_loss": 0.0}
        let mut total_negative_log_likelihood_loss: f32 = 0.0;
        // {
        //   "character_index_pair": {"previous_vocabulary_character_usize_indexes": [5, 13, 13], "next_vocabulary_character_usize_index": 1}
        // }
        for character_index_pair in character_index_pairs.iter() {
            // 1. Берём три предыдущих символа: по ним модель предсказывает следующий.
            // {"previous_vocabulary_character_usize_indexes": [5, 13, 13], "previous_characters": "emm"}
            let previous_vocabulary_character_usize_indexes = character_index_pair.previous_vocabulary_character_usize_indexes;
            // 2. Прямой проход возвращает промежуточные активации и 27 оценок символов.
            // Это иллюстративные оценки, подобранные для вероятностей ниже; числа округлены.
            // Оценки ещё не вероятности: они могут быть отрицательными и не обязаны давать сумму 1.
            // {"previous_characters_embeddings_flattened_and_first_and_second_layer_outputs": {"second_layer_outputs_prefix": [-2.302585, -0.356675], "remaining_25_scores_each": -4.828314}, "second_layer_outputs_len": 27}
            let previous_characters_embeddings_flattened_and_first_and_second_layer_outputs: PreviousCharactersEmbeddingsFlattenedAndFirstAndSecondLayerOutputs = model.forward_pass_to_layer_1_outputs_after_weighted_sum_with_bias_and_silu_and_layer_2_outputs_after_weighted_sum_with_bias(previous_vocabulary_character_usize_indexes)?;
            // 3. Первым аргументом передаём непустой срез выходов второго слоя без копирования.
            // Промежуточные активации первого слоя для вычисления softmax и ошибки не нужны.
            // Индекс оценки совпадает с индексом символа в словаре: 0 — точка, 1 — a, ..., 26 — z.
            // {"second_layer_outputs_prefix": [-2.302585, -0.356675], "remaining_25_scores_each": -4.828314, "second_layer_outputs_len": 27}
            // 4. Правильный следующий символ берём из данных: после "emm" в имени "emma" идёт "a".
            // {"next_vocabulary_character_usize_index": 1, "target_character": "a"}
            // 5. Softmax превращает оценки в вероятности; loss = -ln(вероятности правильного символа).
            // В этом примере сумма вероятностей: 0.1 + 0.7 + 25 * 0.008 = 1; -ln(0.7) ≈ 0.356675.
            // {
            //   "next_character_probabilities_and_negative_log_likelihood": {
            //     "negative_log_likelihood_loss": 0.356675,
            //     "next_character_probabilities_from_second_layer_outputs_prefix": [0.1, 0.7], "remaining_25_probabilities_each": 0.008
            //   }
            // }
            let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesFromSecondLayerOutputsAndNegativeLogLikelihoodLoss =
                SoftmaxCrossEntropy::forward_next_character_probabilities_and_negative_log_likelihood_loss(
                    NonEmptySlice::from(&previous_characters_embeddings_flattened_and_first_and_second_layer_outputs.second_layer_outputs),
                    character_index_pair.next_vocabulary_character_usize_index,
                )?;
            // {
            //   "before": {"total_negative_log_likelihood_loss": 0.0},
            //   "contribution": 0.356675,
            //   "after": {"total_negative_log_likelihood_loss": 0.356675}
            // }
            total_negative_log_likelihood_loss += next_character_probabilities_and_negative_log_likelihood.negative_log_likelihood_loss.0;
        }
        // Сумму и среднее вычисляем в f32, как остальные операции модели.
        // Непустой срез гарантирует, что делитель больше нуля. Числа в примере округлены.
        // {"total_negative_log_likelihood_loss": 1.783375, "character_index_pairs_len": 5, "return_value": 0.356675}
        CrossEntropyLossAsNonNegativeF32::try_from(total_negative_log_likelihood_loss / character_index_pairs.len() as f32)
    };
    // Обучаем модель: на каждом шаге берём следующий батч из training_previous_vocabulary_character_indexes_and_next_character_index_pairs.
    // Дойдя до конца выборки, начинаем снова с первого примера; последний батч может быть короче.
    // Обнуляем градиенты, для каждого примера выполняем прямой проход и считаем -ln(p правильного символа).
    // Обратный проход накапливает градиенты средней ошибки батча; веса внутри батча не меняются.
    // После всего батча model.step обновляет обучаемые параметры: параметр -= learning_rate * градиент.
    // При FREEZE_EMBEDDINGS = true эмбеддинги остаются фиксированными.
    // Выполняем 125000 обновлений: первые 100000 с шагом 0.1, оставшиеся 25000 — с шагом 0.01.
    // _training_batch_average_negative_log_likelihood_loss хранит среднюю ошибку текущего батча до обновления параметров; дальше она не используется.
    {
        let mut training_batch_start_index: usize = 0;
        // {"first_step": 0, "last_step": 124999, "updates": 125000}
        for training_step in 0..training_steps.get() {
            // [{"training_step": 99999, "learning_rate": 0.1}, {"training_step": 100000, "learning_rate": 0.01}]
            let learning_rate: LearningRateAsPositiveFiniteF32 = LearningRateAsPositiveFiniteF32::try_from(if training_step < LEARNING_RATE_CHANGE_STEP {
                INITIAL_LEARNING_RATE
            } else {
                FINAL_LEARNING_RATE
            })?;
            // {"_training_batch_average_negative_log_likelihood_loss": 0.356675}
            let _training_batch_average_negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32 = {
                // {"average_negative_log_likelihood_loss": 0.356675}
                let average_negative_log_likelihood_loss: f32 = {
                    // {"training_previous_vocabulary_character_indexes_and_next_character_index_pairs_len": 169100, "training_batch_start_index": 0, "BATCH_SIZE": 32, "training_batch_end_index": 32}
                    let training_batch_end_index: usize = (training_batch_start_index + BATCH_SIZE).min(training_previous_vocabulary_character_indexes_and_next_character_index_pairs.len());
                    // Иллюстрация состава батча на имени "emma", не первые строки реального train.
                    // {
                    //   "training_previous_vocabulary_character_indexes_and_next_character_index_batch": [
                    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 5},
                    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 5], "next_vocabulary_character_usize_index": 13},
                    //     {"previous_vocabulary_character_usize_indexes": [0, 5, 13], "next_vocabulary_character_usize_index": 13},
                    //     {"previous_vocabulary_character_usize_indexes": [5, 13, 13], "next_vocabulary_character_usize_index": 1}
                    //   ],
                    //   "omitted_elements": 28,
                    //   "source_range": [0, 32]
                    // }
                    let training_previous_vocabulary_character_indexes_and_next_character_index_batch: NonEmptySlice<
                        '_,
                        PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
                    > = NonEmptySlice::try_from(&training_previous_vocabulary_character_indexes_and_next_character_index_pairs[training_batch_start_index..training_batch_end_index])?;
                    // {"training_batch_end_index": 32, "training_previous_vocabulary_character_indexes_and_next_character_index_pairs_len": 169100, "before": {"training_batch_start_index": 0}, "after": {"training_batch_start_index": 32}}
                    training_batch_start_index = if training_batch_end_index == training_previous_vocabulary_character_indexes_and_next_character_index_pairs.len() { 0 } else { training_batch_end_index };
                    // {"zero_as_finite_f32": 0.0}
                    let zero_as_finite_f32: FiniteF32 = FiniteF32::zero();
                    // {"training_batch_len": 32}
                    let training_batch_len: std::num::NonZeroUsize =
                        std::num::NonZeroUsize::new(training_previous_vocabulary_character_indexes_and_next_character_index_batch.len())
                            .ok_or("батч не должен быть пустым")?;
                    // {
                    //   "before": {"second_layer_biases_gradients_prefix": [0.2, -0.1, 0.05]},
                    //   "after": {"second_layer_biases_gradients_prefix": [0.0, 0.0, 0.0]},
                    //   "all_gradient_arrays_reset": true
                    // }
                    model.overwrite_gradients_with_zeros()?;
                    // {"average_negative_log_likelihood_loss": 0.356675}
                    let average_negative_log_likelihood_loss: f32 = {
                        // {"training_batch_len": 32, "total_negative_log_likelihood_loss": 11.413598}
                        let total_negative_log_likelihood_loss: FiniteF32 = {
                            // {"total_negative_log_likelihood_loss": 0.0}
                            let mut total_negative_log_likelihood_loss: FiniteF32 = zero_as_finite_f32;
                            // {
                            //   "training_previous_vocabulary_character_indexes_and_next_character_index_batch_element": {"previous_vocabulary_character_usize_indexes": [5, 13, 13], "next_vocabulary_character_usize_index": 1}
                            // }
                            for training_previous_vocabulary_character_indexes_and_next_character_index_batch_element in
                                training_previous_vocabulary_character_indexes_and_next_character_index_batch.iter()
                            {
                                // {
                                //   "previous_characters_embeddings_flattened_and_first_and_second_layer_outputs": {
                                //     "previous_characters_embeddings_flattened": [0.4, -0.5, 0.1],
                                //     "first_layer_outputs": [0.2, -0.5],
                                //     "second_layer_outputs_prefix": [-2.302585, -0.356675], "remaining_25_scores_each": -4.828314
                                //   },
                                //   "omitted_elements": {"previous_characters_embeddings_flattened": 21, "first_layer_outputs": 62}
                                // }
                                let previous_characters_embeddings_flattened_and_first_and_second_layer_outputs: PreviousCharactersEmbeddingsFlattenedAndFirstAndSecondLayerOutputs = model.forward_pass_to_layer_1_outputs_after_weighted_sum_with_bias_and_silu_and_layer_2_outputs_after_weighted_sum_with_bias(
                                    training_previous_vocabulary_character_indexes_and_next_character_index_batch_element.previous_vocabulary_character_usize_indexes,
                                )?;
                                // {
                                //   "next_character_probabilities_and_negative_log_likelihood": {
                                //     "negative_log_likelihood_loss": 0.356675,
                                //     "next_character_probabilities_from_second_layer_outputs_prefix": [0.1, 0.7], "remaining_25_probabilities_each": 0.008
                                //   }
                                // }
                                let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesFromSecondLayerOutputsAndNegativeLogLikelihoodLoss =
                                    SoftmaxCrossEntropy::forward_next_character_probabilities_and_negative_log_likelihood_loss(
                                        NonEmptySlice::from(&previous_characters_embeddings_flattened_and_first_and_second_layer_outputs.second_layer_outputs),
                                        training_previous_vocabulary_character_indexes_and_next_character_index_batch_element.next_vocabulary_character_usize_index,
                                    )?;
                                // {
                                //   "before": {"total_negative_log_likelihood_loss": 1.070025},
                                //   "contribution": 0.356675,
                                //   "after": {"total_negative_log_likelihood_loss": 1.4267}
                                // }
                                total_negative_log_likelihood_loss = FiniteF32::try_from(
                                    total_negative_log_likelihood_loss.0
                                        + next_character_probabilities_and_negative_log_likelihood.negative_log_likelihood_loss.0,
                                )?;
                                // {
                                //   "target": 1,
                                //   "training_batch_len": 32,
                                //   "next_character_score_gradients_as_f32_from_minus1_to1_prefix": [{"Other": 0.003125}, {"Target": -0.009375}],
                                //   "remaining_25_gradients_each": {"Other": 0.00025}
                                // }
                                let next_character_score_gradients_as_f32_from_minus1_to1: NonEmptyVec<NextCharacterScoreGradientAsF32FromMinus1To1> = SoftmaxCrossEntropy::backward_next_character_score_gradients_as_f32_from_minus1_to1(
                                    &next_character_probabilities_and_negative_log_likelihood,
                                    training_previous_vocabulary_character_indexes_and_next_character_index_batch_element.next_vocabulary_character_usize_index,
                                    training_batch_len,
                                )?;
                                // {
                                //   "character_index": 1,
                                //   "score_gradient": -0.009375,
                                //   "before": {"second_layer_biases_gradients_1": 0.01},
                                //   "after": {"second_layer_biases_gradients_1": 0.000625},
                                //   "parameters_changed": false
                                // }
                                model.accumulate_parameter_gradients_from_training_example(
                                    training_previous_vocabulary_character_indexes_and_next_character_index_batch_element.previous_vocabulary_character_usize_indexes,
                                    &previous_characters_embeddings_flattened_and_first_and_second_layer_outputs,
                                    &next_character_score_gradients_as_f32_from_minus1_to1,
                                )?;
                                // {"parameters_updated": false, "gradients_accumulated": true}
                            }
                            total_negative_log_likelihood_loss
                        };
                        // {"total_negative_log_likelihood_loss": 11.413598, "training_batch_len": 32, "result": 0.356675}
                        total_negative_log_likelihood_loss.0 / training_batch_len.get() as f32
                    };
                    average_negative_log_likelihood_loss
                };
                // {
                //   "learning_rate": 0.1,
                //   "second_layer_biases_gradients_1": -0.15,
                //   "before": {"second_layer_biases_parameters_1": 0.2},
                //   "after": {"second_layer_biases_parameters_1": 0.215}
                // }
                model.step(learning_rate)?;
                CrossEntropyLossAsNonNegativeF32::try_from(average_negative_log_likelihood_loss)?
            };
        }
    }
    // Средняя ошибка по полным выборкам после обучения; эти вызовы не меняют веса.
    // Сохранённый замер для текущей конфигурации, seed = 42 и 125000 шагов:
    // benchmarks/frozen_embeddings/applied_configuration/verification.json.
    // Исторический результат FP32: {"order": ["train", "dev", "test"], "_losses": [2.18501663, 2.21287227, 2.20417547], "parameters_updated": false}
    let _losses: [CrossEntropyLossAsNonNegativeF32; 3] = [
        calculate_average_negative_log_likelihood_loss(&model, NonEmptySlice::from(&training_previous_vocabulary_character_indexes_and_next_character_index_pairs))?,
        calculate_average_negative_log_likelihood_loss(&model, NonEmptySlice::from(&validation_previous_vocabulary_character_indexes_and_next_character_index_pairs))?,
        calculate_average_negative_log_likelihood_loss(&model, NonEmptySlice::from(&test_previous_vocabulary_character_indexes_and_next_character_index_pairs))?,
    ];
    // {"iterations": 20, "names_per_iteration": 1}
    for _ in 0..GENERATED_NAMES_COUNT {
        // {"_name": "emma."}
        let _name: String = 'sample: {
            // {"previous_vocabulary_character_usize_indexes": [0, 0, 0]}
            let mut previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION] =
                [vocabulary.dot_index(); PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION];
            // {"generated_name": ""}
            let mut generated_name: String = String::new();
            // {"max_sampled_characters": 100, "stop_index": 0}
            for _ in 0..MAX_GENERATED_NAME_LENGTH {
                // {
                //   "target": 0,
                //   "next_character_probabilities_and_negative_log_likelihood": {
                //     "negative_log_likelihood_loss": 2.302585,
                //     "next_character_probabilities_from_second_layer_outputs_prefix": [0.1, 0.7], "remaining_25_probabilities_each": 0.008
                //   },
                //   "loss_used_for_sampling": false
                // }
                let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesFromSecondLayerOutputsAndNegativeLogLikelihoodLoss =
                    SoftmaxCrossEntropy::forward_next_character_probabilities_and_negative_log_likelihood_loss(
                        NonEmptySlice::from(
                            &model
                                .forward_pass_to_layer_1_outputs_after_weighted_sum_with_bias_and_silu_and_layer_2_outputs_after_weighted_sum_with_bias(
                                    previous_vocabulary_character_usize_indexes,
                                )?
                                .second_layer_outputs,
                        ),
                        vocabulary.dot_index(),
                    )?;
                // {"random_probability": 0.65}
                let random_probability: CharacterProbabilityAsF32From0To1 =
                    CharacterProbabilityAsF32From0To1::try_from(rand::RngExt::random::<f32>(&mut random_generator))?;
                // {
                //   "random_probability": 0.65,
                //   "next_character_probabilities_from_second_layer_outputs_prefix": [0.1, 0.7], "remaining_25_probabilities_each": 0.008,
                //   "sampled_character_index": 1
                // }
                let sampled_character_index: usize = {
                    // {"cumulative_probability": 0.0}
                    let mut cumulative_probability: f32 = ZERO_F32;
                    // {"vocabulary_len": 27, "sampled_character_index": 26}
                    let mut sampled_character_index: usize = vocabulary_len - 1;
                    for next_character_probability_with_index in next_character_probabilities_and_negative_log_likelihood
                        .next_character_probabilities_from_second_layer_outputs
                        .iter()
                        .enumerate()
                    {
                        // {"character_index": 1, "character_probability": 0.7}
                        let (character_index, character_probability): (usize, &CharacterProbabilityAsF32From0To1) = next_character_probability_with_index;
                        // {"character_probability": 0.7, "before": {"cumulative_probability": 0.1}, "after": {"cumulative_probability": 0.8}}
                        cumulative_probability += character_probability.0;
                        // {"random_probability": 0.65, "cumulative_probability": 0.8, "condition": true}
                        if random_probability.0 < cumulative_probability {
                            // {"character_index": 1, "sampled_character_index": 1}
                            sampled_character_index = character_index;
                            break;
                        }
                    }
                    sampled_character_index
                };
                // {"sampled_character_index": 1}
                let sampled_character_index: VocabularyCharacterIndexAsUsize =
                    VocabularyCharacterIndexAsUsize::try_from((sampled_character_index, &vocabulary))?;
                // {
                //   "sampled_character_index": 1,
                //   "character": "a",
                //   "before": {"generated_name": "emm"},
                //   "after": {"generated_name": "emma"}
                // }
                generated_name.push(vocabulary.character(sampled_character_index)?);
                // [
                //   {"sampled_character_index": 1, "generated_name": "emma", "stop": false},
                //   {"sampled_character_index": 0, "generated_name": "emma.", "stop": true}
                // ]
                if sampled_character_index.0 == 0 {
                    break 'sample generated_name;
                }
                // {
                //   "before": {"previous_vocabulary_character_usize_indexes": [5, 13, 13]},
                //   "after": {"previous_vocabulary_character_usize_indexes": [13, 13, 13]}
                // }
                for previous_character_position in 0..PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION - 1 {
                    previous_vocabulary_character_usize_indexes[previous_character_position] =
                        previous_vocabulary_character_usize_indexes[previous_character_position + 1];
                }
                // {
                //   "sampled_character_index": 1,
                //   "before": {"previous_vocabulary_character_usize_indexes": [13, 13, 13]},
                //   "after": {"previous_vocabulary_character_usize_indexes": [13, 13, 1]}
                // }
                previous_vocabulary_character_usize_indexes[PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION - 1] = sampled_character_index;
            }
            // {"generated_name_len_before": 100, "appended_suffix": " [лимит 100 символов]"}
            generated_name.push_str(&format!(" [лимит {MAX_GENERATED_NAME_LENGTH} символов]"));
            generated_name
        };
    }
    // {"Ok": null}
    Ok(())
}
// Все параметры, активации и градиенты хранятся и вычисляются в FP32.
#[derive(Clone)]
struct Model {
    character_embeddings_parameters: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
    first_layer_weights_parameters: [[FiniteF32; FIRST_LAYER_SIZE]; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    first_layer_biases_parameters: [FiniteF32; FIRST_LAYER_SIZE],
    second_layer_weights_parameters: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
    second_layer_biases_parameters: NonEmptyVec<FiniteF32>,
    // Градиенты для соответствующих параметров накапливаются отдельно.
    character_embeddings_gradients: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
    first_layer_weights_gradients: [[FiniteF32; FIRST_LAYER_SIZE]; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    first_layer_biases_gradients: [FiniteF32; FIRST_LAYER_SIZE],
    second_layer_weights_gradients: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
    second_layer_biases_gradients: NonEmptyVec<SecondLayerBiasGradientAsF32FromMinus1To1>,
}
impl Model {
    // Размеры ниже соответствуют константам урока и словарю из 27 символов.
    // Числа иллюстративные и округлены; отдельные примеры операций независимы.
    // Поля с суффиксом _prefix показывают только начало массива.
    // Прямой проход: три индекса -> 24 координаты -> 64 активации -> 27 оценок.
    // {"previous_vocabulary_character_usize_indexes": [0, 0, 5], "previous_characters": "..e"}
    fn forward_pass_to_layer_1_outputs_after_weighted_sum_with_bias_and_silu_and_layer_2_outputs_after_weighted_sum_with_bias(
        &self,
        previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    ) -> Result<PreviousCharactersEmbeddingsFlattenedAndFirstAndSecondLayerOutputs, &'static str> {
        // Конкатенация сохраняет порядок символов и координат внутри каждого эмбеддинга.
        // {"previous_character_embeddings_shape": [3, 8], "previous_characters_embeddings_flattened_shape": [24]}
        let previous_characters_embeddings_flattened: [FiniteF32; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION] = {
            // Повторяющийся индекс 0 дважды выбирает одну и ту же строку параметров.
            // Пример для контекста "..e": из таблицы модели берём строки 0, 0 и 5.
            // Ниже показаны все 10 координат выбранных строк; другие строки таблицы опущены.
            // Числа иллюстративные. map копирует строки в порядке входных индексов.
            // {
            //   "previous_vocabulary_character_usize_indexes": [0, 0, 5],
            //   "character_embeddings_parameters_selected_rows": [
            //     {"index": 0, "character": ".", "embedding": [0.2, -0.1, 0.3, 0.0, -0.4, 0.5, 0.1, -0.2, 0.6, -0.3]},
            //     {"index": 5, "character": "e", "embedding": [0.4, 0.3, -0.2, 0.1, 0.0, -0.5, 0.7, 0.2, -0.1, 0.6]}
            //   ],
            //   "previous_character_embeddings": [
            //     [0.2, -0.1, 0.3, 0.0, -0.4, 0.5, 0.1, -0.2, 0.6, -0.3],
            //     [0.2, -0.1, 0.3, 0.0, -0.4, 0.5, 0.1, -0.2, 0.6, -0.3],
            //     [0.4, 0.3, -0.2, 0.1, 0.0, -0.5, 0.7, 0.2, -0.1, 0.6]
            //   ],
            //   "previous_character_embeddings_shape": [3, 10]
            // }
            let previous_character_embeddings: [[FiniteF32; EMBEDDING_DIMENSIONS]; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION] =
                previous_vocabulary_character_usize_indexes.map(
                    |vocabulary_character_index: VocabularyCharacterIndexAsUsize| -> [FiniteF32; EMBEDDING_DIMENSIONS] {
                        self.character_embeddings_parameters[vocabulary_character_index.0]
                    },
                );
            // {"flattened_coordinate_sources": [{"index": 0, "symbol_index": 0, "coordinate": 0}, {"index": 8, "symbol_index": 0, "coordinate": 0}, {"index": 16, "symbol_index": 5, "coordinate": 0}]}
            previous_character_embeddings
                .as_flattened()
                .try_into()
                .map_err(|_: std::array::TryFromSliceError| -> &'static str { "неверная длина объединённых эмбеддингов" })?
        };
        let first_layer_inputs: [FiniteF32; FIRST_LAYER_SIZE];
        let first_layer_outputs: [FiniteF32; FIRST_LAYER_SIZE] = {
            // Копируем first_layer_biases_parameters в first_layer_outputs как начальные значения сумм.
            // {"first_layer_biases_parameters_prefix": [0.1, -0.2], "first_layer_outputs_prefix": [0.1, -0.2]}
            let mut first_layer_outputs: [f32; FIRST_LAYER_SIZE] =
                self.first_layer_biases_parameters.map(|value: FiniteF32| -> f32 { value.0 });
            // На входе 24 числа: по 8 элементов эмбеддинга для каждого из трёх предыдущих символов.
            // На выходе нужны 64 числа: каждый элемент first_layer_outputs накапливает все 24 входа со своими весами.
            // first_layer_weights_parameters имеет форму [24, 64]:
            // строка i содержит 64 веса для вклада входного элемента i в 64 элемента first_layer_outputs.
            // Пока first_layer_outputs хранит суммы ДО SiLU, первоначально равные смещениям.
            //
            // Внешний цикл выбирает один из 24 входных элементов.
            // enumerate() даёт его индекс 0..23 и ссылку на значение; & в шаблоне копирует FiniteF32.
            // Индексы 0..7 относятся к первому символу, 8..15 — ко второму, 16..23 — к третьему.
            for (previous_characters_embeddings_flattened_index, &previous_character_embedding_element) in
                previous_characters_embeddings_flattened.iter().enumerate()
            {
                // Выбираем строку весов для текущего входного элемента: в ней ровно 64 веса.
                // iter_mut() даёт изменяемые ссылки на 64 накапливаемые суммы.
                // zip() соединяет first_layer_outputs[0] с весом строки [0], элемент [1] с весом [1] и так далее.
                // Поэтому внутренний цикл делает 64 итерации: один вход добавляет вклад во все 64 элемента first_layer_outputs.
                // first_layer_activation — изменяемая ссылка на текущий элемент first_layer_outputs;
                // first_layer_weight — копия соответствующего элемента выбранной строки first_layer_weights_parameters.
                for (first_layer_activation, &first_layer_weight) in first_layer_outputs
                    .iter_mut()
                    .zip(&self.first_layer_weights_parameters[previous_characters_embeddings_flattened_index])
                {
                    // Добавляем один вклад: *first_layer_activation += previous_character_embedding_element.0 * first_layer_weight.0.
                    // Например, при входе 0.2 и весах [0.5, -0.3, ...] суммы [0.1, -0.2, ...]
                    // станут [0.2, -0.26, ...]. Для каждого элемента first_layer_outputs используется свой вес.
                    // .0 извлекает f32; * слева позволяет изменить сумму по ссылке.
                    // Параметры весов и эмбеддингов здесь только читаются, обучение их пока не меняет.
                    // {"previous_character_embedding_element": 0.2, "first_layer_weight": 0.5, "before": {"first_layer_activation": 0.1}, "after": {"first_layer_activation": 0.2}}
                    *first_layer_activation += previous_character_embedding_element.0 * first_layer_weight.0;
                }
            }
            // После обоих циклов каждая из 64 сумм равна:
            // first_layer_outputs[j] = first_layer_biases_parameters[j]
            //     + previous_characters_embeddings_flattened[0] * first_layer_weights_parameters[0][j]
            //     + ... + previous_characters_embeddings_flattened[23] * first_layer_weights_parameters[23][j].
            // Здесь j — индекс элемента first_layer_outputs от 0 до 63; в формуле опущено извлечение .0.
            // Всего выполнено 24 × 64 = 1536 умножений и столько же прибавлений к суммам.
            // После суммирования всех 24 вкладов применяем SiLU к каждому из 64 элементов first_layer_outputs.
            // {"before": {"first_layer_activation": 0.2}, "after": {"first_layer_activation": 0.109967}}
            // SiLU нельзя однозначно обратить: сохраняем суммы до функции для обратного прохода.
            // Проверяем конечность сумм и сохраняем их в FP32 для обратного прохода.
            first_layer_inputs = try_map_array(first_layer_outputs, FiniteF32::try_from)?;
            try_map_array(first_layer_inputs, Silu::forward_activation)?
        };
        let second_layer_outputs: NonEmptyVec<FiniteF32> = {
            // Для каждого из 27 символов берём его смещение как начальное значение суммы.
            // Затем прибавим произведения выходов первого слоя на веса. Получатся оценки символов, а не вероятности.
            // map извлекает f32: получаем отдельный изменяемый массив из 27 чисел.
            // Поэтому до циклов second_layer_outputs[j] = second_layer_biases_parameters[j].
            // Параметры модели при этом не изменяются — ниже изменяем только выходные суммы.
            // {"second_layer_biases_parameters_prefix": [0.05, -0.1], "second_layer_outputs_prefix": [0.05, -0.1]}
            let mut second_layer_outputs: NonEmptyVec<f32> = self.second_layer_biases_parameters
                .map(|value: &FiniteF32| -> f32 { value.0 });
            // Внешний цикл перебирает все 64 выхода первого слоя, уже прошедшие SiLU.
            // enumerate даёт индекс i (0..63) и значение first_layer_outputs[i].
            // Одно это значение должно добавить свой вклад в КАЖДУЮ из 27 выходных сумм.
            for (first_layer_index, &first_layer_activation) in first_layer_outputs.iter().enumerate() {
                // second_layer_weights_parameters — 27 строк по 64 веса.
                // Строка j содержит веса всех 64 входов для одного выхода second_layer_outputs[j].
                // zip связывает изменяемый выход j с его строкой весов j:
                // выход 0 со строкой 0, выход 1 со строкой 1, ..., выход 26 со строкой 26.
                // second_layer_output — ссылка на одну сумму; second_layer_weights — ссылка на 64 веса.
                // Внутренний цикл делает 27 итераций для текущего first_layer_index.
                for (second_layer_output, second_layer_weights) in second_layer_outputs.0.iter_mut().zip(self.second_layer_weights_parameters.iter())
                {
                    // Из строки весов выбираем вес с индексом i, соответствующий текущему входу.
                    // Одна итерация: second_layer_outputs[j] += first_layer_outputs[i] * weights[j][i].
                    // Например, к начальному смещению 0.05 добавляем 0.197375 * 0.4 = 0.07895.
                    // {"first_layer_activation": 0.197375, "second_layer_weight": 0.4, "before": {"second_layer_output": 0.05}, "after": {"second_layer_output": 0.12895}}
                    *second_layer_output += first_layer_activation.0 * second_layer_weights[first_layer_index].0;
                }
            }
            // После обоих циклов каждый выход содержит смещение и все 64 произведения:
            // second_layer_outputs[j] = second_layer_biases_parameters[j]
            //     + сумма по i от 0 до 63 (first_layer_outputs[i] * second_layer_weights_parameters[j][i]).
            // Всего выполнено 64 * 27 = 1728 добавлений произведений; результат — 27 оценок.
            // SiLU здесь не применяется. Позже softmax преобразует эти оценки в вероятности символов.
            // Проверяем, что вычисленные оценки конечны, и возвращаем их из блока.
            NonEmptyVec::try_from(
                second_layer_outputs
                    .into_iter()
                    .map(FiniteF32::try_from)
                    .collect::<Result<Vec<FiniteF32>, &'static str>>()?,
            )?
        };
        // Сохраняем промежуточные значения для accumulate_parameter_gradients_from_training_example; softmax вычисляется отдельно.
        // {"return_value_shapes": {"previous_characters_embeddings_flattened": [24], "first_layer_outputs": [64], "second_layer_outputs": [27]}}
        Ok(PreviousCharactersEmbeddingsFlattenedAndFirstAndSecondLayerOutputs {
            previous_characters_embeddings_flattened,
            first_layer_inputs,
            first_layer_outputs,
            second_layer_outputs,
        })
    }
    // Перед новым батчем обнуляем все накопленные производные, сохраняя параметры.
    // {"gradient_shapes": [[27, 8], [24, 64], [64], [27, 64], [27]], "after": {"every_gradient": 0.0}}
    fn overwrite_gradients_with_zeros(&mut self) -> Result<(), &'static str> {
        let zero_as_finite_f32: FiniteF32 = FiniteF32::zero();
        if !FREEZE_EMBEDDINGS {
            self.character_embeddings_gradients.0.as_flattened_mut().fill(zero_as_finite_f32);
        }
        self.first_layer_weights_gradients.as_flattened_mut().fill(zero_as_finite_f32);
        self.first_layer_biases_gradients.fill(zero_as_finite_f32);
        self.second_layer_weights_gradients.0.as_flattened_mut().fill(zero_as_finite_f32);
        self.second_layer_biases_gradients.0.fill(SecondLayerBiasGradientAsF32FromMinus1To1::zero());
        Ok(())
    }
    // Один вызов добавляет вклад одного примера к градиентам текущего батча.
    // Входные производные по оценкам уже учитывают усреднение loss по батчу.
    // Например, для целевого символа e: (0.2 - 1) / 32 = -0.025.
    // {"target_character_index": 5, "target_probability": 0.2, "batch_size": 32, "score_gradient_at_target": -0.025, "score_gradients_shape": [27]}
    // Индексы передаются отдельно: по ним выбираем строки таблицы для накопления градиентов эмбеддингов.
    // Они должны соответствовать тому же примеру, для которого получен previous_characters_embeddings_flattened_and_first_and_second_layer_outputs.
    // {"previous_vocabulary_character_usize_indexes": [0, 0, 5]}
    fn accumulate_parameter_gradients_from_training_example(
        &mut self,
        previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
        previous_characters_embeddings_flattened_and_first_and_second_layer_outputs: &PreviousCharactersEmbeddingsFlattenedAndFirstAndSecondLayerOutputs,
        next_character_score_gradients_as_f32_from_minus1_to1: &NonEmptyVec<NextCharacterScoreGradientAsF32FromMinus1To1>,
    ) -> Result<(), &'static str> {
        let zero_as_finite_f32: FiniteF32 = FiniteF32::zero();
        // {"second_layer_biases_parameters_len": 27}
        let second_layer_biases_parameters_len: usize = self.second_layer_biases_parameters.len();
        // Правило для произведения: вклад = вход (или вес) * производная по выходу.
        // {"input_or_weight": 0.4, "output_gradient": -0.025, "contribution": -0.01}
        fn add_input_or_weight_times_output_gradient_to_gradient(
            gradient: &mut FiniteF32,
            input_or_weight: FiniteF32,
            output_gradient: f32,
            location: &'static str,
        ) -> Result<(), &'static str> {
            // Произведение и накопление градиента выполняются в FP32.
            *gradient = FiniteF32::try_from(gradient.0 + input_or_weight.0 * output_gradient)
                .map_err(|_: &'static str| -> &'static str { location })?;
            Ok(())
        }
        {
            // Производная score по его смещению равна 1: переносим градиент без множителя.
            // {"vocabulary_character_index": 5, "score_gradient": -0.025, "before": {"bias_gradient": 0.01}, "after": {"bias_gradient": -0.015}}
            for (second_layer_bias_gradient, &next_character_score_gradient_as_f32_from_minus1_to1) in self.second_layer_biases_gradients
                .0
                .iter_mut()
                .zip(next_character_score_gradients_as_f32_from_minus1_to1.iter())
            {
                *second_layer_bias_gradient = SecondLayerBiasGradientAsF32FromMinus1To1::try_from(
                    second_layer_bias_gradient.0 + f32::from(next_character_score_gradient_as_f32_from_minus1_to1),
                )?;
            }
        }
        {
            // К элементу second_layer_weights_gradients добавляем произведение элемента first_layer_outputs и градиента оценки символа.
            // {"first_layer_activation": 0.4, "score_gradient": -0.025, "before": {"weight_gradient": 0.0}, "after": {"weight_gradient": -0.01}}
            for first_layer_index in 0..FIRST_LAYER_SIZE {
                for vocabulary_character_index in 0..second_layer_biases_parameters_len {
                    add_input_or_weight_times_output_gradient_to_gradient(
                        &mut self.second_layer_weights_gradients.0[vocabulary_character_index][first_layer_index],
                        previous_characters_embeddings_flattened_and_first_and_second_layer_outputs.first_layer_outputs[first_layer_index],
                        f32::from(next_character_score_gradients_as_f32_from_minus1_to1[vocabulary_character_index]),
                        "переполнение градиента second_layer_weights_parameters",
                    )?;
                }
            }
        }
        // Возвращаем градиент через W2: суммируем вклады всех 27 выходных символов.
        // Затем умножаем на производную SiLU по сохранённой сумме x: s + x * s * (1 - s).
        // {"first_layer_input": 0.5, "summed_activation_gradient": -0.1, "silu_derivative": 0.739961, "pre_activation_gradient": -0.073996, "result_shape": [64]}
        let first_layer_pre_activation_gradients: [FiniteF32; FIRST_LAYER_SIZE] = {
            let mut first_layer_pre_activation_gradients: [FiniteF32; FIRST_LAYER_SIZE] = [zero_as_finite_f32; FIRST_LAYER_SIZE];
            for first_layer_index in 0..FIRST_LAYER_SIZE {
                for vocabulary_character_index in 0..second_layer_biases_parameters_len {
                    add_input_or_weight_times_output_gradient_to_gradient(
                        &mut first_layer_pre_activation_gradients[first_layer_index],
                        self.second_layer_weights_parameters[vocabulary_character_index][first_layer_index],
                        f32::from(next_character_score_gradients_as_f32_from_minus1_to1[vocabulary_character_index]),
                        "переполнение градиента входа первого слоя",
                    )?;
                }
                let silu_input_gradient = Silu::backward_input_gradient(
                    previous_characters_embeddings_flattened_and_first_and_second_layer_outputs.first_layer_inputs[first_layer_index].0,
                    first_layer_pre_activation_gradients[first_layer_index],
                )?;
                first_layer_pre_activation_gradients[first_layer_index] = silu_input_gradient;
            }
            first_layer_pre_activation_gradients
        };
        {
            // Смещение первого слоя получает производную по сумме до SiLU.
            // {"pre_activation_gradient": -0.084, "before": {"bias_gradient": 0.01}, "after": {"bias_gradient": -0.074}}
            for first_layer_index in 0..FIRST_LAYER_SIZE {
                self.first_layer_biases_gradients[first_layer_index] = FiniteF32::try_from(
                    self.first_layer_biases_gradients[first_layer_index].0 + first_layer_pre_activation_gradients[first_layer_index].0,
                )
                .map_err(|_: &'static str| -> &'static str { "переполнение градиента first_layer_biases_parameters" })?;
            }
        }
        {
            // К элементу first_layer_weights_gradients добавляем произведение элемента previous_characters_embeddings_flattened и элемента first_layer_pre_activation_gradients.
            // {"embedding_coordinate": 0.2, "pre_activation_gradient": -0.084, "before": {"weight_gradient": 0.0}, "after": {"weight_gradient": -0.0168}}
            for previous_characters_embeddings_flattened_index in 0..previous_characters_embeddings_flattened_and_first_and_second_layer_outputs
                .previous_characters_embeddings_flattened
                .len()
            {
                for first_layer_index in 0..FIRST_LAYER_SIZE {
                    add_input_or_weight_times_output_gradient_to_gradient(
                        &mut self.first_layer_weights_gradients[previous_characters_embeddings_flattened_index][first_layer_index],
                        previous_characters_embeddings_flattened_and_first_and_second_layer_outputs.previous_characters_embeddings_flattened
                            [previous_characters_embeddings_flattened_index],
                        first_layer_pre_activation_gradients[first_layer_index].0,
                        "переполнение градиента first_layer_weights_parameters",
                    )?;
                }
            }
        }
        if !FREEZE_EMBEDDINGS {
            // Возвращаем градиент к каждой из 24 входных координат через веса первого слоя.
            // {"first_layer_weight": 0.5, "pre_activation_gradient": -0.084, "one_neuron_contribution": -0.042, "summed_neuron_count": 64}
            for previous_characters_embeddings_flattened_index in 0..previous_characters_embeddings_flattened_and_first_and_second_layer_outputs
                .previous_characters_embeddings_flattened
                .len()
            {
                let input_coordinate_gradient: FiniteF32 = {
                    let mut input_coordinate_gradient: FiniteF32 = zero_as_finite_f32;
                    for first_layer_index in 0..FIRST_LAYER_SIZE {
                        add_input_or_weight_times_output_gradient_to_gradient(
                            &mut input_coordinate_gradient,
                            self.first_layer_weights_parameters[previous_characters_embeddings_flattened_index][first_layer_index],
                            first_layer_pre_activation_gradients[first_layer_index].0,
                            "переполнение градиента координаты эмбеддинга",
                        )?;
                    }
                    input_coordinate_gradient
                };
                // Деление на 8 выбирает позицию символа, остаток — координату его эмбеддинга.
                // {"previous_characters_embeddings_flattened_index": 17, "previous_character_position": 2, "symbol_index": 5, "embedding_coordinate_index": 1}
                // Повторы символа складывают вклады в общую строку таблицы эмбеддингов.
                // {"previous_characters": [0, 0, 5], "flattened_indexes": [0, 8], "shared_gradient_index": [0, 0], "contributions": [0.02, -0.03], "gradient_before": 0.0, "gradient_after": -0.01}
                let embedding_gradient: &mut FiniteF32 = &mut self.character_embeddings_gradients.0
                    [previous_vocabulary_character_usize_indexes[previous_characters_embeddings_flattened_index / EMBEDDING_DIMENSIONS].0]
                    [previous_characters_embeddings_flattened_index % EMBEDDING_DIMENSIONS];
                *embedding_gradient = FiniteF32::try_from(embedding_gradient.0 + input_coordinate_gradient.0)
                    .map_err(|_: &'static str| -> &'static str { "переполнение градиента character_embeddings_parameters" })?;
            }
        }
        Ok(())
    }
    // Шаг SGD обновляет веса и смещения после накопления градиентов батча.
    // Эмбеддинги обновляются только при FREEZE_EMBEDDINGS = false.
    // Градиенты здесь не обнуляются: перед следующим батчем вызывается overwrite_gradients_with_zeros.
    // {"learning_rate": 0.1, "updated_parameter_count": 3355}
    fn step(&mut self, learning_rate: LearningRateAsPositiveFiniteF32) -> Result<(), &'static str> {
        let update_parameters = |parameters: &mut [FiniteF32], gradients: &[FiniteF32]| -> Result<(), &'static str> {
            if parameters.len() != gradients.len() {
                return Err("число параметров и градиентов должно совпадать");
            }
            for (parameter, gradient) in parameters.iter_mut().zip(gradients) {
                *parameter = FiniteF32::try_from(parameter.0 - learning_rate.0 * gradient.0)?;
            }
            Ok(())
        };
        if !FREEZE_EMBEDDINGS {
            update_parameters(
                self.character_embeddings_parameters.0.as_flattened_mut(),
                self.character_embeddings_gradients.as_flattened(),
            )?;
        }
        update_parameters(
            self.first_layer_weights_parameters.as_flattened_mut(),
            self.first_layer_weights_gradients.as_flattened(),
        )?;
        update_parameters(
            &mut self.first_layer_biases_parameters,
            &self.first_layer_biases_gradients,
        )?;
        update_parameters(
            self.second_layer_weights_parameters.0.as_flattened_mut(),
            self.second_layer_weights_gradients.as_flattened(),
        )?;
        if self.second_layer_biases_parameters.len() != self.second_layer_biases_gradients.len() {
            return Err("число смещений второго слоя и градиентов должно совпадать");
        }
        for (parameter, gradient) in self.second_layer_biases_parameters.0.iter_mut().zip(self.second_layer_biases_gradients.iter()) {
            *parameter = FiniteF32::try_from(parameter.0 - learning_rate.0 * gradient.0)?;
        }
        Ok(())
    }
}
struct PreviousCharactersEmbeddingsFlattenedAndFirstAndSecondLayerOutputs {
    previous_characters_embeddings_flattened: [FiniteF32; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    // Выходы первого слоя после SiLU; длина равна first_layer_biases_parameters.len().
    first_layer_outputs: [FiniteF32; FIRST_LAYER_SIZE],
    // Суммы до SiLU, необходимые для вычисления её производной.
    first_layer_inputs: [FiniteF32; FIRST_LAYER_SIZE],
    // Выходы второго слоя — оценки символов до softmax; длина равна second_layer_biases_parameters.len().
    second_layer_outputs: NonEmptyVec<FiniteF32>,
}
struct Silu;
impl Silu {
    // Разные ветви не допускают переполнения exp для большого отрицательного входа.
    fn sigmoid(input: f32) -> f32 {
        if input >= ZERO_F32 {
            1.0 / (1.0 + (-input).exp())
        } else {
            let exponential = input.exp();
            exponential / (1.0 + exponential)
        }
    }
    // SiLU(x) = x * sigmoid(x). Функция не ограничивает выход диапазоном [-1, 1].
    // {"input": 0.5, "sigmoid": 0.622459, "return_value": 0.311230}
    fn forward_activation(input: FiniteF32) -> Result<FiniteF32, &'static str> {
        FiniteF32::try_from(input.0 * Self::sigmoid(input.0))
    }
    // Производная SiLU по сохранённому входу x: s + x * s * (1 - s), где s = sigmoid(x).
    // {"input": 0.5, "output_gradient": 0.2, "silu_derivative": 0.739961, "return_value": 0.147992}
    fn backward_input_gradient(input: f32, output_gradient: FiniteF32) -> Result<FiniteF32, &'static str> {
        let sigmoid = Self::sigmoid(input);
        FiniteF32::try_from(output_gradient.0 * (sigmoid + input * sigmoid * (1.0 - sigmoid)))
    }
}
struct SoftmaxCrossEntropy;
impl SoftmaxCrossEntropy {
    // Возвращаем вероятности следующего символа из выходов второго слоя и ошибку -ln(p цели).
    // Это ошибка одного примера; усреднение по выборке или батчу выполняется снаружи.
    // Для компактности здесь учебный словарь из трёх символов; в модели урока их 27.
    // Все массивы в этом примере полные. Целевой символ имеет индекс 1.
    // {"second_layer_outputs": [1.0, 2.0, 3.0], "next_vocabulary_character_usize_index": 1}
    fn forward_next_character_probabilities_and_negative_log_likelihood_loss(
        second_layer_outputs: NonEmptySlice<'_, FiniteF32>,
        next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize,
    ) -> Result<NextCharacterProbabilitiesFromSecondLayerOutputsAndNegativeLogLikelihoodLoss, &'static str> {
        let (next_character_probabilities_from_second_layer_outputs, maximum_second_layer_output, exponentials_from_second_layer_outputs_minus_maximum_sum): (
            NonEmptyVec<CharacterProbabilityAsF32From0To1>,
            FiniteF32,
            ExponentialsSumAsFiniteF32AtLeast1,
        ) = {
            // Вычитаем максимум перед exp: наибольшая экспонента будет 1, вероятности не изменятся.
            // Максимум может быть отрицательным, но конечен: срез непустой и все элементы — FiniteF32.
            // NEG_INFINITY — только начальное значение fold; первый элемент заменит его.
            // {"second_layer_outputs": [1.0, 2.0, 3.0], "maximum_second_layer_output": 3.0}
            let maximum_second_layer_output: FiniteF32 = FiniteF32::try_from(
                second_layer_outputs
                    .iter()
                    .map(|second_layer_output: &FiniteF32| -> f32 { second_layer_output.0 })
                    .fold(f32::NEG_INFINITY, f32::max),
            )?;
            // Разность с максимумом не положительна, поэтому exp лежит в [0, 1].
            // Ноль возможен из-за округления f32; пока это экспоненты, ещё не вероятности.
            // {"second_layer_outputs_minus_maximum": [-2.0, -1.0, 0.0], "exponentials_from_second_layer_outputs_minus_maximum": [0.135335, 0.367879, 1.0]}
            let exponentials_from_second_layer_outputs_minus_maximum: NonEmptyVec<ExponentialFromOutputMinusMaximumAsF32From0To1> = NonEmptyVec::try_from(
                second_layer_outputs
                    .iter()
                    .map(
                        |second_layer_output: &FiniteF32| -> Result<ExponentialFromOutputMinusMaximumAsF32From0To1, &'static str> {
                            ExponentialFromOutputMinusMaximumAsF32From0To1::try_from((second_layer_output.0 - maximum_second_layer_output.0).exp())
                        },
                    )
                    .collect::<Result<Vec<ExponentialFromOutputMinusMaximumAsF32From0To1>, &'static str>>()?,
            )?;
            // {"exponentials_from_second_layer_outputs_minus_maximum_sum": 1.503215}
            // Экспонента максимального выхода равна 1, поэтому сумма не меньше 1.
            // ExponentialsSumAsFiniteF32AtLeast1 проверяет эту границу и конечность суммы.
            let exponentials_from_second_layer_outputs_minus_maximum_sum: ExponentialsSumAsFiniteF32AtLeast1 = ExponentialsSumAsFiniteF32AtLeast1::try_from(
                exponentials_from_second_layer_outputs_minus_maximum
                    .iter()
                    .map(|exponential| exponential.0)
                    .sum::<f32>(),
            )?;
            // Делим каждую экспоненту на общую сумму: получаем распределение по символам.
            // {"next_character_probabilities_from_second_layer_outputs": [0.090031, 0.244728, 0.665241], "probabilities_sum": 1.0}
            let next_character_probabilities_from_second_layer_outputs: NonEmptyVec<CharacterProbabilityAsF32From0To1> = NonEmptyVec::try_from(
                exponentials_from_second_layer_outputs_minus_maximum
                    .into_iter()
                    .map(
                        |exponential: ExponentialFromOutputMinusMaximumAsF32From0To1| -> Result<CharacterProbabilityAsF32From0To1, &'static str> {
                            CharacterProbabilityAsF32From0To1::try_from(exponential.0 / exponentials_from_second_layer_outputs_minus_maximum_sum.0)
                        },
                    )
                    .collect::<Result<Vec<CharacterProbabilityAsF32From0To1>, &'static str>>()?,
            )?;
            (
                next_character_probabilities_from_second_layer_outputs,
                maximum_second_layer_output,
                exponentials_from_second_layer_outputs_minus_maximum_sum,
            )
        };
        let negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32 = {
            // Берём оценку правильного символа, а не максимальную оценку.
            // {"next_vocabulary_character_usize_index": 1, "second_layer_output_for_target_character": 2.0}
            let second_layer_output_for_target_character: f32 = second_layer_outputs
                .get(next_vocabulary_character_usize_index.0)
                .ok_or("индекс целевого символа выходит за границы оценок")?
                .0;
            // Обозначим m = maximum_second_layer_output, s = second_layer_output_for_target_character,
            // S = exponentials_from_second_layer_outputs_minus_maximum_sum.
            // Softmax даёт p цели = exp(s - m) / S. Раскрываем минус логарифм:
            // -ln(p цели) = -[ln(exp(s - m)) - ln(S)] = -[s - m - ln(S)] = (m - s) + ln(S).
            // Плюс перед ln(S) получается из двух минусов; всё выражение равно -ln(p цели).
            // Не вычисляем логарифм вероятности, которая могла округлиться до нуля.
            // {"maximum_second_layer_output": 3.0, "second_layer_output_for_target_character": 2.0, "log_exponentials_sum": 0.407606, "negative_log_likelihood_loss": 1.407606}
            let negative_log_likelihood_loss: f32 =
                (maximum_second_layer_output.0 - second_layer_output_for_target_character) + exponentials_from_second_layer_outputs_minus_maximum_sum.0.ln();
            CrossEntropyLossAsNonNegativeF32::try_from(negative_log_likelihood_loss)?
        };
        // {"return_value": {"negative_log_likelihood_loss": 1.407606, "next_character_probabilities_from_second_layer_outputs": [0.090031, 0.244728, 0.665241]}}
        Ok(NextCharacterProbabilitiesFromSecondLayerOutputsAndNegativeLogLikelihoodLoss {
            negative_log_likelihood_loss,
            next_character_probabilities_from_second_layer_outputs,
        })
    }
    // Продолжаем пример выше. training_batch_len — число примеров в батче, а не размер словаря.
    // Для средней ошибки батча производную каждого примера делим на training_batch_len.
    // {"next_character_probabilities_from_second_layer_outputs_and_negative_log_likelihood_loss": {"negative_log_likelihood_loss": 1.407606, "next_character_probabilities_from_second_layer_outputs": [0.090031, 0.244728, 0.665241]}, "target_vocabulary_character_index_as_usize": 1, "training_batch_len": 32}
    // {"return_value": [{"Other": 0.002813}, {"Target": -0.023602}, {"Other": 0.020789}]}
    fn backward_next_character_score_gradients_as_f32_from_minus1_to1(
        next_character_probabilities_from_second_layer_outputs_and_negative_log_likelihood_loss: &NextCharacterProbabilitiesFromSecondLayerOutputsAndNegativeLogLikelihoodLoss,
        target_vocabulary_character_index_as_usize: VocabularyCharacterIndexAsUsize,
        training_batch_len: std::num::NonZeroUsize,
    ) -> Result<NonEmptyVec<NextCharacterScoreGradientAsF32FromMinus1To1>, &'static str> {
        // {"training_batch_len_before_conversion": 32, "training_batch_len": 32.0}
        let training_batch_len: f32 = training_batch_len.get() as f32;
        NonEmptyVec::try_from(
            next_character_probabilities_from_second_layer_outputs_and_negative_log_likelihood_loss
                .next_character_probabilities_from_second_layer_outputs
                .iter()
                .enumerate()
                .map(
                    |(next_character_probability_from_second_layer_outputs_index, next_character_probability_from_second_layer_outputs): (usize, &CharacterProbabilityAsF32From0To1)| -> Result<NextCharacterScoreGradientAsF32FromMinus1To1, &'static str> {
                        Ok(if next_character_probability_from_second_layer_outputs_index == target_vocabulary_character_index_as_usize.0 {
                            // Для целевого символа: (p - 1) / 32; отрицательный градиент повышает его оценку при SGD.
                            // {"next_character_probability_from_second_layer_outputs_index": 1, "next_character_probability_from_second_layer_outputs": 0.244728, "target_vocabulary_character_index_as_usize": 1, "gradient": -0.023602}
                            NextCharacterScoreGradientAsF32FromMinus1To1::Target(
                                TargetScoreGradientAsF32FromMinus1To0::try_from((next_character_probability_from_second_layer_outputs.0 - 1.0) / training_batch_len)?,
                            )
                        } else {
                            // Для остальных символов: p / 32; положительный градиент понижает их оценки при SGD.
                            // {"next_character_probability_from_second_layer_outputs_index": 0, "next_character_probability_from_second_layer_outputs": 0.090031, "target_vocabulary_character_index_as_usize": 1, "gradient": 0.002813}
                            NextCharacterScoreGradientAsF32FromMinus1To1::Other(
                                CharacterProbabilityAsF32From0To1::try_from(next_character_probability_from_second_layer_outputs.0 / training_batch_len)?,
                            )
                        })
                    },
                )
                .collect::<Result<Vec<NextCharacterScoreGradientAsF32FromMinus1To1>, &'static str>>()?,
        )
    }
}
// Результат для одного контекста и одного целевого символа; вероятности идут в порядке словаря.
struct NextCharacterProbabilitiesFromSecondLayerOutputsAndNegativeLogLikelihoodLoss {
    negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32,
    next_character_probabilities_from_second_layer_outputs: NonEmptyVec<CharacterProbabilityAsF32From0To1>,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Name<'a>(&'a str);
impl<'a> TryFrom<&'a str> for Name<'a> {
    type Error = &'static str;
    fn try_from(text: &'a str) -> Result<Self, Self::Error> {
        if text.is_empty() || !text.chars().all(|character: char| -> bool { character.is_ascii_lowercase() }) {
            Err("имя должно быть непустым и содержать только буквы a–z")
        } else {
            Ok(Self(text))
        }
    }
}
impl std::fmt::Debug for Name<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.0, formatter)
    }
}
#[derive(Clone, Copy, Debug)]
struct VocabularyCharacter {
    character: char,
    index: VocabularyCharacterIndexAsUsize,
}
struct Vocabulary(NonEmptyVec<VocabularyCharacter>);
impl Vocabulary {
    fn new(text: &str) -> Result<Self, &'static str> {
        let mut characters: Vec<char> = text
            .chars()
            .filter(|character: &char| -> bool { character.is_ascii_lowercase() })
            .collect::<Vec<char>>();
        characters.sort_unstable();
        characters.dedup();
        characters.insert(0, '.');
        let vocabulary_characters: Vec<VocabularyCharacter> = characters
            .into_iter()
            .enumerate()
            .map(|(index, character): (usize, char)| -> VocabularyCharacter {
                VocabularyCharacter {
                    character,
                    index: VocabularyCharacterIndexAsUsize(index),
                }
            })
            .collect();
        Ok(Self(NonEmptyVec::try_from(vocabulary_characters)?))
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn dot_index(&self) -> VocabularyCharacterIndexAsUsize {
        self.0[0].index
    }
    fn index_of(&self, character: char) -> Result<VocabularyCharacterIndexAsUsize, &'static str> {
        self.0
            .iter()
            .find(|candidate: &&VocabularyCharacter| -> bool { candidate.character == character })
            .map(|entry: &VocabularyCharacter| -> VocabularyCharacterIndexAsUsize { entry.index })
            .ok_or("символ отсутствует в словаре")
    }
    fn character(&self, index: VocabularyCharacterIndexAsUsize) -> Result<char, &'static str> {
        self.0
            .get(index.0)
            .map(|entry: &VocabularyCharacter| -> char { entry.character })
            .ok_or("индекс символа должен быть меньше размера словаря")
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct VocabularyCharacterIndexAsUsize(usize);
impl TryFrom<(usize, &Vocabulary)> for VocabularyCharacterIndexAsUsize {
    type Error = &'static str;
    fn try_from((value, vocabulary): (usize, &Vocabulary)) -> Result<Self, Self::Error> {
        vocabulary
            .0
            .get(value)
            .map(|_: &VocabularyCharacter| -> Self { Self(value) })
            .ok_or("индекс символа должен быть меньше размера словаря")
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct PreviousVocabularyCharacterIndexesAndNextCharacterIndex {
    previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize,
}
// Сумма вкладов (p - y) / N одного батча после обнуления лежит в [-1, 1].
// Ограничение не распространяется на градиенты весов и первого слоя.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct SecondLayerBiasGradientAsF32FromMinus1To1(f32);
impl SecondLayerBiasGradientAsF32FromMinus1To1 {
    fn zero() -> Self {
        Self(0.0)
    }
}
impl TryFrom<f32> for SecondLayerBiasGradientAsF32FromMinus1To1 {
    type Error = &'static str;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        // Допуск для деления и последовательного сложения не более BATCH_SIZE вкладов.
        // Небольшой выход за границу округляем обратно к ней; храним строго [-1, 1].
        // Значительное нарушение диапазона, NaN и бесконечности возвращают ошибку.
        let rounding_tolerance = (BATCH_SIZE as f32 + 2.0) * f32::EPSILON;
        if value.is_finite() && (-1.0 - rounding_tolerance..=1.0 + rounding_tolerance).contains(&value) {
            Ok(Self(value.clamp(-1.0, 1.0)))
        } else {
            Err("градиент смещения второго слоя должен быть от -1 до 1")
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct FiniteF32(f32);
impl FiniteF32 {
    fn zero() -> Self {
        Self(ZERO_F32)
    }
}
impl TryFrom<f32> for FiniteF32 {
    type Error = &'static str;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_finite() {
            Ok(Self(value))
        } else {
            Err("значение должно быть конечным числом")
        }
    }
}
// Одна экспонента exp(выход - максимум) до нормализации; это ещё не вероятность символа.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct ExponentialFromOutputMinusMaximumAsF32From0To1(f32);
impl TryFrom<f32> for ExponentialFromOutputMinusMaximumAsF32From0To1 {
    type Error = &'static str;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        // exp(output - maximum) лежит в [0, 1]; ноль возможен из-за округления f32.
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err("экспонента выхода после вычитания максимума должна быть от 0 до 1")
        }
    }
}
// Сумма экспонент после вычитания максимума: конечный знаменатель softmax, не меньше 1.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct ExponentialsSumAsFiniteF32AtLeast1(f32);
impl TryFrom<f32> for ExponentialsSumAsFiniteF32AtLeast1 {
    type Error = &'static str;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if value.is_finite() && value >= 1.0 {
            Ok(Self(value))
        } else {
            Err("сумма экспонент после вычитания максимума должна быть конечной и не меньше 1")
        }
    }
}
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
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct TargetScoreGradientAsF32FromMinus1To0(f32);
impl TryFrom<f32> for TargetScoreGradientAsF32FromMinus1To0 {
    type Error = &'static str;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        if (-1.0..=ZERO_F32).contains(&value) {
            Ok(Self(value))
        } else {
            Err("градиент правильного символа должен быть от −1 до 0")
        }
    }
}
#[derive(Clone, Copy, Debug)]
enum NextCharacterScoreGradientAsF32FromMinus1To1 {
    Target(TargetScoreGradientAsF32FromMinus1To0),
    Other(CharacterProbabilityAsF32From0To1),
}
impl From<NextCharacterScoreGradientAsF32FromMinus1To1> for f32 {
    fn from(gradient: NextCharacterScoreGradientAsF32FromMinus1To1) -> Self {
        match gradient {
            NextCharacterScoreGradientAsF32FromMinus1To1::Target(value) => value.0,
            NextCharacterScoreGradientAsF32FromMinus1To1::Other(value) => value.0,
        }
    }
}
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
    fn map<U>(&self, transform: impl FnMut(&T) -> U) -> NonEmptyVec<U> {
        NonEmptyVec(self.iter().map(transform).collect::<Vec<U>>())
    }
}
impl<T> NonEmptyVec<T> {
    fn try_map<U>(&self, mut transform: impl FnMut(&T) -> Result<U, &'static str>) -> Result<NonEmptyVec<U>, &'static str> {
        Ok(NonEmptyVec(self.iter().map(&mut transform).collect::<Result<Vec<U>, &'static str>>()?))
    }
}
#[derive(Debug, derive_more::Deref)]
#[deref(forward)]
#[derive_where::derive_where(Clone, Copy)]
struct NonEmptySlice<'a, T>(&'a [T]);
impl<'a, T> TryFrom<&'a [T]> for NonEmptySlice<'a, T> {
    type Error = &'static str;
    fn try_from(values: &'a [T]) -> Result<Self, Self::Error> {
        if values.is_empty() {
            Err("срез не должен быть пустым")
        } else {
            Ok(Self(values))
        }
    }
}
impl<'a, T> From<&'a NonEmptyVec<T>> for NonEmptySlice<'a, T> {
    fn from(values: &'a NonEmptyVec<T>) -> Self {
        Self(&values.0)
    }
}
fn try_map_array<T, U, const N: usize>(values: [T; N], transform: impl FnMut(T) -> Result<U, &'static str>) -> Result<[U; N], &'static str> {
    values
        .into_iter()
        .map(transform)
        .collect::<Result<Vec<U>, &'static str>>()?
        .try_into()
        .map_err(|_: Vec<U>| -> &'static str { "при преобразовании изменилась длина массива" })
}
