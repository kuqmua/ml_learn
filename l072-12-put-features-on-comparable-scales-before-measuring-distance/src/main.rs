// Урок 12.2. Приведение признаков к сопоставимым масштабам перед измерением расстояния.
// Связь с принятой терминологией: Масштабирование признаков перед вычислением расстояния до соседей.
// Зачем здесь эта тема: Признак с большим числовым диапазоном может захватить расстояние независимо
//   от полезности.
// Почему код устроен так: Масштабируем координаты по train и сравниваем порядок соседей до и после.
// Представь: Разность возраста на 30 лет численно перекроет разность доли 0,2, если признаки не
//   привести к сравнимому масштабу.
//
// Что изучаем: Масштабирование признаков.
// Зачем это нужно: Признак с большими единицами измерения может захватить расстояние. Масштабируем
// координаты перед сравнением соседей.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

fn main() {
    let first: [f64; 2] = [1.0, 1000.0];
    let second: [f64; 2] = [2.0, 1010.0];
    let scale: [f64; 2] = [1.0, 1000.0];
    assert!(
        scale.iter().all(|&value| value > 0.0),
        "масштабы должны быть положительными"
    );
    let raw_squared: f64 =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &second)
            .unwrap();
    let scaled_first: [f64; 2] = [first[0] / scale[0], first[1] / scale[1]];
    let scaled_second: [f64; 2] = [second[0] / scale[0], second[1] / scale[1]];
    let scaled_squared: f64 =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(
            &scaled_first,
            &scaled_second,
        )
        .unwrap();

    plot_squared_distances_before_and_after_feature_scaling(raw_squared, scaled_squared);
}

// Строим график по результатам урока.
fn plot_squared_distances_before_and_after_feature_scaling(raw_squared: f64, scaled_squared: f64) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Эффект масштабирования",
        "квадрат расстояния",
        &[("до", raw_squared), ("после", scaled_squared)],
    )
    .expect("не удалось сохранить график");
}
