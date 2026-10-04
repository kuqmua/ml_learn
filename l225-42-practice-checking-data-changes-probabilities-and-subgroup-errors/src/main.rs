// Урок 225. Проверяем сдвиг данных, качество по подгруппам и вероятностные прогнозы.
// Эти проверки отвечают на разные вопросы; одна не заменяет другую.

fn main() {
    // (группа, правильный класс, вероятность положительного класса)
    let cases = [
        ("A", true, 0.9_f64),
        ("A", false, 0.1),
        ("A", true, 0.8),
        ("A", false, 0.2),
        ("B", true, 0.4),
        ("B", false, 0.7),
        ("B", true, 0.6),
        ("B", false, 0.3),
    ];
    let mut accuracies = Vec::new();
    for group in ["A", "B"] {
        let rows: Vec<_> = cases.iter().filter(|row| row.0 == group).collect();
        let correct = rows.iter().filter(|row| (row.2 >= 0.5) == row.1).count();
        let accuracy = correct as f64 / rows.len() as f64;
        let probability_error = rows
            .iter()
            .map(|row| {
                let expected = if row.1 { 1.0 } else { 0.0 };
                (row.2 - expected).powi(2)
            })
            .sum::<f64>()
            / rows.len() as f64;
        println!(
            "Группа {group}, n={}: точность={accuracy}, ошибка вероятностей={probability_error}",
            rows.len()
        );
        accuracies.push(accuracy);
    }
    assert_eq!(accuracies, vec![1.0, 0.5]);

    // Калибровка: сравниваем среднюю заявленную вероятность с частотой события.
    // Ошибка вероятностей выше не заменяет это сравнение.
    for (lower, upper) in [(0.0, 0.5), (0.5, 1.01)] {
        let rows: Vec<_> = cases
            .iter()
            .filter(|row| row.2 >= lower && row.2 < upper)
            .collect();
        assert!(!rows.is_empty());
        let mean_probability = rows.iter().map(|row| row.2).sum::<f64>() / rows.len() as f64;
        let event_frequency = rows.iter().filter(|row| row.1).count() as f64 / rows.len() as f64;
        println!(
            "Интервал [{lower}, {upper}), n={}: вероятность={mean_probability}, частота события={event_frequency}",
            rows.len()
        );
        assert!((0.0..=1.0).contains(&mean_probability));
        assert!((0.0..=1.0).contains(&event_frequency));
    }
    // Наглядный контрпример: класс угадан у всех, но вероятность 0.6 не равна частоте 1.
    let confident_cases = [(true, 0.6_f64); 4];
    assert!(
        confident_cases
            .iter()
            .all(|&(truth, p)| (p >= 0.5) == truth)
    );
    let mean_probability = confident_cases.iter().map(|row| row.1).sum::<f64>() / 4.0;
    let frequency = 1.0;
    assert!(frequency - mean_probability > 0.3);
    println!(
        "Контрпример: точность=1, заявлено {mean_probability}, наблюдаемая частота={frequency}"
    );

    // Сдвиг входного признака: сравниваем доли в одинаковых интервалах.
    let reference = [0.1_f64, 0.2, 0.3, 0.4, 0.6, 0.7, 0.8, 0.9];
    let shifted = [0.6_f64, 0.65, 0.7, 0.75, 0.8, 0.85, 0.9, 0.95];
    let histogram = |values: &[f64]| {
        let lower_count = values.iter().filter(|&&x| x < 0.5).count();
        [
            lower_count as f64 / values.len() as f64,
            (values.len() - lower_count) as f64 / values.len() as f64,
        ]
    };
    let reference_shares = histogram(&reference);
    for (label, current, expected) in [
        ("без сдвига", &reference, 0.0),
        ("со сдвигом", &shifted, 0.5),
    ] {
        let current_shares = histogram(current);
        let drift = (0..2)
            .map(|i| (reference_shares[i] - current_shares[i]).abs())
            .sum::<f64>()
            / 2.0;
        assert_eq!(drift, expected);
        println!(
            "{label}, n={}: доли={current_shares:?}, сдвиг={drift}",
            current.len()
        );
    }
    println!(
        "Малые группы показывают расчёт; по четырём наблюдениям нельзя уверенно судить о калибровке в целом."
    );
}

// Чему учит этот урок:
// Отдельно сравниваем распределения входов, ошибки по группам и вероятности с частотами.
// Видим, что хорошая точность классов ещё не означает совпадения вероятностей и частот.
// Указываем размеры выборок: эти маленькие примеры не доказывают качество модели на всей популяции.
