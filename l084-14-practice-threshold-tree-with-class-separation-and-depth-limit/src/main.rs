// Урок 084. Строить дерево из пороговых проверок и получать ответ, проходя до конечной ветви.
// Рекурсивно выбираем разделения и ограничиваем глубину, чтобы управлять сложностью дерева.

fn main() {
    #[derive(Debug)]
    enum Tree {
        Leaf(bool),

        Split {
            threshold: f64,

            below_threshold_tree: Box<Tree>,

            at_or_above_threshold_tree: Box<Tree>,
        },
    }

    /// Нечистота Джини для двух классов: 2·p·(1−p), где p — доля положительных меток.
    fn calc_binary_class_mixing_as_twice_pos_share_times_neg_share(data: &[(f64, bool)]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let pos_class_share: f64 =
            data.iter().filter(|(_, target)| *target).count() as f64 / data.len() as f64;
        2.0 * pos_class_share * (1.0 - pos_class_share)
    }
    /// Дерево решений: выбираем порог с наименьшей взвешенной нечистотой Джини и повторяем до ограничения глубины.
    fn build_threshold_tree_by_minimizing_weighted_class_mixing(
        data: &[(f64, bool)],
        remaining_depth: usize,
    ) -> Tree {
        let pos_count: usize = data.iter().filter(|(_, target)| *target).count();
        if remaining_depth == 0 || pos_count == 0 || pos_count == data.len() {
            return Tree::Leaf(pos_count * 2 >= data.len());
        }
        let mut sorted_feature_values: Vec<f64> = data.iter().map(|sample| sample.0).collect();
        sorted_feature_values.sort_by(f64::total_cmp);
        let mut best_split: Option<(f64, f64)> = None;
        for pair in sorted_feature_values.windows(2) {
            if pair[0] == pair[1] {
                continue;
            }
            let candidate_threshold: f64 = (pair[0] + pair[1]) / 2.0;
            let samples_below_threshold: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 < candidate_threshold)
                .collect();
            let samples_at_or_above_threshold: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 >= candidate_threshold)
                .collect();
            let weighted_binary_class_mixing: f64 = (samples_below_threshold.len() as f64
                * calc_binary_class_mixing_as_twice_pos_share_times_neg_share(
                    &samples_below_threshold,
                )
                + samples_at_or_above_threshold.len() as f64
                    * calc_binary_class_mixing_as_twice_pos_share_times_neg_share(
                        &samples_at_or_above_threshold,
                    ))
                / data.len() as f64;
            if best_split
                .is_none_or(|(previous_score, _)| weighted_binary_class_mixing < previous_score)
            {
                best_split = Some((weighted_binary_class_mixing, candidate_threshold));
            }
        }
        if let Some((_, threshold)) = best_split {
            Tree::Split {
                threshold,

                below_threshold_tree: Box::new(
                    build_threshold_tree_by_minimizing_weighted_class_mixing(
                        &data
                            .iter()
                            .copied()
                            .filter(|sample| sample.0 < threshold)
                            .collect::<Vec<_>>(),
                        remaining_depth - 1,
                    ),
                ),

                at_or_above_threshold_tree: Box::new(
                    build_threshold_tree_by_minimizing_weighted_class_mixing(
                        &data
                            .iter()
                            .copied()
                            .filter(|sample| sample.0 >= threshold)
                            .collect::<Vec<_>>(),
                        remaining_depth - 1,
                    ),
                ),
            }
        } else {
            Tree::Leaf(pos_count * 2 >= data.len())
        }
    }

    /// Прогноз дерева: сравниваем признак с порогами, идём по ветвям и возвращаем класс листа.
    fn predict_class_by_following_threshold_branches_to_leaf(
        tree: &Tree,
        feature_value: f64,
    ) -> bool {
        match tree {
            Tree::Leaf(target) => *target,

            Tree::Split {
                threshold,

                below_threshold_tree,

                at_or_above_threshold_tree,
            } => predict_class_by_following_threshold_branches_to_leaf(
                if feature_value < *threshold {
                    below_threshold_tree
                } else {
                    at_or_above_threshold_tree
                },
                feature_value,
            ),
        }
    }

    let data: [(f64, bool); 4] = [(1.0, false), (2.0, false), (3.0, true), (4.0, true)];
    let tree: Tree = build_threshold_tree_by_minimizing_weighted_class_mixing(&data, 2);
    let _ = &(predict_class_by_following_threshold_branches_to_leaf(&tree, 3.5));

    // Выполняем вычисления из примера.
    let _ = tree;

    println!("Построенное дерево: {tree:?}");
    for &(value, target) in &data {
        let prediction = predict_class_by_following_threshold_branches_to_leaf(&tree, value);
        println!("Вход={value}: класс={prediction}, правильный={target}");
        assert_eq!(prediction, target);
    }
    let shallow = build_threshold_tree_by_minimizing_weighted_class_mixing(&data, 0);
    let errors = data
        .iter()
        .filter(|&&(x, y)| predict_class_by_following_threshold_branches_to_leaf(&shallow, x) != y)
        .count();
    assert!(errors > 0);
    println!(
        "Без разрешённых разделений ошибок={errors}; ограничение глубины меняет возможности модели."
    );
}

// Чему учит этот урок:
// Учимся строить дерево из пороговых проверок и получать ответ, проходя до конечной ветви.
// Рекурсивно выбираем разделения и ограничиваем глубину, чтобы управлять сложностью дерева.
