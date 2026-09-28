// Урок 27.2. Cross-entropy языковой модели.
//
// Чем выше вероятность правильных токенов, тем меньше ошибка. Для вероятности 1
// вклад равен нулю; вероятность 0 запрещена, потому что её логарифм не определён.

fn main() {
    let cases: [(&str, &[f64]); 3] = [
        ("идеальная уверенность", &[1.0, 1.0]),
        ("правильные токены вероятны", &[0.8, 0.5]),
        ("правильные токены маловероятны", &[0.2, 0.1]),
    ];
    let mut previous_error = 0.0;
    for (index, (description, probabilities)) in cases.into_iter().enumerate() {
        assert!(!probabilities.is_empty(), "нужна хотя бы одна вероятность");
        assert!(
            probabilities.iter().all(|&p| p > 0.0 && p <= 1.0),
            "вероятность должна быть больше 0 и не больше 1"
        );
        let mut negative_log_sum = 0.0;
        for &probability in probabilities {
            // ln(x) ≈ 2·(t+t³/3+t⁵/5+...), где t=(x−1)/(x+1).
            let ratio = (probability - 1.0) / (probability + 1.0);
            let mut term = ratio;
            let mut logarithm = 0.0;
            for odd in (1..=99).step_by(2) {
                logarithm += term / odd as f64;
                term *= ratio * ratio;
            }
            negative_log_sum -= 2.0 * logarithm;
        }
        let error = negative_log_sum / probabilities.len() as f64;
        if index > 0 {
            assert!(error > previous_error);
        }
        previous_error = error;
        println!("{description}: {probabilities:?} → cross-entropy {error:.3}");
    }
    let invalid = [0.0, 0.5];
    if invalid.iter().any(|&p| p <= 0.0) {
        println!("нулевая вероятность правильного токена: конечную ошибку вычислить нельзя");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (1..=100)
        .map(|i| {
            let p = i as f64 / 100.0;
            (p, -p.ln())
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Cross-entropy правильного токена",
        "вероятность",
        "ошибка",
        &[lesson_visualization::Series {
            name: "-ln(p)",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
