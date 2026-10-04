// Урок 123. Проверять ручную формулу производной вычислениями в соседних точках.
// Сравнение принимает правильную формулу и обнаруживает ошибку с пропущенным множителем.

use l018_03_estimate_derivative_from_two_nearby_function_values::estimate_derivative_from_two_nearby_function_values;
use lesson_float_comparison::check_f64_eq_1e_minus_8;

fn main() {
    let f = |x: f64| x * x;
    let x = 3.0;
    let step = 0.0001;
    let numerical = estimate_derivative_from_two_nearby_function_values(f, x, step);
    let correct = 2.0 * x;
    let wrong = x;
    println!(
        "Численная производная={numerical}; формула 2*x={correct}; ошибочная формула x={wrong}"
    );
    assert!(check_f64_eq_1e_minus_8(correct, numerical));
    assert!((wrong - numerical).abs() > 1.0);
    println!("Проверка обнаруживает пропущенный множитель 2.");
}

// Чему учит этот урок:
// Учимся проверять ручную формулу производной вычислениями в соседних точках.
// Сравнение принимает правильную формулу и обнаруживает ошибку с пропущенным множителем.
