// Квадрат расстояния: сложение квадратов разностей координат.

use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

fn main() {
    let first = [1.0, 2.0];
    let second = [4.0, 6.0];
    let squared_distance =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &second)
            .unwrap();

    assert_eq!(squared_distance, 25.0);
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&second, &first),
        Ok(25.0)
    );
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &first),
        Ok(0.0)
    );
}
