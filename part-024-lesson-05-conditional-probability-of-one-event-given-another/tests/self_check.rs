// Из 100 человек 20 больны. У 8 больных и 32 здоровых тест положительный.
#[test]
#[ignore = "заполни ответы и запусти тест с --ignored"]
fn distinguish_two_conditional_probabilities() {
    let probability_of_illness_given_positive: Option<f64> = None;
    let probability_of_positive_given_illness: Option<f64> = None;
    let illness_given_positive =
        probability_of_illness_given_positive.expect("впиши 8 / число положительных тестов");
    let positive_given_illness =
        probability_of_positive_given_illness.expect("впиши 8 / число больных");
    assert!((illness_given_positive - 0.2).abs() < 1e-12);
    assert!((positive_given_illness - 0.4).abs() < 1e-12);
}
