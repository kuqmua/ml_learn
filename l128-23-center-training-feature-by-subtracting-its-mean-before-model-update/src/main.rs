// Урок 23.4. Центрирование входа для обучения: вычитание среднего обучающего признака.
// Связь с принятой терминологией: Нормализация входных признаков для обучения по градиенту.
// Зачем здесь эта тема: Разные масштабы признаков делают градиентные шаги несоразмерными.
// Почему код устроен так: Вычитаем среднее train из его значений, чтобы отдельно увидеть эффект
//   центрирования.
// Представь: Для train [10, 20, 30] среднее 20; после вычитания получаем [−10, 0, 10].
//
// Что изучаем: Нормализация признаков.
// Зачем это нужно: Приведение признаков к сопоставимому масштабу помогает градиентным шагам работать с
// разными координатами.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let training_data: [f64; 3] = [10.0, 20.0, 30.0];
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(&training_data).unwrap();
    let centered: [f64; 3] = training_data.map(|value| value - mean);

    plot_feature_before_and_after_subtracting_mean(training_data, centered);
}

// Строим график по результатам урока.
fn plot_feature_before_and_after_subtracting_mean(training_data: [f64; 3], centered: [f64; 3]) {
    let original_points: Vec<(f64, f64)> = training_data
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let normalized_points: Vec<(f64, f64)> = centered
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Центрирование признака",
        "номер объекта",
        "значение",
        &[
            lesson_visualization::Series {
                name: "до",

                points: &original_points,
            },
            lesson_visualization::Series {
                name: "после",

                points: &normalized_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
