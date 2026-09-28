// Урок 09.2. Средняя абсолютная ошибка.
//
// Ошибки разных знаков не сокращают друг друга. В отличие от MSE, удвоение промаха
// удваивает его вклад в MAE.

fn main() {
    let targets = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 4] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка выше ответа", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка ниже ответа", &[2.0, 3.0, 6.0], 1.0 / 3.0),
        ("ошибка вдвое больше", &[2.0, 6.0, 6.0], 2.0 / 3.0),
    ];
    for (description, predictions, expected) in cases {
        assert_eq!(
            targets.len(),
            predictions.len(),
            "число прогнозов должно совпадать с числом ответов"
        );
        assert!(
            !targets.is_empty(),
            "для MAE нужна хотя бы одна пара значений"
        );
        let mut absolute_error_sum = 0.0;
        for index in 0..targets.len() {
            let error = predictions[index] - targets[index];
            absolute_error_sum += if error < 0.0 { -error } else { error };
        }
        let mae = absolute_error_sum / targets.len() as f64;
        assert!((mae - expected).abs() < 1e-10);
        println!("{description}: {predictions:?} → MAE {mae:.3}");
    }
}
