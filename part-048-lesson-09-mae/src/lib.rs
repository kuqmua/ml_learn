//! Вычисления и примеры урока part-048-lesson-09-mae.

fn validate_prediction_pairs(targets: &[f64], predictions: &[f64]) -> Result<(), &'static str> {
    if targets.len() != predictions.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    if targets.is_empty() {
        return Err("для оценки нужна хотя бы одна пара значений");
    }
    Ok(())
}

/// Средний модуль ошибки с проверкой числа пар.
pub fn mean_absolute_error(targets: &[f64], predictions: &[f64]) -> Result<f64, &'static str> {
    validate_prediction_pairs(targets, predictions)?;
    let mut absolute_sum = 0.0;
    for index in 0..targets.len() {
        let error = predictions[index] - targets[index];
        absolute_sum += if error < 0.0 { -error } else { error };
    }
    Ok(absolute_sum / targets.len() as f64)
}

// Урок 09.2. Средняя абсолютная ошибка.
//
// Ошибки разных знаков не сокращаются; удвоение промаха удваивает вклад в MAE.
// Общая библиотека проверяет, что у каждого прогноза есть правильный ответ.

pub fn run() {
    let targets = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 4] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка выше ответа", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка ниже ответа", &[2.0, 3.0, 6.0], 1.0 / 3.0),
        ("ошибка вдвое больше", &[2.0, 6.0, 6.0], 2.0 / 3.0),
    ];
    for (description, predictions, expected) in cases {
        let mae = crate::mean_absolute_error(&targets, predictions)
            .expect("у каждого прогноза есть правильный ответ");
        assert!((mae - expected).abs() < 1e-10);
        println!("{description}: {predictions:?} → MAE {mae:.3}");
    }
}
