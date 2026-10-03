// Урок 106. Проверяем, отклоняются ли два признака от своих средних в одну сторону.
// Для каждой пары вычитаем из обоих значений их средние и перемножаем разницы.
// Сумму делим на число пар минус один.
// Плюс означает, что значения чаще отклоняются в одну сторону, минус — в разные.
// Ноль не доказывает, что между признаками вообще нет связи: связь может быть сложнее.

fn main() {
    // Оба ряда имеют три значения: число пар задано типами массивов.
    let first_feature_values: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(
        first_feature_values.len() >= 2,
        "для выборочной ковариации нужны хотя бы две пары"
    );
    let mean_horizontal_coord: f64 =
        first_feature_values.iter().sum::<f64>() / first_feature_values.len() as f64;
    let second_feature_values: [f64; 3] = [2.0, 4.0, 6.0];
    let mean_vertical_coord: f64 =
        second_feature_values.iter().sum::<f64>() / second_feature_values.len() as f64;
    let mut sum_after_multiplying_paired_diffs_from_mean: f64 = 0.0;
    for index in 0..first_feature_values.len() {
        sum_after_multiplying_paired_diffs_from_mean += (first_feature_values[index]
            - mean_horizontal_coord)
            * (second_feature_values[index] - mean_vertical_coord);
    }
    let _covariance_where_pos_means_same_direction_neg_means_opposite_and_0_means_no_linear_covariation: f64 = sum_after_multiplying_paired_diffs_from_mean
        / (first_feature_values.len() - 1) as f64;

    plot_paired_feature_values_to_show_joint_variation(first_feature_values, second_feature_values);
}

// Строим график по результатам урока.
fn plot_paired_feature_values_to_show_joint_variation(
    horizontal_value: [f64; 3],
    vertical_value: [f64; 3],
) {
    lesson_visualization::scatter_chart(
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
