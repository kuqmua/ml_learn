// Урок 088. Отдельно считать смещение среднего прогноза относительно ответа и разброс прогнозов
// вокруг среднего.
// Это помогает различать общий промах моделей и их несогласие между собой.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

fn main() {
    let predictions: [f64; 3] = [2.0, 4.0, 6.0];
    assert!(
        !predictions.is_empty(),
        "для оценки разброса нужен хотя бы один прогноз"
    );
    let mean: f64 = predictions.iter().sum::<f64>() / predictions.len() as f64;
    let target: f64 = 5.0;
    let average_prediction_minus_target: f64 = mean - target;
    let prediction_variance: f64 = predictions
        .iter()
        .map(|&input_value| (input_value - mean) * (input_value - mean))
        .sum::<f64>()
        / predictions.len() as f64;

    // Выполняем вычисления из примера.
    let _ = (&average_prediction_minus_target, &prediction_variance);

    println!(
        "Прогнозы={predictions:?}; ответ={target}; смещение среднего={average_prediction_minus_target}; разброс={prediction_variance}"
    );
    let total_error = predictions
        .map(|p| (p - target).powi(2))
        .iter()
        .sum::<f64>()
        / predictions.len() as f64;
    assert!(check_f64_eq_1e_minus_12(
        total_error - average_prediction_minus_target.powi(2),
        prediction_variance
    ));
    println!("Средний квадрат ошибки={total_error} = квадрат смещения + разброс.");
}

// Чему учит этот урок:
// Учимся отдельно считать смещение среднего прогноза относительно ответа и разброс прогнозов
// вокруг среднего.
// Это помогает различать общий промах моделей и их несогласие между собой.
