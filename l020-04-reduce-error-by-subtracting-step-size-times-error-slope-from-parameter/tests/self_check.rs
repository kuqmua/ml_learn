// Один шаг спуска для f(x)=x²: x_new = x_old − rate·2x_old.
use lesson_float_comparison::check_f64_eq_1e_minus_12;

#[test]
#[ignore = "заполни ответы и запусти тест с --ignored"]
fn subtract_learning_rate_times_slope_from_parameter() {
    let next_x: Option<f64> = None; // x=2, rate=0.1
    let next_x: f64 = next_x.expect("вычисли новый x и впиши Some(...)");
    assert!(check_f64_eq_1e_minus_12(next_x, 1.6));
    assert!(next_x * next_x < 4.0);

    // При rate=1.5 новое x=-4: ошибка возрастает с 4 до 16.
    let loss_after_large_step: Option<f64> = None;
    assert_eq!(loss_after_large_step, Some(16.0));
}
