// Квадрат расстояния: сложение квадратов разностей координат.

use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

fn main() {
    let first_point = [1.0, 2.0];
    let second_point = [4.0, 6.0];
    let squared_distance =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(
            &first_point,
            &second_point,
        )
        .unwrap();

    assert_eq!(squared_distance, 25.0);
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(
            &second_point,
            &first_point
        ),
        Ok(25.0)
    );
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(
            &first_point,
            &first_point
        ),
        Ok(0.0)
    );
}
