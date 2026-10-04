// Урок 194. Выравнивать масштаб вектора делением на корень из среднего квадратов координат.
// Проверяем средний квадрат нормализованных координат и почти одинаковый результат для исходного и
// удвоенного входа.

use l194_36_normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights::normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights;

fn main() {
    let input_component: [f64; 2] = [3.0, 4.0];
    let _: [f64; 2] =
        normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
            &input_component,
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();

    let normalized =
        normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
            &input_component,
            &[1.0, 1.0],
            1e-8,
        )
        .unwrap();
    let doubled = normalize_vec_scale_by_dividing_coords_by_root_mean_square_then_applying_weights(
        &[6.0, 8.0],
        &[1.0, 1.0],
        1e-8,
    )
    .unwrap();
    println!("Вход={input_component:?}; после={normalized:?}; удвоенный вход после={doubled:?}");
    for i in 0..2 {
        assert!((normalized[i] - doubled[i]).abs() < 1e-8);
    }
    assert!((normalized.iter().map(|v| v * v).sum::<f64>() / 2.0 - 1.0).abs() < 1e-8);
}

// Чему учит этот урок:
// Учимся выравнивать масштаб вектора делением на корень из среднего квадратов координат.
// Проверяем средний квадрат нормализованных координат и почти одинаковый результат для исходного и
// удвоенного входа.
