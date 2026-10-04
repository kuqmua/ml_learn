// Урок 105. Вычитать среднее из каждого значения признака.
// Так переносим центр данных в ноль, сохраняя разницы между значениями.

use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let values: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(
        !values.is_empty(),
        "для центрирования нужно хотя бы одно значение"
    );
    let mean: f64 = calc_mean_by_summing_values_and_dividing_by_count(&values).unwrap();

    // Выполняем вычисления из примера.
    let _ = (values, values.map(|value| value - mean));

    let centered = values.map(|v| v - mean);
    println!("До={values:?}; среднее={mean}; после={centered:?}");
    assert!(centered.iter().sum::<f64>().abs() < 1e-12);
    assert_eq!(values[2] - values[0], centered[2] - centered[0]);
}

// Чему учит этот урок:
// Учимся вычитать среднее из каждого значения признака.
// Так переносим центр данных в ноль, сохраняя разницы между значениями.
