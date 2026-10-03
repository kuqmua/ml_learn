// Урок 01.6. Практика: умножение координат, длины векторов и расстояния между точками.
// Связь с принятой терминологией: Скалярное произведение, нормы, расстояние и косинус двух векторов.
// Зачем здесь эта тема: Теперь можно различать величину, расстояние и направление одних и тех же
//   векторов.
// Почему код устроен так: Считаем все четыре характеристики на одной паре, чтобы увидеть, на какой
//   вопрос отвечает каждая.
// Представь: Для одной пары векторов ответ «насколько длинные?» отличается от ответа «насколько
//   похожи направления?».
//
// Здесь соединяем вычисления из уроков 01.1–01.5. Их реализации находятся в общей
// библиотеках предыдущих уроков: позже те же функции применяются в матрицах, kNN и поиске.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector;
use l002_01_calculate_sum_of_absolute_vector_coordinates::calculate_sum_of_absolute_vector_coordinates_as_total_axis_aligned_length_where_0_means_zero_vector;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer;
use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther;
use l006_01_calculate_cos_of_angle_between_vectors::calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite;

fn main() {
    let first_vector: [f64; 2] = [3.0, 4.0];
    assert_eq!(
        calculate_sum_of_absolute_vector_coordinates_as_total_axis_aligned_length_where_0_means_zero_vector(&first_vector),
        7.0
    );
    assert_eq!(
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates_where_0_means_zero_vector_and_larger_means_longer(&first_vector),
        5.0
    );

    let cases: [(&str, &[f64; 2], f64, Option<f64>); 4] = [
        ("тот же вектор", &[3.0, 4.0], 25.0, Some(1.0)),
        ("перпендикулярный", &[-4.0, 3.0], 0.0, Some(0.0)),
        ("противоположный", &[-3.0, -4.0], -25.0, Some(-1.0)),
        ("нулевой без направления", &[0.0, 0.0], 0.0, None),
    ];
    for (
        _description,
        second_vector,
        expected_sum,
        expected_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite,
    ) in cases
    {
        let direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite: Option<f64> =
            calculate_cos_of_angle_between_vectors_as_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite(&first_vector, second_vector).ok();
        assert_eq!(
            multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(&first_vector, second_vector)
                .expect("ожидались векторы с одинаковым числом координат"),
            expected_sum
        );
        if let (Some(actual), Some(expected)) = (direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite, expected_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite) {
            assert!(check_f64_eq_1e_minus_10(actual, expected));
        } else {
            assert_eq!(direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite, expected_direction_similarity_where_1_means_same_0_means_perpendicular_and_minus_1_means_opposite);
        }
        assert!(
            check_f64_eq_1e_minus_10(calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
                &first_vector,
                second_vector,
            )
            .unwrap(), calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences_where_0_means_same_point_and_larger_means_farther(
                    second_vector,
                    &first_vector,
                )
                .unwrap())
        );
    }

    let too_short: [f64; 1] = [1.0];
    let _: &str = multiply_matching_coordinates_then_add_results_as_unnormalized_alignment_where_positive_means_acute_negative_means_obtuse_and_0_means_perpendicular_or_zero_vector(&first_vector, &too_short)
        .expect_err("векторы разной длины нужно отклонить");

    plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum();
}

// Строим график по результатам урока.
fn plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сравнение норм",
        "первая координата",
        "норма",
        &[
            lesson_visualization::Series {
                name: "L1",

                points: &(-50..=50)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, horizontal_value.abs() + 4.0)
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "L2",

                points: &(-50..=50)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (
                            horizontal_value,
                            (horizontal_value * horizontal_value + 16.0).sqrt(),
                        )
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
