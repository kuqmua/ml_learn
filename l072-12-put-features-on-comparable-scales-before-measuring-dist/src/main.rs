// Урок 072. Делить признаки на выбранные масштабы перед измерением расстояния.
// Вычисляем расстояния до и после преобразования, чтобы понять влияние разных единиц и диапазонов
// признаков.

use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs::calc_squared_point_dist_by_summing_squared_coord_diffs;

fn main() {
    let scale: [f64; 2] = [1.0, 1000.0];
    assert!(
        scale.iter().all(|&value| value > 0.0),
        "масштабы должны быть положительными"
    );

    let example1: [f64; 2] = [1.0, 1000.0];
    let scaled_example1: [f64; 2] = [example1[0] / scale[0], example1[1] / scale[1]];
    let example2: [f64; 2] = [2.0, 1010.0];
    let scaled_example2: [f64; 2] = [example2[0] / scale[0], example2[1] / scale[1]];

    // Выполняем вычисления из примера.
    let _ = (
        calc_squared_point_dist_by_summing_squared_coord_diffs(&example1, &example2).unwrap(),
        calc_squared_point_dist_by_summing_squared_coord_diffs(&scaled_example1, &scaled_example2)
            .unwrap(),
    );

    let candidates = [[0.0_f64, 900.0], [2.0, 1000.0]];
    let query = [0.0_f64, 1000.0];
    let raw = candidates.map(|p| (p[0] - query[0]).powi(2) + (p[1] - query[1]).powi(2));
    let scaled = candidates
        .map(|p| ((p[0] - query[0]) / scale[0]).powi(2) + ((p[1] - query[1]) / scale[1]).powi(2));
    println!("Квадраты расстояний: до масштаба={raw:?}, после={scaled:?}");
    assert!(raw[0] > raw[1]);
    assert!(scaled[0] < scaled[1]);
    println!("После приведения единиц ближайший кандидат поменялся.");
}

// Чему учит этот урок:
// Учимся делить признаки на выбранные масштабы перед измерением расстояния.
// Вычисляем расстояния до и после преобразования, чтобы понять влияние разных единиц и диапазонов
// признаков.
