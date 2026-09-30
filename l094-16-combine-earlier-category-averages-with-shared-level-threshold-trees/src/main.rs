// Урок 16.5. Объединение средних по предыдущим строкам категории с деревьями общих пороговых проверок.
// Связь с принятой терминологией: Учебный бустинг категорий с упорядоченной статистикой и симметричным деревом.
// Зачем здесь эта тема: Категориальный бустинг объединяет исправление остатков, безопасные
//   статистики категорий и симметричное дерево.
// Почему код устроен так: Оставляем модель маленькой, чтобы проследить вклад каждого механизма
//   отдельно.
// Представь: Сначала безопасно кодируем категорию, затем маленькое дерево исправляет остатки
//   прогноза.
// Соединяем упорядоченную статистику и симметричное дерево; это учебная схема, не полная реализация CatBoost.

use l091_16_build_symmetric_tree_using_shared_threshold_test_at_each_level::ObliviousTree;
use l092_16_encode_category_by_averaging_earlier_targets_without_current_answer::encode_categories_as_average_previous_targets_with_prior_weight;

fn main() {
    let categories: [&str; 6] = ["A", "B", "A", "B", "A", "B"];
    let targets: [f64; 6] = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
    let category_target_mean_values: [f64; 6] =
        encode_categories_as_average_previous_targets_with_prior_weight(
            &categories,
            &targets,
            0.5,
            1.0,
        )
        .unwrap();
    let tree: ObliviousTree = ObliviousTree {
        splits: vec![(0, 0.5)],
        leaves: vec![-0.25, 0.25],
    };
    let base: f64 = 0.5;
    let learning_rate: f64 = 0.5;
    let mut before: f64 = 0.0;
    let mut after: f64 = 0.0;
    for (_index, (&feature, &target)) in
        category_target_mean_values.iter().zip(&targets).enumerate()
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
    plot_average_squared_error_before_and_after_training(before / 6.0, after / 6.0);
}

fn plot_average_squared_error_before_and_after_training(before: f64, after: f64) {
    let _path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "mse",
        "Ошибка учебной схемы",
        "MSE",
        &[("до", before), ("после", after)],
    )
    .expect("не удалось сохранить график");
}
