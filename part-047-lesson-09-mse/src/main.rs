// Урок 09.1. Среднеквадратичная ошибка.
//
// При точных прогнозах MSE равна нулю. Ошибка в два раза больше даёт вклад в четыре раза больше.

fn main() {
    let targets = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 3] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    for (description, predictions, expected) in cases {
        assert_eq!(
            targets.len(),
            predictions.len(),
            "число прогнозов должно совпадать с числом ответов"
        );
        assert!(
            !targets.is_empty(),
            "для MSE нужна хотя бы одна пара значений"
        );
        let mut squared_error_sum = 0.0;
        for index in 0..targets.len() {
            let error = predictions[index] - targets[index];
            squared_error_sum += error * error;
        }
        let mse = squared_error_sum / targets.len() as f64;
        assert!((mse - expected).abs() < 1e-10);
        println!("{description}: {predictions:?} → MSE {mse:.3}");
    }
    let too_short = [2.0, 4.0];
    if targets.len() != too_short.len() {
        println!("разная длина ответов и прогнозов: MSE вычислить нельзя");
    }
}
