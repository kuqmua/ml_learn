// Урок 14.5. Практика: дерево пороговых проверок с разделением классов и ограничением глубины.
// Связь с принятой терминологией: Дерево решений, меры нечистоты, разбиения и ограничение глубины.
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
    let data: [(f64, bool); 4] = [(1., false), (2., false), (3., true), (4., true)];

    #[derive(Debug)]
    enum Tree {
        Leaf(bool),

        Split {
            threshold: f64,

            left: Box<Tree>,

            right: Box<Tree>,
        },
    }

    /// Нечистота Джини для двух классов: 2·p·(1−p), где p — доля положительных меток.
    fn calculate_class_mixing_as_twice_positive_share_times_negative_share(
        data: &[(f64, bool)],
    ) -> f64 {
        if data.is_empty() {
            return 0.;
        }
        let positive_class_share: f64 =
            data.iter().filter(|(_, target)| *target).count() as f64 / data.len() as f64;
        2. * positive_class_share * (1. - positive_class_share)
    }
    /// Дерево решений: выбираем порог с наименьшей взвешенной нечистотой Джини и повторяем до ограничения глубины.
    fn build_threshold_tree_by_minimizing_weighted_class_mixing(
        data: &[(f64, bool)],
        remaining_depth: usize,
    ) -> Tree {
        let positive_count: usize = data.iter().filter(|(_, target)| *target).count();
        if remaining_depth == 0 || positive_count == 0 || positive_count == data.len() {
            return Tree::Leaf(positive_count * 2 >= data.len());
        }
        let mut sorted_feature_values: Vec<f64> = data.iter().map(|sample| sample.0).collect();
        sorted_feature_values.sort_by(f64::total_cmp);
        let mut best_split: Option<(f64, f64)> = None;
        for pair in sorted_feature_values.windows(2) {
            if pair[0] == pair[1] {
                continue;
            }
            let candidate_threshold: f64 = (pair[0] + pair[1]) / 2.;
            let left_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 < candidate_threshold)
                .collect();
            let right_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 >= candidate_threshold)
                .collect();
            let score: f64 = (left_samples.len() as f64
                * calculate_class_mixing_as_twice_positive_share_times_negative_share(
                    &left_samples,
                )
                + right_samples.len() as f64
                    * calculate_class_mixing_as_twice_positive_share_times_negative_share(
                        &right_samples,
                    ))
                / data.len() as f64;
            if best_split.is_none_or(|(previous_score, _)| score < previous_score) {
                best_split = Some((score, candidate_threshold));
            }
        }
        if let Some((_, threshold)) = best_split {
            let left_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 < threshold)
                .collect();
            let right_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 >= threshold)
                .collect();
            Tree::Split {
                threshold,

                left: Box::new(build_threshold_tree_by_minimizing_weighted_class_mixing(
                    &left_samples,
                    remaining_depth - 1,
                )),

                right: Box::new(build_threshold_tree_by_minimizing_weighted_class_mixing(
                    &right_samples,
                    remaining_depth - 1,
                )),
            }
        } else {
            Tree::Leaf(positive_count * 2 >= data.len())
        }
    }

    let tree: Tree = build_threshold_tree_by_minimizing_weighted_class_mixing(&data, 2);
    /// Прогноз дерева: сравниваем признак с порогами, идём по ветвям и возвращаем класс листа.
    fn predict_class_by_following_threshold_branches_to_leaf(
        tree: &Tree,
        feature_value: f64,
    ) -> bool {
        match tree {
            Tree::Leaf(target) => *target,

            Tree::Split {
                threshold,

                left,

                right,
            } => predict_class_by_following_threshold_branches_to_leaf(
                if feature_value < *threshold {
                    left
                } else {
                    right
                },
                feature_value,
            ),
        }
    }

    let _ = &(predict_class_by_following_threshold_branches_to_leaf(&tree, 3.5));

    plot_predicted_leaf_class_for_changing_feature(tree);

    fn plot_predicted_leaf_class_for_changing_feature(tree: Tree) {
        let decision_tree_points: Vec<(f64, f64)> = (0..=50)
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
            .collect();
        let _chart: std::path::PathBuf = lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Решение дерева по признаку",
            "признак",
            "класс 1",
            &[lesson_visualization::Series {
                name: "прогноз",

                points: &decision_tree_points,
            }],
        )
        .expect("не удалось сохранить график");
    }
}
