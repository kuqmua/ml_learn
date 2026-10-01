// Урок 01.5. Сходство направлений векторов: умножение соответствующих координат, сложение и деление на длины.
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

use l006_01_calculate_direction_similarity_by_multiplying_coordinates_then_dividing_sum_by_lengths::calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths;

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
        let similarity: f64 =
            calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths(
                &first_vector, second_vector,
            )
            .expect("оба вектора ненулевые и одинаковой длины");
        assert!((similarity - expected).abs() < 1e-10);
    }

    for (_description, second_vector) in [
        ("нулевой вектор", &[0.0, 0.0][..]),
        ("разная длина", &[1.0][..]),
    ] {
        let _error: &str =
            calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths(
                &first_vector, second_vector,
            )
            .expect_err("этот вход должен быть отклонён");
    }

    plot_direction_similarity_after_multiplying_coordinates_and_dividing_by_lengths_for_changing_angle();
}

// Строим график по результатам урока.
fn plot_direction_similarity_after_multiplying_coordinates_and_dividing_by_lengths_for_changing_angle()
 {
    let cosine_similarity_between_two_vectors_points: Vec<(f64, f64)> = (0..=180)
        .step_by(5)
        .map(|plot_step_index| {
            let angle: f64 = (plot_step_index as f64).to_radians();
            (
                plot_step_index as f64,
                calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths(
                    &[1.0, 0.0],
                    &[angle.cos(), angle.sin()],
                )
                .unwrap(),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Косинусное сходство двух векторов",
        "угол, градусы",
        "сходство",
        &[lesson_visualization::Series {
            name: "вектор [1, 0]",

            points: &cosine_similarity_between_two_vectors_points,
        }],
    )
    .expect("не удалось сохранить график");
}
