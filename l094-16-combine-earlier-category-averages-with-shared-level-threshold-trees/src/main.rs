// Урок 094. Соединять кодирование категорий по прошлым ответам с поправкой от заданного дерева.
// Считаем ошибку до и после поправки; обучение дерева в этом примере не выполняется.

use l091_16_predict_tree_output_by_choosing_leaf_with_shared_threshold_tests::ObliviousTree;
use l092_16_encode_categories_as_average_previous_targets_with_prior_weight::encode_categories_as_average_previous_targets_with_prior_weight;

fn main() {
    let tree: ObliviousTree = ObliviousTree {
        splits: vec![(0, 0.5)],
        leaves: vec![-0.25, 0.25],
    };
    let base: f64 = 0.5;
    let learning_rate: f64 = 0.5;
    let mut before: f64 = 0.0;
    let mut after: f64 = 0.0;
    let categories: [&str; 6] = ["A", "B", "A", "B", "A", "B"];
    let targets: [f64; 6] = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
    for (_index, (&feature, &target)) in
        encode_categories_as_average_previous_targets_with_prior_weight(
            &categories,
            &targets,
            0.5,
            1.0,
        )
        .unwrap()
        .iter()
        .zip(&targets)
        .enumerate()
    {
        let prediction: f64 = base
            + learning_rate
                * tree
                    .predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[feature])
                    .unwrap();
        before += (base - target).powi(2);
        after += (prediction - target).powi(2);
    }
    let _ = (&(before / 6.0), &(after / 6.0));

    // Выполняем вычисления из примера.
    let _ = (before / 6.0, after / 6.0);

    println!(
        "Средний квадрат ошибки: до={}, после поправки={}",
        before / 6.0,
        after / 6.0
    );
    assert!(after < before);
}

// Чему учит этот урок:
// Учимся соединять кодирование категорий по прошлым ответам с поправкой от заданного дерева.
// Считаем ошибку до и после поправки; обучение дерева в этом примере не выполняется.
