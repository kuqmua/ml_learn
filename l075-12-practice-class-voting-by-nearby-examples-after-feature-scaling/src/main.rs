// Урок 075. Искать ближайшие примеры, сортировать расстояния и голосовать выбранным числом соседей.
// На дополнительном наборе проверяем, что приведение признаков к сопоставимому масштабу может
// изменить выбранный класс.

fn main() {
    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calc_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        value * value
    }

    let training_examples: [([f64; 2], bool); 4] = [
        ([0.0, 0.0], false),
        ([0.0, 1.0], false),
        ([2.0, 2.0], true),
        ([2.0, 3.0], true),
    ];
    for neighbor_count in [1, 3] {
        let _ = &((|| -> bool {
            let training_examples: &[([f64; 2], bool)] = &training_examples;

            let neighbor_count: usize = neighbor_count;

            assert!(neighbor_count > 0 && neighbor_count <= training_examples.len());

            let query_point: [f64; 2] = [1.8, 2.1];
            let mut nearest_neighbor_vote_uses_selected_count: Vec<(f64, bool)> = training_examples
                .iter()
                .map(|&(features, target)| {
                    (
                        (calc_square_by_multiplying_number_by_itself(features[0] - query_point[0])
                            + calc_square_by_multiplying_number_by_itself(
                                features[1] - query_point[1],
                            )),
                        target,
                    )
                })
                .collect();

            nearest_neighbor_vote_uses_selected_count.sort_by(|left_neighbor, right_neighbor| {
                left_neighbor.0.total_cmp(&right_neighbor.0)
            });

            nearest_neighbor_vote_uses_selected_count
                .iter()
                .take(neighbor_count)
                .filter(|(_, target)| *target)
                .count()
                * 2
                > neighbor_count
        })());
    }

    // Выполняем вычисления из примера.
    let _ = training_examples;

    // Признак 2 измерен в тысячных долях других единиц: без масштаба он подавляет признак 1.
    let examples = [([0.0_f64, 900.0], false), ([2.0, 1000.0], true)];
    let query = [0.0_f64, 1000.0];
    for scales in [[1.0_f64, 1.0], [1.0, 1000.0]] {
        let mut distances: Vec<_> = examples
            .iter()
            .map(|(p, y)| {
                (
                    ((p[0] - query[0]) / scales[0]).powi(2)
                        + ((p[1] - query[1]) / scales[1]).powi(2),
                    *y,
                )
            })
            .collect();
        distances.sort_by(|a, b| a.0.total_cmp(&b.0));
        let prediction = distances[0].1;
        println!("Масштабы={scales:?}, соседи={distances:?}, класс ближайшего={prediction}");
        assert_eq!(prediction, scales[1] == 1.0);
    }
}

// Чему учит этот урок:
// Учимся искать ближайшие примеры, сортировать расстояния и голосовать выбранным числом соседей.
// На дополнительном наборе проверяем, что приведение признаков к сопоставимому масштабу может
// изменить выбранный класс.
