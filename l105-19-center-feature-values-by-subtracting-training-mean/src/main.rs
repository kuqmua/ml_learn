// Урок 19.1. Центрирование признака: вычитание среднего по обучающим данным.
// Связь с принятой терминологией: Центрирование признаков вычитанием среднего по обучающим данным.
// Зачем здесь эта тема: PCA ищет разброс относительно центра; без вычитания среднего направление
//   может отражать смещение данных.
// Почему код устроен так: Вычисляем среднее на train и вычитаем его из каждой координаты.
// Представь: Если все x увеличить на 100, форма облака не меняется; вычитание среднего убирает это
//   смещение.
//
// Что изучаем: Центрирование признаков.
// Зачем это нужно: Вычитаем среднее из каждой координаты, чтобы PCA искал направления изменчивости
// относительно центра данных.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let values: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(
        !values.is_empty(),
        "для центрирования нужно хотя бы одно значение"
    );
    let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(&values).unwrap();
    let centered: [f64; 3] = values.map(|value| value - mean);

    plot_feature_values_after_subtracting_mean(values, centered);
}

// Строим график по результатам урока.
fn plot_feature_values_after_subtracting_mean(values: [f64; 3], centered: [f64; 3]) {
    let original_points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let centered_points: Vec<(f64, f64)> = centered
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Центрирование признака",
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "исходные",

                points: &original_points,
            },
            lesson_visualization::Series {
                name: "центрированные",

                points: &centered_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
