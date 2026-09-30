// Квадрат расстояния: сложение квадратов разностей координат.

use l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences;

use lesson_trace::{enable_tracing, trace_step};

fn main() {
    enable_tracing();
    let first = [1.0, 2.0];
    let second = [4.0, 6.0];
    trace_step!(first);
    trace_step!(second);
    let squared_distance =
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &second)
            .unwrap();
    trace_step!(squared_distance);
    println!(
        "Квадрат расстояния: {squared_distance}; обычное расстояние будет вычислено в следующей части."
    );
    assert_eq!(squared_distance, 25.0);
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&second, &first),
        Ok(25.0)
    );
    assert_eq!(
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &first),
        Ok(0.0)
    );
    println!(
        "Разная размерность: {}",
        calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &[1.0])
            .unwrap_err()
    );
}
