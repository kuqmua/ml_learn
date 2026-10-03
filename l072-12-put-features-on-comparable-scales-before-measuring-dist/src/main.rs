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
use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

fn main() {
    let scale: [f64; 2] = [1.0, 1000.0];
    assert!(
        scale.iter().all(|&value| value > 0.0),
        "масштабы должны быть положительными"
    );

    let first_example: [f64; 2] = [1.0, 1000.0];
    let scaled_first_example: [f64; 2] = [first_example[0] / scale[0], first_example[1] / scale[1]];
    let second_example: [f64; 2] = [2.0, 1010.0];
    let scaled_second_example: [f64; 2] =
        [second_example[0] / scale[0], second_example[1] / scale[1]];

    plot_squared_dists_before_and_after_feature_scaling(
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &first_example,
            &second_example,
        )
        .unwrap(),
        calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &scaled_first_example,
            &scaled_second_example,
        )
        .unwrap(),
    );
}

// Строим график по результатам урока.
fn plot_squared_dists_before_and_after_feature_scaling(raw_squared: f64, scaled_squared: f64) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Эффект масштабирования",
        "квадрат расстояния",
        &[("до", raw_squared), ("после", scaled_squared)],
    )
    .expect("не удалось сохранить график");
}
