// Урок 128. Центрировать обучающий признак, вычитая его среднее.
// Это подготовка данных перед обновлением модели; само обновление в этом примере не выполняется.

use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let training_data: [f64; 3] = [10.0, 20.0, 30.0];
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    let mean: f64 = calc_mean_by_summing_values_and_dividing_by_count(&training_data).unwrap();

    // Выполняем вычисления из примера.
    let _ = (training_data, training_data.map(|value| value - mean));

    let centered = training_data.map(|x| x - mean);
    println!("До={training_data:?}, среднее={mean}, после={centered:?}");
    assert_eq!(centered, [-10.0, 0.0, 10.0]);
}

// Чему учит этот урок:
// Учимся центрировать обучающий признак, вычитая его среднее.
// Это подготовка данных перед обновлением модели; само обновление в этом примере не выполняется.
