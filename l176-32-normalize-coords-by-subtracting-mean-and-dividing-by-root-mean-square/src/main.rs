// Урок 176. Вычитать среднее координат и делить отклонения на общий масштаб.
// Небольшая добавка под корнем позволяет избежать нулевого знаменателя при одинаковых координатах.

fn main() {
    let text_unit: [f64; 2] = [1.0, 3.0];
    let mean: f64 = (text_unit[0] + text_unit[1]) / 2.0;
    let variance: f64 = ((text_unit[0] - mean) * (text_unit[0] - mean)
        + (text_unit[1] - mean) * (text_unit[1] - mean))
        / 2.0;
    let squared_scale: f64 = variance + 0.00001;
    let mut scale: f64 = squared_scale;
    for _ in 0..80 {
        scale = (scale + squared_scale / scale) / 2.0;
    }
    let coords_in_standard_deviation_units: [f64; 2] =
        [(text_unit[0] - mean) / scale, (text_unit[1] - mean) / scale];

    // Выполняем вычисления из примера.
    let _ = &coords_in_standard_deviation_units;

    println!(
        "До={text_unit:?}; среднее={mean}; масштаб={scale}; после={coords_in_standard_deviation_units:?}"
    );
    assert!(coords_in_standard_deviation_units.iter().sum::<f64>().abs() < 1e-10);
    assert!(
        coords_in_standard_deviation_units[0] < 0.0 && coords_in_standard_deviation_units[1] > 0.0
    );
}

// Чему учит этот урок:
// Учимся вычитать среднее координат и делить отклонения на общий масштаб.
// Небольшая добавка под корнем позволяет избежать нулевого знаменателя при одинаковых координатах.
