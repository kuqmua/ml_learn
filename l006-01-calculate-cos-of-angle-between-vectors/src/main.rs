// Урок 01.5. cos угла между векторами: умножение соответствующих координат, сложение и деление на длины.
// Связь с принятой терминологией: Косинусное сходство двух векторов.
// Зачем здесь эта тема: Для сравнения направления одной длины недостаточно; нормированное скалярное
//   произведение убирает влияние масштаба.
// Почему код устроен так: Делим произведение на обе длины и отдельно рассматриваем нулевой вектор,
//   для которого деление невозможно.
// Представь: Векторы [1, 0] и [10, 0] имеют разную длину, но одинаковое направление; косинус у них
//   равен 1.
//
// Что изучаем: сравнение направлений независимо от длины векторов.
// 1 означает одинаковое направление, 0 — перпендикулярность, −1 — противоположное.
// Положительные значения означают угол меньше 90° (включая 0°), отрицательные — больше 90° (включая 180°).
// Для нулевого вектора направления нет.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l006_01_calculate_cos_of_angle_between_vectors::calc_cos_of_angle_between_vecs_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite;

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
        assert!(
            check_f64_eq_1e_minus_10(calc_cos_of_angle_between_vecs_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&first_vec, second_vec).expect(
                "для вычисления cos нужны два ненулевых вектора с одинаковым числом координат"
            ), expected)
        );
    }

    for (_description, second_vec) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разное число координат", &[1.0][..]),
    ] {
        let _: &str = calc_cos_of_angle_between_vecs_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&first_vec, second_vec)
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
                        calc_cos_of_angle_between_vecs_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(
                            &[1.0, 0.0],
                            &[cos_as_horizontal_coord_of_unit_direction, sin_as_vertical_coord_of_unit_direction],
                        )
                        .unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
