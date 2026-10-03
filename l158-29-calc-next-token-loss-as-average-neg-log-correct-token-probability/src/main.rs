// Урок 29.2. Ошибка прогноза следующей части текста: среднее отрицательных логарифмов правильных вероятностей.
// Зачем здесь эта тема: Чтобы обучать вероятности следующего токена, нужна ошибка по правильному
//   продолжению.
// Почему код устроен так: Берём отрицательный логарифм назначенной ему вероятности и усредняем по
//   позициям.
// Представь: Если правильному слову дали вероятность 0,9, ошибка меньше, чем при вероятности 0,1.
//
// Чем выше вероятность правильных токенов, тем меньше ошибка. Для вероятности 1
// вклад равен нулю; вероятность 0 запрещена, потому что её логарифм не определён.

fn main() {
    let cases: [(&str, &[f64]); 3] = [
        ("идеальная уверенность", &[1.0, 1.0]),
        ("правильные токены вероятны", &[0.8, 0.5]),
        ("правильные токены маловероятны", &[0.2, 0.1]),
    ];
    let mut previous_error: f64 = 0.0;
    for (index, (_description, probabilities)) in cases.into_iter().enumerate() {
        assert!(!probabilities.is_empty(), "нужна хотя бы одна вероятность");
        assert!(
            probabilities.iter().all(|&p| p > 0.0 && p <= 1.0),
            "вероятность должна быть больше 0 и не больше 1"
        );
        let mut neg_log_sum: f64 = 0.0;
        for &probability in probabilities {
            let ratio: f64 = (probability - 1.0) / (probability + 1.0);
            let mut term: f64 = ratio;
            let mut logarithm: f64 = 0.0;
            for odd_divisor in (1..=99).step_by(2) {
                logarithm += term / odd_divisor as f64;
                term *= ratio * ratio;
            }
            neg_log_sum -= 2.0 * logarithm;
        }
        let average_neg_log_correct_token_probability_where_closer_to_0_means_better: f64 =
            neg_log_sum / probabilities.len() as f64;
        if index > 0 {
            assert!(
                average_neg_log_correct_token_probability_where_closer_to_0_means_better
                    > previous_error
            );
        }
        previous_error = average_neg_log_correct_token_probability_where_closer_to_0_means_better;
    }
    let invalid: [f64; 2] = [0.0, 0.5];
    if invalid.iter().any(|&probability| probability <= 0.0) {}

    plot_next_token_loss_as_neg_log_of_correct_text_unit_probability();
}

// Строим график по результатам урока.
fn plot_next_token_loss_as_neg_log_of_correct_text_unit_probability() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Cross-entropy правильного токена",
        "вероятность",
        "ошибка",
        &[lesson_visualization::Series {
            name: "-ln(p)",

            points: &(1..=100)
                .map(|plot_step_index| {
                    let probability: f64 = plot_step_index as f64 / 100.0;
                    (probability, -probability.ln())
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
