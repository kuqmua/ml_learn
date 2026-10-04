/// Приближённая производная: изменение ответа между x-step и x+step,
/// делённое на расстояние между этими точками.
/// Функция должна быть гладкой около x, а step — положительным и конечным.
/// Это численная оценка: слишком большой шаг даёт погрешность приближения,
/// слишком маленький усиливает влияние округления f64.
pub fn estimate_derivative_from_two_nearby_function_values(
    function: impl Fn(f64) -> f64,
    x: f64,
    step: f64,
) -> f64 {
    assert!(
        step.is_finite() && step > 0.0,
        "шаг должен быть положительным и конечным"
    );
    let right_input = x + step;
    let left_input = x - step;
    let right_output = function(right_input);
    let left_output = function(left_input);
    let output_change = right_output - left_output;
    let input_change = 2.0 * step;
    output_change / input_change
}
