// Урок 16.5. Объединение средних по предыдущим строкам категории с деревьями общих пороговых проверок.
// Связь с принятой терминологией: Учебный бустинг категорий с упорядоченной статистикой и симметричным деревом.
// Зачем здесь эта тема: Категориальный бустинг объединяет исправление остатков, безопасные
//   статистики категорий и симметричное дерево.
// Почему код устроен так: Оставляем модель маленькой, чтобы проследить вклад каждого механизма
//   отдельно.
// Представь: Сначала безопасно кодируем категорию, затем маленькое дерево исправляет остатки
//   прогноза.
// Соединяем упорядоченную статистику и симметричное дерево; это учебная схема, не полная реализация CatBoost.

fn main() {
    lesson_trace::enable();
    let categories: [&str; 6] = ["A", "B", "A", "B", "A", "B"];
    lesson_trace::trace_step!(categories);
    let targets: [f64; 6] = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
    lesson_trace::trace_step!(targets);
    // Замену категорий числами, рассчитанными по целям, называют target encoding.
    let category_target_mean_values: Vec<f64> =
        part_088_lesson_16_average_earlier_targets_for_category_without_current_answer::average_previous_targets_per_category_with_prior_weight(
            &categories,
            &targets,
            0.5,
            1.0,
        )
        .unwrap();
    lesson_trace::trace_step!(category_target_mean_values);
    // Порог фиксирован для прозрачности примера; настоящий алгоритм выбирает split по данным.
    let tree: part_087_lesson_16_use_same_threshold_test_at_each_tree_level::ObliviousTree =
        part_087_lesson_16_use_same_threshold_test_at_each_tree_level::ObliviousTree {
            splits: vec![(0, 0.5)],
            leaves: vec![-0.25, 0.25],
        };
    lesson_trace::trace_step!(tree);
    // 0.5 — начальный прогноз для бинарной метки; дерево ниже добавляет поправку к нему.
    let base: f64 = 0.5;
    lesson_trace::trace_step!(base);
    // Коэффициент 0.5 берёт половину поправки дерева, чтобы пример показал постепенное усиление.
    let learning_rate: f64 = 0.5;
    lesson_trace::trace_step!(learning_rate);
    let mut before: f64 = 0.0;
    lesson_trace::trace_step!(before);
    let mut after: f64 = 0.0;
    lesson_trace::trace_step!(after);
    for (index, (&feature, &target)) in category_target_mean_values.iter().zip(&targets).enumerate()
    {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(feature);
        lesson_trace::trace_step!(target);
        let prediction: f64 = base
            + learning_rate
                * tree
                    .choose_leaf_by_shared_threshold_test_at_each_level(&[feature])
                    .unwrap();
        lesson_trace::trace_step!(prediction);
        before += (base - target).powi(2);
        lesson_trace::trace_step!(before);
        after += (prediction - target).powi(2);
        lesson_trace::trace_step!(after);
        println!("строка {index}: target={target}, код={feature:.3}, prediction={prediction:.3}");
    }
    println!("MSE до: {:.3}; после: {:.3}", before / 6.0, after / 6.0);
    lesson_trace::disable();
    plot_average_squared_error_before_and_after_training(before / 6.0, after / 6.0);
}

fn plot_average_squared_error_before_and_after_training(before: f64, after: f64) {
    let path: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "mse",
        "Ошибка учебной схемы",
        "MSE",
        &[("до", before), ("после", after)],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", path.display());
}
