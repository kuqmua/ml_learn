// Урок 107. Проецировать точки и сравнивать сохранённый разброс для двух кандидатных направлений.
// Проверяем, какое из этих направлений сохраняет информацию, а какое сводит все проекции к нулю.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    let centered_points: [[f64; 2]; 4] = [[-2.0, 0.0], [-1.0, 0.0], [1.0, 0.0], [2.0, 0.0]];
    let unit_direction_preserving_largest_spread: [f64; 2] = [1.0, 0.0];
    for point in centered_points {
        let _projection_coord: f64 = multiply_matching_coords_then_add_results(
            &point,
            &unit_direction_preserving_largest_spread,
        )
        .unwrap();
    }

    // Выполняем вычисления из примера.
    let _ = centered_points;

    let candidates = [[1.0, 0.0], [0.0, 1.0]];
    let spreads = candidates.map(|direction| {
        centered_points
            .iter()
            .map(|point| {
                multiply_matching_coords_then_add_results(point, &direction)
                    .unwrap()
                    .powi(2)
            })
            .sum::<f64>()
    });
    println!("Разброс проекций на горизонтальную и вертикальную оси={spreads:?}");
    assert!(spreads[0] > spreads[1]);
    println!(
        "Из этих двух направлений горизонтальное сохраняет весь разброс, вертикальное теряет его."
    );
}

// Чему учит этот урок:
// Учимся проецировать точки и сравнивать сохранённый разброс для двух кандидатных направлений.
// Проверяем, какое из этих направлений сохраняет информацию, а какое сводит все проекции к нулю.
