// Урок 001. Умножаем числа на одинаковых местах в двух списках и складываем результаты.
// Например, для [1, 2] и [3, 4] получаем 1×3 + 2×4 = 11.
// Так можно посчитать сумму признаков, каждый из которых умножен на свой вес.
// Если представить списки как стрелки, знак результата говорит об угле между ними:
// плюс — угол меньше 90°, минус — больше 90°, ноль — угол 90° или нулевая стрелка.
// Величина результата зависит и от направлений, и от длин стрелок.
// Списки должны содержать одинаковое количество чисел.

use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

fn main() {
    let first_vec: [f64; 2] = [1.0, 2.0];
    let cases: [(&str, &[f64], f64); 6] = [
        ("то же направление", &[2.0, 4.0], 10.0),
        ("угол меньше 90°", &[2.0, 1.0], 4.0),
        ("перпендикулярные векторы", &[-2.0, 1.0], 0.0),
        ("угол больше 90°", &[-3.0, 1.0], -1.0),
        ("противоположные направления", &[-1.0, -2.0], -5.0),
        ("нулевой вектор без направления", &[0.0, 0.0], 0.0),
    ];

    for (_description, second_vec, expected) in cases {
        assert_eq!(
            multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&first_vec, second_vec)
                .expect("ожидались векторы с одинаковым числом координат"),
            expected
        );
    }

    let too_short: [f64; 1] = [3.0];
    let _: &str = multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&first_vec, &too_short)
        .expect_err("разная длина должна быть отклонена");
    let _ = &(first_vec);
    plot_sum_after_multiplying_coords_for_changing_second_coord(&first_vec);
}

// Визуализация вынесена из основного сценария урока.
fn plot_sum_after_multiplying_coords_for_changing_second_coord(first_vec: &[f64; 2]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Умножение координат и сложение результатов: [1, 2] и [1, x]",
        "вторая координата правого вектора, x",
        "сумма после умножения координат",
        &[lesson_visualization::Series {
            name: "вектор [1, 2]",

            points: &(-40..=40)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;

                    (
                        horizontal_value,
                        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(
                            first_vec,
                            &[1.0, horizontal_value],
                        )
                        .expect("ожидалось по две координаты у каждого вектора"),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
