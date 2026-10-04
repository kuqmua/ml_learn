// Урок 106. Оценивать совместное изменение двух признаков через произведения отклонений от их
// средних.
// Полученная ковариация показывает согласованность изменений и зависит от масштаба обоих
// признаков.

fn main() {
    // Оба ряда имеют три значения: число пар задано типами массивов.
    let feature1_values: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(
        feature1_values.len() >= 2,
        "для выборочной ковариации нужны хотя бы две пары"
    );
    let mean_horizontal_coord: f64 =
        feature1_values.iter().sum::<f64>() / feature1_values.len() as f64;
    let feature2_values: [f64; 3] = [2.0, 4.0, 6.0];
    let mean_vertical_coord: f64 =
        feature2_values.iter().sum::<f64>() / feature2_values.len() as f64;
    let mut sum_after_multiplying_paired_diffs_from_mean: f64 = 0.0;
    for index in 0..feature1_values.len() {
        sum_after_multiplying_paired_diffs_from_mean += (feature1_values[index]
            - mean_horizontal_coord)
            * (feature2_values[index] - mean_vertical_coord);
    }
    let _covariance: f64 =
        sum_after_multiplying_paired_diffs_from_mean / (feature1_values.len() - 1) as f64;

    // Выполняем вычисления из примера.
    let _ = (&feature1_values, &feature2_values);

    println!("Оба признака растут вместе: ковариация={_covariance}");
    assert!(_covariance > 0.0);
    let reversed = feature2_values.map(|value| -value);
    let reversed_mean = reversed.iter().sum::<f64>() / reversed.len() as f64;
    let opposite = feature1_values
        .iter()
        .zip(reversed)
        .map(|(x, y)| (x - mean_horizontal_coord) * (y - reversed_mean))
        .sum::<f64>()
        / (reversed.len() - 1) as f64;
    println!("Второй признак меняет знак: ковариация={opposite}");
    assert_eq!(opposite, -_covariance);
}

// Чему учит этот урок:
// Учимся оценивать совместное изменение двух признаков через произведения отклонений от их
// средних.
// Полученная ковариация показывает согласованность изменений и зависит от масштаба обоих
// признаков.
