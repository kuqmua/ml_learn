// Урок 054. Определять прямую по двум обучающим точкам и оценивать её на отдельных примерах.
// Разделяем подбор параметров и проверку прогноза на данных, не использованных для подбора.

use l050_09_calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count::calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    let training: [(f64, f64); 2] = [(1.0, 3.0), (2.0, 5.0)];
    assert!(
        training.len() >= 2,
        "для прямой нужны хотя бы две обучающие точки"
    );
    assert_ne!(
        training[0].0, training[1].0,
        "обучающие точки должны иметь разные значения x"
    );
    let test: [(f64, f64); 2] = [(3.0, 7.0), (4.0, 9.0)];
    assert!(
        !test.is_empty(),
        "для MSE нужен хотя бы один тестовый пример"
    );
    let weight: f64 = (training[1].1 - training[0].1) / (training[1].0 - training[0].0);
    let constant_input_weight: f64 = training[0].1 - weight * training[0].0;

    let _: f64 = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &test.map(|(_, target)| target),
        &test.map(|(feature, _)| weight * feature + constant_input_weight),
    )
    .unwrap();

    // Выполняем вычисления из примера.
    let _ = (&training, &test, &weight, &constant_input_weight);

    let predictions = test.map(|(feature, _)| weight * feature + constant_input_weight);
    let error = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &test.map(|(_, target)| target),
        &predictions,
    )
    .unwrap();
    println!(
        "Из обучающих точек нашли y={weight}*x+{constant_input_weight}; прогнозы теста={predictions:?}, ошибка={error}"
    );
    assert_eq!(predictions, [7.0, 9.0]);
    assert_eq!(error, 0.0);
}

// Чему учит этот урок:
// Учимся определять прямую по двум обучающим точкам и оценивать её на отдельных примерах.
// Разделяем подбор параметров и проверку прогноза на данных, не использованных для подбора.
