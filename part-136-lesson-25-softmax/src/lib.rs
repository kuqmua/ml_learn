//! Вычисления и примеры урока part-136-lesson-25-softmax.

// Урок 25.5. Нормировка softmax.
//
// Большая оценка получает больший вес. Равные оценки дают равные веса.
// Прибавление одной константы к обеим оценкам не меняет веса, а сумма весов равна 1.

pub fn run() {
    let cases = [
        ("равные оценки", [0.0, 0.0]),
        ("вторая оценка выше", [0.0, 1.0]),
        ("первая оценка выше", [1.0, 0.0]),
        ("к обеим прибавили 1", [1.0, 2.0]),
    ];
    let mut reference_weights = [0.0; 2];
    for (description, logits) in cases {
        let mut exponentials = [0.0; 2];
        for index in 0..2 {
            // Считаем exp(x) первыми 30 членами ряда Тейлора для малых учебных оценок.
            let mut term = 1.0;
            let mut sum = 1.0;
            for order in 1..=30 {
                term *= logits[index] / order as f64;
                sum += term;
            }
            exponentials[index] = sum;
        }
        let denominator = exponentials[0] + exponentials[1];
        assert!(
            denominator > 0.0,
            "сумма экспонент должна быть положительной"
        );
        let weights = [exponentials[0] / denominator, exponentials[1] / denominator];
        assert!((weights[0] + weights[1] - 1.0).abs() < 1e-10);
        assert!(weights.iter().all(|&weight| (0.0..=1.0).contains(&weight)));
        match description {
            "равные оценки" => assert!((weights[0] - weights[1]).abs() < 1e-10),
            "вторая оценка выше" => {
                assert!(weights[1] > weights[0]);
                reference_weights = weights;
            }
            "первая оценка выше" => assert!(weights[0] > weights[1]),
            "к обеим прибавили 1" => {
                assert!((weights[0] - reference_weights[0]).abs() < 1e-10);
                assert!((weights[1] - reference_weights[1]).abs() < 1e-10);
            }
            _ => unreachable!(),
        }
        println!("{description}: {logits:?} → {weights:?}");
    }
}
