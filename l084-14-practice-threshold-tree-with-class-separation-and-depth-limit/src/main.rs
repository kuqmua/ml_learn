// Урок 14.5. Практика: дерево пороговых проверок с разделением классов и ограничением глубины.
// Зачем здесь эта тема: После критерия и порога нужен рекурсивный алгоритм с условием остановки.
// Почему код устроен так: Строим маленькое дерево с ограничением глубины и проверяем его прогнозы.
// Представь: В каждом узле выбираем порог, делим строки и прекращаем ветвление при достижении
//   ограничения глубины.
//
// Что повторяем вместе: энтропия, Gini, жадное разбиение, переобучение.
// Зачем это нужно: Дерево последовательно делит пространство признаков на области и хранит решение в
//   листьях.
// Что показывает программа: Создаём одномерную задачу с явной границей классов. Обучаем дерево, выбирая
//   порог по уменьшению неоднородности. Проверяем прогноз на новой точке и проверяем структуру дерева.
// Что проверить при изменении примера: Проверь чистый лист, константный признак и изменение train/test
//   метрик с глубиной.
// Дополнительная практика: Реализуй дерево для числовых признаков: выбор порога, max_depth, min_samples_leaf.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

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
    fn calc_binary_class_mixing_as_twice_pos_share_times_neg_share_where_0_means_one_class_and_half_means_equal_class_shares(
        data: &[(f64, bool)],
    ) -> f64 {
        if data.is_empty() {
            return 0.;
        }
        let pos_class_share: f64 =
            data.iter().filter(|(_, target)| *target).count() as f64 / data.len() as f64;
        2. * pos_class_share * (1. - pos_class_share)
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
            let candidate_threshold: f64 = (pair[0] + pair[1]) / 2.;
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
            let weighted_binary_class_mixing_where_0_means_pure_groups_and_smaller_means_better_split: f64 = (samples_below_threshold.len() as f64
                * calc_binary_class_mixing_as_twice_pos_share_times_neg_share_where_0_means_one_class_and_half_means_equal_class_shares(
                    &samples_below_threshold,
                )
                + samples_at_or_above_threshold.len() as f64
                    * calc_binary_class_mixing_as_twice_pos_share_times_neg_share_where_0_means_one_class_and_half_means_equal_class_shares(
                        &samples_at_or_above_threshold,
                    ))
                / data.len() as f64;
            if best_split.is_none_or(|(previous_score, _)| weighted_binary_class_mixing_where_0_means_pure_groups_and_smaller_means_better_split < previous_score) {
                best_split = Some((weighted_binary_class_mixing_where_0_means_pure_groups_and_smaller_means_better_split, candidate_threshold));
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

    let data: [(f64, bool); 4] = [(1., false), (2., false), (3., true), (4., true)];
    let tree: Tree = build_threshold_tree_by_minimizing_weighted_class_mixing(&data, 2);
    let _ = &(predict_class_by_following_threshold_branches_to_leaf(&tree, 3.5));

    plot_predicted_leaf_class_for_changing_feature(tree);

    fn plot_predicted_leaf_class_for_changing_feature(tree: Tree) {
        lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Решение дерева по признаку",
            "признак",
            "класс 1",
            &[lesson_visualization::Series {
                name: "прогноз",

                points: &(0..=50)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (
                            horizontal_value,
                            f64::from(predict_class_by_following_threshold_branches_to_leaf(
                                &tree,
                                horizontal_value,
                            )),
                        )
                    })
                    .collect::<Vec<_>>(),
            }],
        )
        .expect("не удалось сохранить график");
    }
}
