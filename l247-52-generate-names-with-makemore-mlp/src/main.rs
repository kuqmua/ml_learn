const PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION: usize = 3;
const ZERO_F32: f32 = 0.0;
const EMBEDDING_DIMENSIONS: usize = 10;
const TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION: usize =
    PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION * EMBEDDING_DIMENSIONS;
const FIRST_LAYER_SIZE: usize = 200;
const BATCH_SIZE: usize = 32;
const _: () = assert!(PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION > 0 && EMBEDDING_DIMENSIONS > 0 && FIRST_LAYER_SIZE > 0 && BATCH_SIZE > 0);
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let training_steps: std::num::NonZeroUsize = std::num::NonZeroUsize::new(200_000).ok_or("число шагов должно быть положительным")?;
    let mut random_generator: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(42);
    // {
    //   "vocabulary": [
    //     {"character": ".", "index": 0},
    //     {"character": "a", "index": 1},
    //     {"character": "e", "index": 2},
    //     {"character": "i", "index": 3},
    //     {"character": "l", "index": 4},
    //     {"character": "m", "index": 5},
    //     {"character": "n", "index": 6},
    //     {"character": "o", "index": 7},
    //     {"character": "v", "index": 8}
    //   ],
    //   "training_character_index_pairs": [
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 2},
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 2], "next_vocabulary_character_usize_index": 5},
    //     {"previous_vocabulary_character_usize_indexes": [0, 2, 5], "next_vocabulary_character_usize_index": 5},
    //     {"previous_vocabulary_character_usize_indexes": [2, 5, 5], "next_vocabulary_character_usize_index": 1}
    //   ],
    //   "validation_character_index_pairs": [
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 5},
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 5], "next_vocabulary_character_usize_index": 3},
    //     {"previous_vocabulary_character_usize_indexes": [0, 5, 3], "next_vocabulary_character_usize_index": 1},
    //     {"previous_vocabulary_character_usize_indexes": [5, 3, 1], "next_vocabulary_character_usize_index": 0}
    //   ],
    //   "test_character_index_pairs": [
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 1},
    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 1], "next_vocabulary_character_usize_index": 8},
    //     {"previous_vocabulary_character_usize_indexes": [0, 1, 8], "next_vocabulary_character_usize_index": 1},
    //     {"previous_vocabulary_character_usize_indexes": [1, 8, 1], "next_vocabulary_character_usize_index": 0}
    //   ],
    // }
    let (vocabulary, training_character_index_pairs, validation_character_index_pairs, test_character_index_pairs): (
        Vocabulary,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
        NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex>,
    ) = {
        // {"names_text": "emma\nanna\nemma\nolivia\namelia\neva\nmila\nnina\nnoel\nmia\nava\n"}
        let names_text: &str = include_str!("../data/names.txt");
        // {"names": ["emma", "anna", "emma", "olivia", "amelia", "eva", "mila", "nina", "noel", "mia", "ava"]}
        let names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(names_text.lines().map(Name::try_from).collect::<Result<Vec<Name<'_>>, &'static str>>()?)?;
        // {
        //   "name_groups": [["amelia"], ["anna"], ["ava"], ["emma", "emma"], ["eva"], ["mia"], ["mila"], ["nina"], ["noel"], ["olivia"]]
        // }
        let mut name_groups: NonEmptyVec<NonEmptyVec<Name<'_>>> = {
            let mut groups: std::collections::BTreeMap<Name<'_>, Vec<Name<'_>>> = std::collections::BTreeMap::new();
            for &name in names.iter() {
                // {
                //   "name": "emma",
                //   "before": {"groups": {"anna": ["anna"], "emma": ["emma"]}},
                //   "after": {"groups": {"anna": ["anna"], "emma": ["emma", "emma"]}}
                // }
                groups.entry(name).or_default().push(name);
            }
            NonEmptyVec::try_from(
                groups
                    .into_values()
                    .map(NonEmptyVec::try_from)
                    .collect::<Result<Vec<NonEmptyVec<Name<'_>>>, &'static str>>()?,
            )?
        };
        // {
        //   "before": {
        //     "name_groups": [["amelia"], ["anna"], ["ava"], ["emma", "emma"], ["eva"], ["mia"], ["mila"], ["nina"], ["noel"], ["olivia"]]
        //   },
        //   "possible_after": {
        //     "name_groups": [["emma", "emma"], ["anna"], ["olivia"], ["amelia"], ["eva"], ["mila"], ["nina"], ["noel"], ["mia"], ["ava"]]
        //   }
        // }
        rand::seq::SliceRandom::shuffle(name_groups.0.as_mut_slice(), &mut random_generator);
        // {
        //   "training_name_groups": [["emma", "emma"], ["anna"], ["olivia"], ["amelia"], ["eva"], ["mila"], ["nina"], ["noel"]],
        //   "remaining_name_groups": [["mia"], ["ava"]]
        // }
        let (training_name_groups, remaining_name_groups): (&[NonEmptyVec<Name<'_>>], &[NonEmptyVec<Name<'_>>]) =
            name_groups.split_at(name_groups.len() * 8 / 10);
        // {"validation_name_groups": [["mia"]], "test_name_groups": [["ava"]]}
        let (validation_name_groups, test_name_groups): (&[NonEmptyVec<Name<'_>>], &[NonEmptyVec<Name<'_>>]) =
            remaining_name_groups.split_at(name_groups.len() * 9 / 10 - training_name_groups.len());
        // {"training_names": ["emma", "emma", "anna", "olivia", "amelia", "eva", "mila", "nina", "noel"]}
        let training_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            training_name_groups
                .iter()
                .flat_map(|group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> { group.iter() })
                .copied()
                .collect::<Vec<Name<'_>>>(),
        )?;
        // {"validation_names": ["mia"]}
        let validation_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            validation_name_groups
                .iter()
                .flat_map(|group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> { group.iter() })
                .copied()
                .collect::<Vec<Name<'_>>>(),
        )?;
        // {"test_names": ["ava"]}
        let test_names: NonEmptyVec<Name<'_>> = NonEmptyVec::try_from(
            test_name_groups
                .iter()
                .flat_map(|group: &NonEmptyVec<Name<'_>>| -> std::slice::Iter<'_, Name<'_>> { group.iter() })
                .copied()
                .collect::<Vec<Name<'_>>>(),
        )?;
        // {
        //   "vocabulary": [
        //     {"character": ".", "index": 0},
        //     {"character": "a", "index": 1},
        //     {"character": "e", "index": 2},
        //     {"character": "i", "index": 3},
        //     {"character": "l", "index": 4},
        //     {"character": "m", "index": 5},
        //     {"character": "n", "index": 6},
        //     {"character": "o", "index": 7},
        //     {"character": "v", "index": 8}
        //   ]
        // }
        let vocabulary: Vocabulary = Vocabulary::new(&training_names.map(|name: &Name<'_>| -> &str { name.0 }).join("\n"))?;
        // {
        //   "names": ["emma"],
        //   "return_value": [
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 2},
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 2], "next_vocabulary_character_usize_index": 5},
        //     {"previous_vocabulary_character_usize_indexes": [0, 2, 5], "next_vocabulary_character_usize_index": 5},
        //     {"previous_vocabulary_character_usize_indexes": [2, 5, 5], "next_vocabulary_character_usize_index": 1}
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
                        // {"next_character": "e", "next_vocabulary_character_usize_index": 2}
                        let next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize = vocabulary.index_of(next_character)?;
                        // {
                        //   "before": {"character_index_pairs": []},
                        //   "after": {
                        //     "character_index_pairs": [{"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 2}]
                        //   }
                        // }
                        character_index_pairs.push(PreviousVocabularyCharacterIndexesAndNextCharacterIndex {
                            previous_vocabulary_character_usize_indexes,
                            next_vocabulary_character_usize_index,
                        });
                        // {
                        //   "before": {"previous_vocabulary_character_usize_indexes": [0, 0, 0]},
                        //   "after": {"previous_vocabulary_character_usize_indexes": [0, 0, 0]}
                        // }
                        previous_vocabulary_character_usize_indexes.rotate_left(1);
                        // {
                        //   "next_vocabulary_character_usize_index": 2,
                        //   "before": {"previous_vocabulary_character_usize_indexes": [0, 0, 0]},
                        //   "after": {"previous_vocabulary_character_usize_indexes": [0, 0, 2]}
                        // }
                        previous_vocabulary_character_usize_indexes[PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION - 1] =
                            next_vocabulary_character_usize_index;
                    }
                }
                NonEmptyVec::try_from(character_index_pairs)
            };
        // {
        //   "training_character_index_pairs": [
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 2},
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 2], "next_vocabulary_character_usize_index": 5},
        //     {"previous_vocabulary_character_usize_indexes": [0, 2, 5], "next_vocabulary_character_usize_index": 5},
        //     {"previous_vocabulary_character_usize_indexes": [2, 5, 5], "next_vocabulary_character_usize_index": 1}
        //   ],
        //   "omitted_elements": 44
        // }
        let training_character_index_pairs: NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex> =
            build_previous_vocabulary_character_indexes_and_next_character_indexes(NonEmptySlice::from(&training_names))?;
        // {
        //   "validation_character_index_pairs": [
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 5},
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 5], "next_vocabulary_character_usize_index": 3},
        //     {"previous_vocabulary_character_usize_indexes": [0, 5, 3], "next_vocabulary_character_usize_index": 1},
        //     {"previous_vocabulary_character_usize_indexes": [5, 3, 1], "next_vocabulary_character_usize_index": 0}
        //   ]
        // }
        let validation_character_index_pairs: NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex> =
            build_previous_vocabulary_character_indexes_and_next_character_indexes(NonEmptySlice::from(&validation_names))?;
        // {
        //   "test_character_index_pairs": [
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 1},
        //     {"previous_vocabulary_character_usize_indexes": [0, 0, 1], "next_vocabulary_character_usize_index": 8},
        //     {"previous_vocabulary_character_usize_indexes": [0, 1, 8], "next_vocabulary_character_usize_index": 1},
        //     {"previous_vocabulary_character_usize_indexes": [1, 8, 1], "next_vocabulary_character_usize_index": 0}
        //   ]
        // }
        let test_character_index_pairs: NonEmptyVec<PreviousVocabularyCharacterIndexesAndNextCharacterIndex> =
            build_previous_vocabulary_character_indexes_and_next_character_indexes(NonEmptySlice::from(&test_names))?;
        (
            vocabulary,
            training_character_index_pairs,
            validation_character_index_pairs,
            test_character_index_pairs,
        )
    };
    // {"vocabulary_len": 9}
    let vocabulary_len: usize = vocabulary.len();
    // {
    //   "parameter_shapes": {
    //     "character_embeddings_parameters": [9, 10],
    //     "first_layer_weights_parameters": [30, 200],
    //     "first_layer_biases_parameters": [200],
    //     "second_layer_weights_parameters": [9, 200],
    //     "second_layer_biases_parameters": [9]
    //   },
    //   "gradient_shapes_match_parameters": true,
    //   "initial_gradient_value": 0.0
    // }
    let mut model: Model = {
        // {"distribution": "StandardNormal", "mean": 0.0, "standard_deviation": 1.0, "return_value": -0.37}
        let mut initialize_parameter_standard_normal = || -> Result<FiniteF32, &'static str> {
            FiniteF32::try_from(rand_distr::Distribution::<f32>::sample(&rand_distr::StandardNormal, &mut random_generator))
        };
        // {
        //   "vocabulary_len": 9,
        //   "return_shape": [9, 10],
        //   "first_row": [-1.732051, -1.732051, -1.732051, 0, 0, 0, 0, 0, 0, 0],
        //   "last_row": [1.732051, 1.732051, 1.732051, 0, 0, 0, 0, 0, 0, 0],
        //   "branch_enabled": false
        // }
        let initialize_embeddings_regular_cube = || -> NonEmptyVec<[f32; EMBEDDING_DIMENSIONS]> {
            // {"alternative_input": {"vocabulary_len": 1}, "return_value": [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]]}
            if vocabulary_len < 2 {
                return vocabulary
                    .0
                    .map(|_: &VocabularyCharacter| -> [f32; EMBEDDING_DIMENSIONS] { [ZERO_F32; EMBEDDING_DIMENSIONS] });
            }
            // {"grid_dimensions": 3}
            let grid_dimensions: usize = EMBEDDING_DIMENSIONS.min(3);
            // {"levels_per_axis": 1}
            let mut levels_per_axis: usize = 1;
            // {"vocabulary_len": 9, "levels_per_axis_sequence": [1, 2, 3], "node_counts": [1, 8, 27]}
            while levels_per_axis.pow(grid_dimensions as u32) < vocabulary_len {
                // {"before": {"levels_per_axis": 2}, "after": {"levels_per_axis": 3}}
                levels_per_axis += 1;
            }
            // {"node_count": 27}
            let node_count: usize = levels_per_axis.pow(grid_dimensions as u32);
            // {"half_width": 1.732051}
            let half_width: f32 = 3.0f32.sqrt();
            // {"symbol_index": 0}
            let mut symbol_index: usize = 0;
            vocabulary.0.map(|_: &VocabularyCharacter| -> [f32; EMBEDDING_DIMENSIONS] {
                // {"symbol_index": 0, "node_count": 27, "vocabulary_len": 9, "node_index": 0}
                let mut node_index: usize = symbol_index * (node_count - 1) / (vocabulary_len - 1);
                // {"embedding": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}
                let mut embedding: [f32; EMBEDDING_DIMENSIONS] = [ZERO_F32; EMBEDDING_DIMENSIONS];
                // {"embedding_after_loop": [-1.732051, -1.732051, -1.732051, 0, 0, 0, 0, 0, 0, 0]}
                for coordinate in &mut embedding[..grid_dimensions] {
                    // {"node_index": 0, "levels_per_axis": 3, "level": 0}
                    let level: usize = node_index % levels_per_axis;
                    // {"before": {"node_index": 0}, "after": {"node_index": 0}}
                    node_index /= levels_per_axis;
                    // {"level": 0, "levels_per_axis": 3, "half_width": 1.732051, "coordinate": -1.732051}
                    *coordinate = -half_width + 2.0 * half_width * level as f32 / (levels_per_axis - 1) as f32;
                }
                // {"before": {"symbol_index": 0}, "after": {"symbol_index": 1}}
                symbol_index += 1;
                embedding
            })
        };
        // {"zero_gradient": 0.0}
        let zero_gradient: FiniteF32 = FiniteF32::zero();
        Model {
            character_embeddings_parameters: {
                // {"normal_character_embeddings_shape": [9, 10], "normal_character_embeddings_row_0_prefix": [0.4, -0.5, 0.1]}
                let normal_character_embeddings: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]> =
                    vocabulary
                        .0
                        .try_map(|_: &VocabularyCharacter| -> Result<[FiniteF32; EMBEDDING_DIMENSIONS], &'static str> {
                            try_map_array([(); EMBEDDING_DIMENSIONS], |_: ()| -> Result<FiniteF32, &'static str> {
                                initialize_parameter_standard_normal()
                            })
                        })?;
                // {"regular_cube_enabled": false, "selected_initialization": "StandardNormal"}
                if false {
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
                // {"second_layer_weights_parameters_shape": [9, 200], "initial_value": 0.0}
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
            character_embeddings_gradients: NonEmptyVec::try_from(vec![[zero_gradient; EMBEDDING_DIMENSIONS]; vocabulary_len])?,
            first_layer_weights_gradients: [[zero_gradient; FIRST_LAYER_SIZE]; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
            first_layer_biases_gradients: [zero_gradient; FIRST_LAYER_SIZE],
            second_layer_weights_gradients: NonEmptyVec::try_from(vec![[zero_gradient; FIRST_LAYER_SIZE]; vocabulary_len])?,
            second_layer_biases_gradients: NonEmptyVec::try_from(vec![zero_gradient; vocabulary_len])?,
        }
    };
    // {"per_example_losses": [0.356675, 0.356675, 0.356675, 0.356675], "return_value": 0.356675}
    let loss = move |model: &Model,
                     character_index_pairs: NonEmptySlice<'_, PreviousVocabularyCharacterIndexesAndNextCharacterIndex>|
          -> Result<CrossEntropyLossAsNonNegativeF32, &'static str> {
        // {"total_negative_log_likelihood_loss": 0.0}
        let mut total_negative_log_likelihood_loss: f64 = 0.0;
        // {
        //   "character_index_pair": {"previous_vocabulary_character_usize_indexes": [2, 5, 5], "next_vocabulary_character_usize_index": 1}
        // }
        for character_index_pair in character_index_pairs.iter() {
            // {
            //   "next_character_probabilities_and_negative_log_likelihood": {
            //     "negative_log_likelihood_loss": 0.356675,
            //     "next_character_probabilities": [0.1, 0.7, 0.04, 0.03, 0.03, 0.04, 0.02, 0.02, 0.02]
            //   }
            // }
            let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood =
                SoftmaxCrossEntropy::forward_probabilities_and_loss(
                    NonEmptySlice::from(
                        &model
                            .forward_pass(character_index_pair.previous_vocabulary_character_usize_indexes)?
                            .character_scores,
                    ),
                    character_index_pair.next_vocabulary_character_usize_index,
                )?;
            // {
            //   "before": {"total_negative_log_likelihood_loss": 0.0},
            //   "contribution": 0.356675,
            //   "after": {"total_negative_log_likelihood_loss": 0.356675}
            // }
            total_negative_log_likelihood_loss += f64::from(next_character_probabilities_and_negative_log_likelihood.negative_log_likelihood_loss.0);
        }
        CrossEntropyLossAsNonNegativeF32::try_from((total_negative_log_likelihood_loss / character_index_pairs.len() as f64) as f32)
    };
    // {
    //   "validation_example_count": 4,
    //   "per_example_losses": [0.356675, 0.356675, 0.356675, 0.356675],
    //   "_initial_validation_loss": 0.356675
    // }
    let _initial_validation_loss: CrossEntropyLossAsNonNegativeF32 = loss(&model, NonEmptySlice::from(&validation_character_index_pairs))?;
    {
        let mut batch_start: usize = 0;
        // {"first_step": 0, "last_step": 199999, "updates": 200000}
        for training_step in 0..training_steps.get() {
            // [{"training_step": 99999, "learning_rate": 0.1}, {"training_step": 100000, "learning_rate": 0.01}]
            let learning_rate: LearningRateAsPositiveFiniteF32 = LearningRateAsPositiveFiniteF32::try_from(if training_step < 100_000 { 0.1 } else { 0.01 })?;
            // {"_loss": 0.356675}
            let _loss: CrossEntropyLossAsNonNegativeF32 = {
                // {"average_negative_log_likelihood_loss": 0.356675}
                let average_negative_log_likelihood_loss: f32 = {
                    // {"training_character_index_pairs_len": 48, "batch_start": 0, "BATCH_SIZE": 32, "batch_end": 32}
                    let batch_end: usize = (batch_start + BATCH_SIZE).min(training_character_index_pairs.len());
                    // {
                    //   "previous_vocabulary_character_indexes_and_next_character_index_batch": [
                    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 0], "next_vocabulary_character_usize_index": 2},
                    //     {"previous_vocabulary_character_usize_indexes": [0, 0, 2], "next_vocabulary_character_usize_index": 5},
                    //     {"previous_vocabulary_character_usize_indexes": [0, 2, 5], "next_vocabulary_character_usize_index": 5},
                    //     {"previous_vocabulary_character_usize_indexes": [2, 5, 5], "next_vocabulary_character_usize_index": 1}
                    //   ],
                    //   "omitted_elements": 28,
                    //   "source_range": [0, 32]
                    // }
                    let previous_vocabulary_character_indexes_and_next_character_index_batch: NonEmptySlice<
                        '_,
                        PreviousVocabularyCharacterIndexesAndNextCharacterIndex,
                    > = NonEmptySlice::try_from(&training_character_index_pairs[batch_start..batch_end])?;
                    // {"batch_end": 32, "training_character_index_pairs_len": 48, "before": {"batch_start": 0}, "after": {"batch_start": 32}}
                    batch_start = if batch_end == training_character_index_pairs.len() { 0 } else { batch_end };
                    // {"zero_gradient": 0.0}
                    let zero_gradient: FiniteF32 = FiniteF32::zero();
                    // {"batch_len": 32}
                    let batch_len: std::num::NonZeroUsize =
                        std::num::NonZeroUsize::new(previous_vocabulary_character_indexes_and_next_character_index_batch.len())
                            .ok_or("батч не должен быть пустым")?;
                    // {
                    //   "before": {"second_layer_biases_gradients_prefix": [0.2, -0.1, 0.05]},
                    //   "after": {"second_layer_biases_gradients_prefix": [0.0, 0.0, 0.0]},
                    //   "all_gradient_arrays_reset": true
                    // }
                    model.zero_grad()?;
                    // {"average_negative_log_likelihood_loss": 0.356675}
                    let average_negative_log_likelihood_loss: f32 = {
                        // {"batch_len": 32, "total_negative_log_likelihood_loss": 11.413598}
                        let total_negative_log_likelihood_loss: FiniteF32 = {
                            // {"total_negative_log_likelihood_loss": 0.0}
                            let mut total_negative_log_likelihood_loss: FiniteF32 = zero_gradient;
                            // {
                            //   "previous_vocabulary_character_indexes_and_next_character_index_batch_element": {"previous_vocabulary_character_usize_indexes": [2, 5, 5], "next_vocabulary_character_usize_index": 1}
                            // }
                            for previous_vocabulary_character_indexes_and_next_character_index_batch_element in
                                previous_vocabulary_character_indexes_and_next_character_index_batch.iter()
                            {
                                // {
                                //   "forward_pass_result": {
                                //     "previous_vocabulary_character_usize_indexes": [2, 5, 5],
                                //     "character_embeddings_flattened": [0.4, -0.5, 0.1],
                                //     "first_layer_activations": [0.2, -0.5],
                                //     "character_scores": [-2.302585, -0.356675, -3.218876, -3.506558, -3.506558, -3.218876, -3.912023, -3.912023, -3.912023]
                                //   },
                                //   "omitted_elements": {"character_embeddings_flattened": 27, "first_layer_activations": 198}
                                // }
                                let forward_pass_result: ForwardPassResult = model.forward_pass(
                                    previous_vocabulary_character_indexes_and_next_character_index_batch_element.previous_vocabulary_character_usize_indexes,
                                )?;
                                // {
                                //   "next_character_probabilities_and_negative_log_likelihood": {
                                //     "negative_log_likelihood_loss": 0.356675,
                                //     "next_character_probabilities": [0.1, 0.7, 0.04, 0.03, 0.03, 0.04, 0.02, 0.02, 0.02]
                                //   }
                                // }
                                let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood =
                                    SoftmaxCrossEntropy::forward_probabilities_and_loss(
                                        NonEmptySlice::from(&forward_pass_result.character_scores),
                                        previous_vocabulary_character_indexes_and_next_character_index_batch_element.next_vocabulary_character_usize_index,
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
                                //   "batch_len": 32,
                                //   "score_gradients": [
                                //     {"Other": 0.003125},
                                //     {"Target": -0.009375},
                                //     {"Other": 0.00125},
                                //     {"Other": 0.0009375},
                                //     {"Other": 0.0009375},
                                //     {"Other": 0.00125},
                                //     {"Other": 0.000625},
                                //     {"Other": 0.000625},
                                //     {"Other": 0.000625}
                                //   ]
                                // }
                                let score_gradients: NonEmptyVec<NextCharacterScoreGradientAsF32FromMinus1To1> = SoftmaxCrossEntropy::backward_score_gradients(
                                    &next_character_probabilities_and_negative_log_likelihood,
                                    previous_vocabulary_character_indexes_and_next_character_index_batch_element.next_vocabulary_character_usize_index,
                                    batch_len,
                                )?;
                                // {
                                //   "character_index": 1,
                                //   "score_gradient": -0.009375,
                                //   "before": {"second_layer_biases_gradients_1": 0.01},
                                //   "after": {"second_layer_biases_gradients_1": 0.000625},
                                //   "parameters_changed": false
                                // }
                                model.backward_pass(&forward_pass_result, &score_gradients)?;
                                // {"parameters_updated": false, "gradients_accumulated": true}
                            }
                            total_negative_log_likelihood_loss
                        };
                        // {"total_negative_log_likelihood_loss": 11.413598, "batch_len": 32, "result": 0.356675}
                        total_negative_log_likelihood_loss.0 / batch_len.get() as f32
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
    // {"order": ["train", "dev", "test"], "_losses": [0.3, 0.4, 0.45], "parameters_updated": false}
    let _losses: [CrossEntropyLossAsNonNegativeF32; 3] = [
        loss(&model, NonEmptySlice::from(&training_character_index_pairs))?,
        loss(&model, NonEmptySlice::from(&validation_character_index_pairs))?,
        loss(&model, NonEmptySlice::from(&test_character_index_pairs))?,
    ];
    // {"iterations": 20, "names_per_iteration": 1}
    for _ in 0..20 {
        // {"_name": "emma."}
        let _name: String = 'sample: {
            // {"previous_vocabulary_character_usize_indexes": [0, 0, 0]}
            let mut previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION] =
                [vocabulary.dot_index(); PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION];
            // {"generated_name": ""}
            let mut generated_name: String = String::new();
            // {"max_sampled_characters": 100, "stop_index": 0}
            for _ in 0..100 {
                // {
                //   "target": 0,
                //   "next_character_probabilities_and_negative_log_likelihood": {
                //     "negative_log_likelihood_loss": 2.302585,
                //     "next_character_probabilities": [0.1, 0.7, 0.04, 0.03, 0.03, 0.04, 0.02, 0.02, 0.02]
                //   },
                //   "loss_used_for_sampling": false
                // }
                let next_character_probabilities_and_negative_log_likelihood: NextCharacterProbabilitiesAndNegativeLogLikelihood =
                    SoftmaxCrossEntropy::forward_probabilities_and_loss(
                        NonEmptySlice::from(&model.forward_pass(previous_vocabulary_character_usize_indexes)?.character_scores),
                        vocabulary.dot_index(),
                    )?;
                // {"random_probability": 0.65}
                let random_probability: CharacterProbabilityAsF32From0To1 =
                    CharacterProbabilityAsF32From0To1::try_from(rand::RngExt::random::<f32>(&mut random_generator))?;
                // {
                //   "random_probability": 0.65,
                //   "next_character_probabilities": [0.1, 0.7, 0.04, 0.03, 0.03, 0.04, 0.02, 0.02, 0.02],
                //   "sampled_character_index": 1
                // }
                let sampled_character_index: usize = {
                    // {"cumulative_probability": 0.0}
                    let mut cumulative_probability: f32 = ZERO_F32;
                    // {"vocabulary_len": 9, "sampled_character_index": 8}
                    let mut sampled_character_index: usize = vocabulary_len - 1;
                    for next_character_probability_with_index in next_character_probabilities_and_negative_log_likelihood
                        .next_character_probabilities
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
                //   "before": {"previous_vocabulary_character_usize_indexes": [2, 5, 5]},
                //   "after": {"previous_vocabulary_character_usize_indexes": [5, 5, 2]}
                // }
                previous_vocabulary_character_usize_indexes.rotate_left(1);
                // {
                //   "sampled_character_index": 1,
                //   "before": {"previous_vocabulary_character_usize_indexes": [5, 5, 2]},
                //   "after": {"previous_vocabulary_character_usize_indexes": [5, 5, 1]}
                // }
                previous_vocabulary_character_usize_indexes[PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION - 1] = sampled_character_index;
            }
            // {"generated_name_len_before": 100, "appended_suffix": " [лимит 100 символов]"}
            generated_name.push_str(" [лимит 100 символов]");
            generated_name
        };
    }
    // {"Ok": null}
    Ok(())
}
#[derive(Clone)]
struct Model {
    character_embeddings_parameters: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
    first_layer_weights_parameters: [[FiniteF32; FIRST_LAYER_SIZE]; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    first_layer_biases_parameters: [FiniteF32; FIRST_LAYER_SIZE],
    second_layer_weights_parameters: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
    second_layer_biases_parameters: NonEmptyVec<FiniteF32>,
    character_embeddings_gradients: NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]>,
    first_layer_weights_gradients: [[FiniteF32; FIRST_LAYER_SIZE]; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    first_layer_biases_gradients: [FiniteF32; FIRST_LAYER_SIZE],
    second_layer_weights_gradients: NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]>,
    second_layer_biases_gradients: NonEmptyVec<FiniteF32>,
}
impl Model {
    fn forward_pass(
        &self,
        previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    ) -> Result<ForwardPassResult, &'static str> {
        let character_embeddings_flattened: [FiniteF32; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION] = {
            let context_embeddings: [[FiniteF32; EMBEDDING_DIMENSIONS]; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION] =
                previous_vocabulary_character_usize_indexes
                    .map(|index: VocabularyCharacterIndexAsUsize| -> [FiniteF32; EMBEDDING_DIMENSIONS] { self.character_embeddings_parameters[index.0] });
            context_embeddings
                .as_flattened()
                .try_into()
                .map_err(|_: std::array::TryFromSliceError| -> &'static str { "неверная длина объединённых эмбеддингов" })?
        };
        let first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE] = {
            let mut first_layer_activations: [f32; FIRST_LAYER_SIZE] = self.first_layer_biases_parameters.map(|value: FiniteF32| -> f32 { value.0 });
            for character_embedding_flattened in character_embeddings_flattened.iter().enumerate() {
                let (context_embedding_coordinate_index, &context_embedding_coordinate): (usize, &FiniteF32) = character_embedding_flattened;
                for first_layer_activation_and_weight in first_layer_activations
                    .iter_mut()
                    .zip(&self.first_layer_weights_parameters[context_embedding_coordinate_index])
                {
                    let (first_layer_activation, &first_layer_weight): (&mut f32, &FiniteF32) = first_layer_activation_and_weight;
                    *first_layer_activation += context_embedding_coordinate.0 * first_layer_weight.0;
                }
            }
            for first_layer_activation in &mut first_layer_activations {
                *first_layer_activation = Tanh::forward_activation(FiniteF32::try_from(*first_layer_activation)?)?.0;
            }
            try_map_array(first_layer_activations, FiniteF32::try_from)?
        };
        let character_scores: NonEmptyVec<f32> = {
            let mut character_scores: NonEmptyVec<f32> = self.second_layer_biases_parameters.map(|value: &FiniteF32| -> f32 { value.0 });
            for first_layer_activation_with_index in first_layer_activations.iter().enumerate() {
                let (first_layer_index, &first_layer_activation): (usize, &FiniteF32) = first_layer_activation_with_index;
                for character_score_and_weights in character_scores.0.iter_mut().zip(self.second_layer_weights_parameters.iter()) {
                    let (character_score, character_weights): (&mut f32, &[FiniteF32; FIRST_LAYER_SIZE]) = character_score_and_weights;
                    *character_score += first_layer_activation.0 * character_weights[first_layer_index].0;
                }
            }
            character_scores
        };
        let character_scores: NonEmptyVec<FiniteF32> = NonEmptyVec::try_from(
            character_scores
                .into_iter()
                .map(FiniteF32::try_from)
                .collect::<Result<Vec<FiniteF32>, &'static str>>()?,
        )?;
        Ok(ForwardPassResult {
            previous_vocabulary_character_usize_indexes,
            character_embeddings_flattened,
            first_layer_activations,
            character_scores,
        })
    }
    fn zero_grad(&mut self) -> Result<(), &'static str> {
        let zero_gradient: FiniteF32 = FiniteF32::zero();
        self.character_embeddings_gradients.0.as_flattened_mut().fill(zero_gradient);
        self.first_layer_weights_gradients.as_flattened_mut().fill(zero_gradient);
        self.first_layer_biases_gradients.fill(zero_gradient);
        self.second_layer_weights_gradients.0.as_flattened_mut().fill(zero_gradient);
        self.second_layer_biases_gradients.0.fill(zero_gradient);
        Ok(())
    }
    fn backward_pass(
        &mut self,
        forward_pass_result: &ForwardPassResult,
        next_character_score_gradients_as_f32_from_minus1_to1: &NonEmptyVec<NextCharacterScoreGradientAsF32FromMinus1To1>,
    ) -> Result<(), &'static str> {
        let zero_gradient: FiniteF32 = FiniteF32::zero();
        let second_layer_biases_parameters_len: usize = self.second_layer_biases_parameters.len();
        fn accumulate_gradient_product(
            gradient: &mut FiniteF32,
            input_or_weight: f32,
            output_gradient: f32,
            location: &'static str,
        ) -> Result<(), &'static str> {
            let contribution: FiniteF32 = FiniteF32::try_from(input_or_weight * output_gradient).map_err(|_: &'static str| -> &'static str { location })?;
            accumulate_gradient(gradient, contribution.0, location)
        }
        fn accumulate_gradient(gradient: &mut FiniteF32, contribution: f32, location: &'static str) -> Result<(), &'static str> {
            *gradient = FiniteF32::try_from(gradient.0 + contribution).map_err(|_: &'static str| -> &'static str { location })?;
            Ok(())
        }
        {
            let second_layer_biases_gradients: &mut NonEmptyVec<FiniteF32> = &mut self.second_layer_biases_gradients;
            for (second_layer_bias_gradient, &next_character_score_gradient_as_f32_from_minus1_to1) in second_layer_biases_gradients
                .0
                .iter_mut()
                .zip(next_character_score_gradients_as_f32_from_minus1_to1.iter())
            {
                accumulate_gradient(
                    second_layer_bias_gradient,
                    f32::from(next_character_score_gradient_as_f32_from_minus1_to1),
                    "переполнение градиента second_layer_biases_parameters",
                )?;
            }
        }
        {
            let second_layer_weights_gradients: &mut NonEmptyVec<[FiniteF32; FIRST_LAYER_SIZE]> = &mut self.second_layer_weights_gradients;
            for first_layer_index in 0..FIRST_LAYER_SIZE {
                for vocabulary_character_index in 0..second_layer_biases_parameters_len {
                    accumulate_gradient_product(
                        &mut second_layer_weights_gradients.0[vocabulary_character_index][first_layer_index],
                        forward_pass_result.first_layer_activations[first_layer_index].0,
                        f32::from(next_character_score_gradients_as_f32_from_minus1_to1[vocabulary_character_index]),
                        "переполнение градиента second_layer_weights_parameters",
                    )?;
                }
            }
        }
        let first_layer_pre_activation_gradients: [FiniteF32; FIRST_LAYER_SIZE] = {
            let mut first_layer_pre_activation_gradients: [FiniteF32; FIRST_LAYER_SIZE] = [zero_gradient; FIRST_LAYER_SIZE];
            for first_layer_index in 0..FIRST_LAYER_SIZE {
                for vocabulary_character_index in 0..second_layer_biases_parameters_len {
                    accumulate_gradient_product(
                        &mut first_layer_pre_activation_gradients[first_layer_index],
                        self.second_layer_weights_parameters[vocabulary_character_index][first_layer_index].0,
                        f32::from(next_character_score_gradients_as_f32_from_minus1_to1[vocabulary_character_index]),
                        "переполнение градиента входа первого слоя",
                    )?;
                }
                first_layer_pre_activation_gradients[first_layer_index] = Tanh::backward_input_gradient(
                    forward_pass_result.first_layer_activations[first_layer_index],
                    first_layer_pre_activation_gradients[first_layer_index],
                )?;
            }
            first_layer_pre_activation_gradients
        };
        {
            let first_layer_biases_gradients: &mut [FiniteF32; FIRST_LAYER_SIZE] = &mut self.first_layer_biases_gradients;
            for first_layer_index in 0..FIRST_LAYER_SIZE {
                accumulate_gradient(
                    &mut first_layer_biases_gradients[first_layer_index],
                    first_layer_pre_activation_gradients[first_layer_index].0,
                    "переполнение градиента first_layer_biases_parameters",
                )?;
            }
        }
        {
            let first_layer_weights_gradients: &mut [[FiniteF32; FIRST_LAYER_SIZE]; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION] =
                &mut self.first_layer_weights_gradients;
            for character_embeddings_flattened_index in 0..forward_pass_result.character_embeddings_flattened.len() {
                for first_layer_index in 0..FIRST_LAYER_SIZE {
                    accumulate_gradient_product(
                        &mut first_layer_weights_gradients[character_embeddings_flattened_index][first_layer_index],
                        forward_pass_result.character_embeddings_flattened[character_embeddings_flattened_index].0,
                        first_layer_pre_activation_gradients[first_layer_index].0,
                        "переполнение градиента first_layer_weights_parameters",
                    )?;
                }
            }
        }
        {
            let character_embeddings_gradients: &mut NonEmptyVec<[FiniteF32; EMBEDDING_DIMENSIONS]> = &mut self.character_embeddings_gradients;
            for character_embeddings_flattened_index in 0..forward_pass_result.character_embeddings_flattened.len() {
                let input_coordinate_gradient: FiniteF32 = {
                    let mut input_coordinate_gradient: FiniteF32 = zero_gradient;
                    for first_layer_index in 0..FIRST_LAYER_SIZE {
                        accumulate_gradient_product(
                            &mut input_coordinate_gradient,
                            self.first_layer_weights_parameters[character_embeddings_flattened_index][first_layer_index].0,
                            first_layer_pre_activation_gradients[first_layer_index].0,
                            "переполнение градиента координаты эмбеддинга",
                        )?;
                    }
                    input_coordinate_gradient
                };
                let embedding_gradient: &mut FiniteF32 = &mut character_embeddings_gradients.0
                    [forward_pass_result.previous_vocabulary_character_usize_indexes[character_embeddings_flattened_index / EMBEDDING_DIMENSIONS].0]
                    [character_embeddings_flattened_index % EMBEDDING_DIMENSIONS];
                accumulate_gradient(
                    embedding_gradient,
                    input_coordinate_gradient.0,
                    "переполнение градиента character_embeddings_parameters",
                )?;
            }
        }
        Ok(())
    }
    fn step(&mut self, learning_rate: LearningRateAsPositiveFiniteF32) -> Result<(), &'static str> {
        let update_parameters = |parameters: &mut [FiniteF32], gradients: &[FiniteF32]| -> Result<(), &'static str> {
            for (parameter, gradient) in parameters.iter_mut().zip(gradients) {
                *parameter = FiniteF32::try_from(parameter.0 - learning_rate.0 * gradient.0)?;
            }
            Ok(())
        };
        update_parameters(
            self.character_embeddings_parameters.0.as_flattened_mut(),
            self.character_embeddings_gradients.as_flattened(),
        )?;
        update_parameters(
            self.first_layer_weights_parameters.as_flattened_mut(),
            self.first_layer_weights_gradients.as_flattened(),
        )?;
        update_parameters(&mut self.first_layer_biases_parameters, &self.first_layer_biases_gradients)?;
        update_parameters(
            self.second_layer_weights_parameters.0.as_flattened_mut(),
            self.second_layer_weights_gradients.as_flattened(),
        )?;
        update_parameters(&mut self.second_layer_biases_parameters.0, &self.second_layer_biases_gradients.0)?;
        Ok(())
    }
}
struct ForwardPassResult {
    previous_vocabulary_character_usize_indexes: [VocabularyCharacterIndexAsUsize; PREVIOUS_CHARACTER_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    character_embeddings_flattened: [FiniteF32; TOTAL_EMBEDDING_COORDINATE_COUNT_FOR_NEXT_CHARACTER_PREDICTION],
    first_layer_activations: [FiniteF32; FIRST_LAYER_SIZE],
    character_scores: NonEmptyVec<FiniteF32>,
}
struct Tanh;
impl Tanh {
    fn forward_activation(input: FiniteF32) -> Result<FiniteF32, &'static str> {
        FiniteF32::try_from(input.0.tanh())
    }
    fn backward_input_gradient(output: FiniteF32, output_gradient: FiniteF32) -> Result<FiniteF32, &'static str> {
        FiniteF32::try_from(output_gradient.0 * (1.0 - output.0 * output.0))
    }
}
struct SoftmaxCrossEntropy;
impl SoftmaxCrossEntropy {
    fn forward_probabilities_and_loss(
        character_scores: NonEmptySlice<'_, FiniteF32>,
        next_vocabulary_character_usize_index: VocabularyCharacterIndexAsUsize,
    ) -> Result<NextCharacterProbabilitiesAndNegativeLogLikelihood, &'static str> {
        let (next_character_probabilities, maximum_character_score, exponentials_from_character_scores_minus_maximum_sum): (
            NonEmptyVec<CharacterProbabilityAsF32From0To1>,
            f32,
            f32,
        ) = {
            let maximum_character_score: f32 = character_scores
                .iter()
                .map(|score: &FiniteF32| -> f32 { score.0 })
                .fold(f32::NEG_INFINITY, f32::max);
            let exponentials_from_character_scores_minus_maximum: NonEmptyVec<f32> = NonEmptyVec::try_from(
                character_scores
                    .iter()
                    .map(|character_score: &FiniteF32| -> f32 { (character_score.0 - maximum_character_score).exp() })
                    .collect::<Vec<f32>>(),
            )?;
            let exponentials_from_character_scores_minus_maximum_sum: f32 = exponentials_from_character_scores_minus_maximum.iter().sum();
            let next_character_probabilities: NonEmptyVec<CharacterProbabilityAsF32From0To1> = NonEmptyVec::try_from(
                exponentials_from_character_scores_minus_maximum
                    .into_iter()
                    .map(|value: f32| -> Result<CharacterProbabilityAsF32From0To1, &'static str> {
                        CharacterProbabilityAsF32From0To1::try_from(value / exponentials_from_character_scores_minus_maximum_sum)
                    })
                    .collect::<Result<Vec<CharacterProbabilityAsF32From0To1>, &'static str>>()?,
            )?;
            (
                next_character_probabilities,
                maximum_character_score,
                exponentials_from_character_scores_minus_maximum_sum,
            )
        };
        let negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32 = {
            let target_character_score: f32 = character_scores
                .get(next_vocabulary_character_usize_index.0)
                .ok_or("индекс целевого символа выходит за границы оценок")?
                .0;
            let negative_log_likelihood_loss: f32 =
                (maximum_character_score - target_character_score) + exponentials_from_character_scores_minus_maximum_sum.ln();
            CrossEntropyLossAsNonNegativeF32::try_from(negative_log_likelihood_loss)?
        };
        Ok(NextCharacterProbabilitiesAndNegativeLogLikelihood {
            negative_log_likelihood_loss,
            next_character_probabilities,
        })
    }
    fn backward_score_gradients(
        result: &NextCharacterProbabilitiesAndNegativeLogLikelihood,
        target: VocabularyCharacterIndexAsUsize,
        batch_len: std::num::NonZeroUsize,
    ) -> Result<NonEmptyVec<NextCharacterScoreGradientAsF32FromMinus1To1>, &'static str> {
        let batch_len: f32 = batch_len.get() as f32;
        NonEmptyVec::try_from(
            result
                .next_character_probabilities
                .iter()
                .enumerate()
                .map(
                    |indexed_probability: (usize, &CharacterProbabilityAsF32From0To1)| -> Result<NextCharacterScoreGradientAsF32FromMinus1To1, &'static str> {
                        let (next_character_probability_index, next_character_probability): (usize, &CharacterProbabilityAsF32From0To1) = indexed_probability;
                        let target_character_index: usize = target.0;
                        if next_character_probability_index == target_character_index {
                            let gradient: f32 = (next_character_probability.0 - 1.0) / batch_len;
                            Ok(NextCharacterScoreGradientAsF32FromMinus1To1::Target(
                                TargetScoreGradientAsF32FromMinus1To0::try_from(gradient)?,
                            ))
                        } else {
                            let gradient: f32 = next_character_probability.0 / batch_len;
                            Ok(NextCharacterScoreGradientAsF32FromMinus1To1::Other(
                                CharacterProbabilityAsF32From0To1::try_from(gradient)?,
                            ))
                        }
                    },
                )
                .collect::<Result<Vec<NextCharacterScoreGradientAsF32FromMinus1To1>, &'static str>>()?,
        )
    }
}
struct NextCharacterProbabilitiesAndNegativeLogLikelihood {
    negative_log_likelihood_loss: CrossEntropyLossAsNonNegativeF32,
    next_character_probabilities: NonEmptyVec<CharacterProbabilityAsF32From0To1>,
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
