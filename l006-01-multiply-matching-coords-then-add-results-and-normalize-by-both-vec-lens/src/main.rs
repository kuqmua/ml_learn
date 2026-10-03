// Урок 006. Сравниваем направления двух стрелок независимо от их длин.
// Сначала умножаем соответствующие числа и складываем результаты, как в уроке 001.
// Затем делим сумму на длину первой стрелки и на длину второй.
// Результат 1 означает одно направление, 0 — угол 90°, −1 — противоположные направления.
// Например, [1, 0] и [10, 0] дают 1: длины разные, но направления совпадают.
// Для нулевой стрелки направления нет, поэтому функция возвращает ошибку.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l006_01_multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens;

fn main() {
    let first_vec: [f64; 2] = [1.0, 0.0];
    let cases: [(&str, &[f64], f64); 5] = [
        ("то же направление", &[2.0, 0.0], 1.0),
        ("угол меньше 90°", &[1.0, 1.0], 0.7071067811865475),
        ("перпендикулярные векторы", &[0.0, 2.0], 0.0),
        ("угол больше 90°", &[-1.0, 1.0], -0.7071067811865475),
        ("противоположные направления", &[-2.0, 0.0], -1.0),
    ];

    for (_description, second_vec, expected) in cases {
        assert!(check_f64_eq_1e_minus_10(
            multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
                &first_vec, second_vec
            )
            .expect("для вычисления cos нужны два ненулевых вектора с одинаковым числом координат"),
            expected
        ));
    }

    for (_description, second_vec) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разное число координат", &[1.0][..]),
    ] {
        let _: &str = multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
            &first_vec, second_vec,
        )
        .expect_err("ожидалась ошибка для нулевого вектора или разного числа координат");
    }

    plot_cos_of_angle_between_vecs();
}

// Строим график по результатам урока.
fn plot_cos_of_angle_between_vecs() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Косинусное сходство двух векторов",
        "угол, градусы",
        "сходство",
        &[lesson_visualization::Series {
            name: "вектор [1, 0]",

            points: &(0..=180)
                .step_by(5)
                .map(|plot_step_index| {
                    let angle: f64 = (plot_step_index as f64).to_radians();
                    let cos_as_horizontal_coord_of_unit_direction: f64 = angle.cos();
                    let sin_as_vertical_coord_of_unit_direction: f64 = angle.sin();
                    (
                        plot_step_index as f64,
                        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
                            &[1.0, 0.0],
                            &[
                                cos_as_horizontal_coord_of_unit_direction,
                                sin_as_vertical_coord_of_unit_direction,
                            ],
                        )
                        .unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
