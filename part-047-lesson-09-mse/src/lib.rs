//! Вычисления и примеры урока part-047-lesson-09-mse.

fn validate_prediction_pairs(targets: &[f64], predictions: &[f64]) -> Result<(), &'static str> {
    if targets.len() != predictions.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    if targets.is_empty() {
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    Ok(())
}

/// Средний квадрат ошибки с проверкой числа пар.
pub fn mean_squared_error(targets: &[f64], predictions: &[f64]) -> Result<f64, &'static str> {
    validate_prediction_pairs(targets, predictions)?;
    let mut squared_sum = 0.0;
    for index in 0..targets.len() {
        let error = predictions[index] - targets[index];
        squared_sum += error * error;
    }
    Ok(squared_sum / targets.len() as f64)
}

// Урок 09.1. Среднеквадратичная ошибка.
//
// При точном прогнозе MSE равна нулю. Ошибка вдвое больше даёт вклад вчетверо больше.
// Та же общая функция будет использоваться для оценки моделей в следующих уроках.

pub fn run() {
    let targets = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 3] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    for (description, predictions, expected) in cases {
        let mse = crate::mean_squared_error(&targets, predictions)
            .expect("у каждого прогноза есть правильный ответ");
        assert!((mse - expected).abs() < 1e-10);
        println!("{description}: {predictions:?} → MSE {mse:.3}");
    }
    let error =
        crate::mean_squared_error(&targets, &[2.0, 4.0]).expect_err("длины должны совпадать");
    println!("разная длина: {error}");
}
