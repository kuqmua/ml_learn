// Для f(x)=x² производная показывает наклон 2x в выбранной точке.
#[test]
#[ignore = "заполни ответы и запусти тест с --ignored"]
fn calculate_slopes() {
    let slopes_at_minus_two_zero_and_three: Option<[i32; 3]> = None;
    let slopes =
        slopes_at_minus_two_zero_and_three.expect("впиши три наклона в Some([..., ..., ...])");
    assert_eq!(slopes, [-4, 0, 6]);

    // Почему наклон слева от нуля отрицательный, а справа положительный?
    let function_decreases_to_the_left_of_zero: Option<bool> = None;
    assert_eq!(function_decreases_to_the_left_of_zero, Some(true));
}
