// Урок 016. Если вычисление состоит из нескольких шагов, изменение проходит через каждый шаг.
// Здесь сначала считаем 2*x + 1, затем умножаем полученное число само на себя.
// Первый шаг меняется со скоростью 2, второй — со скоростью 2*(2*x + 1).
// Перемножаем эти скорости, чтобы узнать влияние маленького изменения исходного x.
// При x=3 получаем 2*14 = 28.

fn main() {
    let input_value: f64 = 3.0;
    let inner: f64 = 2.0 * input_value + 1.0;
    let outer_derivative: f64 = 2.0 * inner;
    let inner_derivative: f64 = 2.0;
    let _composed_function_slope_as_output_change_per_original_input_change: f64 =
        outer_derivative * inner_derivative;
}
