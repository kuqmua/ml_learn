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
// Промежуточные значения показывают острый или тупой угол. Для нулевого вектора направления нет.

use l006_01_calculate_cos_of_angle_between_vectors::calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite;

fn main() {
    let first_vector: [f64; 2] = [1.0, 0.0];
    let cases: [(&str, &[f64], f64); 5] = [
        ("то же направление", &[2.0, 0.0], 1.0),
        ("острый угол", &[1.0, 1.0], 0.7071067811865475),
        ("перпендикулярные векторы", &[0.0, 2.0], 0.0),
        ("тупой угол", &[-1.0, 1.0], -0.7071067811865475),
        ("противоположные направления", &[-2.0, 0.0], -1.0),
    ];

    for (_description, second_vector, expected) in cases {
        assert!(
            (calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&first_vector, second_vector).expect(
                "для вычисления cos нужны два ненулевых вектора с одинаковым числом координат"
            ) - expected)
                .abs()
                < 1e-10
        );
    }

    for (_description, second_vector) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разное число координат", &[1.0][..]),
    ] {
        let _: &str = calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&first_vector, second_vector)
            .expect_err("ожидалась ошибка для нулевого вектора или разного числа координат");
    }

    plot_cos_of_angle_between_vectors();
}

// Строим график по результатам урока.
fn plot_cos_of_angle_between_vectors() {
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
                    let cos_as_horizontal_coordinate_of_unit_direction: f64 = angle.cos();
                    let sin_as_vertical_coordinate_of_unit_direction: f64 = angle.sin();
                    (
                        plot_step_index as f64,
                        calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(
                            &[1.0, 0.0],
                            &[cos_as_horizontal_coordinate_of_unit_direction, sin_as_vertical_coordinate_of_unit_direction],
                        )
                        .unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
