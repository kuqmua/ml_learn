// Урок 01.6. Практика: произведения координат, длины векторов и расстояния между точками.
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

use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;
use l002_01_calculate_l1_vector_norm_by_summing_absolute_coordinates::calculate_l1_vector_norm_by_summing_absolute_coordinates;
use l003_01_calculate_vector_length_as_square_root_of_sum_of_squared_coordinates::calculate_vector_length_as_square_root_of_sum_of_squared_coordinates;
use l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences;
use l006_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths;

fn main() {
    let first: [f64; 2] = [3.0, 4.0];
    assert_eq!(
        calculate_l1_vector_norm_by_summing_absolute_coordinates(&first),
        7.0
    );
    assert_eq!(
        calculate_vector_length_as_square_root_of_sum_of_squared_coordinates(&first),
        5.0
    );

    let cases: [(&str, &[f64; 2], f64, Option<f64>); 4] = [
        ("тот же вектор", &[3.0, 4.0], 25.0, Some(1.0)),
        ("перпендикулярный", &[-4.0, 3.0], 0.0, Some(0.0)),
        ("противоположный", &[-3.0, -4.0], -25.0, Some(-1.0)),
        ("нулевой без направления", &[0.0, 0.0], 0.0, None),
    ];
    for (_description, other, expected_sum, expected_cosine) in cases {
        let sum: f64 =
            calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&first, other)
                .expect("у этих векторов одинаковое число координат");
        let distance: f64 =
            calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                &first, other,
            )
            .unwrap();
        let cosine: Option<f64> =
            calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(
                &first, other,
            )
            .ok();
        assert_eq!(sum, expected_sum);
        if let (Some(actual), Some(expected)) = (cosine, expected_cosine) {
            assert!((actual - expected).abs() < 1e-10);
        } else {
            assert_eq!(cosine, expected_cosine);
        }
        let reverse_distance: f64 =
            calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                other, &first,
            )
            .unwrap();
        assert!((distance - reverse_distance).abs() < 1e-10);
    }

    let too_short: [f64; 1] = [1.0];
    let _error: &str = calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
        &first, &too_short,
    )
    .expect_err("векторы разной длины нужно отклонить");

    plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum();
}

// Строим график по результатам урока.
fn plot_l1_and_euclidean_lengths_as_absolute_sum_and_square_root_of_squared_sum() {
    let sum_absolute_values_of_vector_coordinates_points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (horizontal_value, horizontal_value.abs() + 4.0)
        })
        .collect();
    let euclidean_norm_points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (
                horizontal_value,
                (horizontal_value * horizontal_value + 16.0).sqrt(),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сравнение норм",
        "первая координата",
        "норма",
        &[
            lesson_visualization::Series {
                name: "L1",

                points: &sum_absolute_values_of_vector_coordinates_points,
            },
            lesson_visualization::Series {
                name: "L2",

                points: &euclidean_norm_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
