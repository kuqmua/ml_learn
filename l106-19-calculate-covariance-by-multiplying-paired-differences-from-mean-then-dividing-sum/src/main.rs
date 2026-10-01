// Урок 19.2. Совместное изменение признаков (ковариация): умножение парных отклонений, сложение и деление на число наблюдений минус один.
// Связь с принятой терминологией: Выборочная ковариация двух признаков.
// Зачем здесь эта тема: После центрирования нужна мера совместного изменения двух признаков.
// Почему код устроен так: Усредняем произведения отклонений с выборочным знаменателем n−1.
// Представь: Если два признака растут вместе, произведения их отклонений чаще положительны.
//
// Что изучаем: Ковариация.
// Зачем это нужно: Ковариация показывает, меняются ли два признака вместе. Для выборки делим сумму
// результатов умножения отклонений на n−1.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    // Оба ряда имеют три значения: число пар задано типами массивов.
    let first_feature_values: [f64; 3] = [1.0, 2.0, 3.0];
    let second_feature_values: [f64; 3] = [2.0, 4.0, 6.0];
    assert!(
        first_feature_values.len() >= 2,
        "для выборочной ковариации нужны хотя бы две пары"
    );
    let mean_horizontal_coordinate: f64 =
        first_feature_values.iter().sum::<f64>() / first_feature_values.len() as f64;
    let mean_vertical_coordinate: f64 =
        second_feature_values.iter().sum::<f64>() / second_feature_values.len() as f64;
    let mut sum_after_multiplying_paired_differences_from_mean: f64 = 0.0;
    for index in 0..first_feature_values.len() {
        sum_after_multiplying_paired_differences_from_mean += (first_feature_values[index]
            - mean_horizontal_coordinate)
            * (second_feature_values[index] - mean_vertical_coordinate);
    }
    let _ = &(sum_after_multiplying_paired_differences_from_mean
        / (first_feature_values.len() - 1) as f64);

    plot_paired_feature_values_to_show_joint_variation(first_feature_values, second_feature_values);
}

// Строим график по результатам урока.
fn plot_paired_feature_values_to_show_joint_variation(
    horizontal_value: [f64; 3],
    vertical_value: [f64; 3],
) {
    let _chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ковариация: совместное изменение",
        "x",
        "y",
        &[lesson_visualization::Series {
            name: "наблюдения",

            points: &horizontal_value
                .iter()
                .zip(vertical_value.iter())
                .map(|(&first_feature_value, &second_feature_value)| {
                    (first_feature_value, second_feature_value)
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
