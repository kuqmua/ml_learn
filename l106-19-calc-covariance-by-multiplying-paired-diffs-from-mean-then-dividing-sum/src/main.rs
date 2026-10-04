// Урок 106. Проверяем, отклоняются ли два признака от своих средних в одну сторону.
// Для каждой пары вычитаем из обоих значений их средние и перемножаем разницы.
// Сумму делим на число пар минус один.
// Плюс означает, что значения чаще отклоняются в одну сторону, минус — в разные.
// Ноль не доказывает, что между признаками вообще нет связи: связь может быть сложнее.

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
    let _ = (feature1_values, feature2_values);
}

// Чему учит этот урок:
// Учимся оценивать совместное изменение двух признаков через произведения отклонений от их
// средних.
// Полученная ковариация показывает согласованность изменений и зависит от масштаба обоих
// признаков.
