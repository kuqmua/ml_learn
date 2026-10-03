// Из 100 человек 20 больны. У 8 больных и 32 здоровых тест положительный.
use lesson_float_comparison::check_f64_eq_1e_minus_12;

#[test]
#[ignore = "заполни ответы и запусти тест с --ignored"]
fn compare_event_shares_within_two_different_groups() {
    let probability_of_illness_given_pos: Option<f64> = None;
    let probability_of_pos_given_illness: Option<f64> = None;
    let illness_given_pos: f64 =
        probability_of_illness_given_pos.expect("впиши 8 / число положительных тестов");
    let pos_given_illness: f64 = probability_of_pos_given_illness.expect("впиши 8 / число больных");
    assert!(check_f64_eq_1e_minus_12(illness_given_pos, 0.2));
    assert!(check_f64_eq_1e_minus_12(pos_given_illness, 0.4));
}
