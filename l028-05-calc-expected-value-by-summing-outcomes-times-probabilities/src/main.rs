// Урок 028. Получать ожидаемый результат как сумму исходов, умноженных на их вероятности.
// Для равновероятных граней кубика это среднее 3.5, а не обязательный результат одного броска.

fn main() {
    let outcomes: [f64; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    assert!(
        !outcomes.is_empty(),
        "для среднего нужен хотя бы один исход"
    );
    let probability: f64 = 1.0 / outcomes.len() as f64;
    let mut _expected_outcome_as_probability_weighted_average_over_repeated_trials: f64 = 0.0;
    for outcome in outcomes {
        _expected_outcome_as_probability_weighted_average_over_repeated_trials +=
            outcome * probability;
    }

    let expected = outcomes.iter().sum::<f64>() / outcomes.len() as f64;
    println!("Исходы={outcomes:?}; среднее многих бросков={expected}");
    assert_eq!(expected, 3.5);
    assert!(!outcomes.contains(&expected));
    println!("Ожидаемое среднее не обязано быть возможным исходом одного броска.");
}

// Чему учит этот урок:
// Учимся получать ожидаемый результат как сумму исходов, умноженных на их вероятности.
// Для равновероятных граней кубика это среднее 3.5, а не обязательный результат одного броска.
