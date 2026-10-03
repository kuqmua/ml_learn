// Урок 018. Проверяем скорость изменения без заранее выведенной формулы.
// Считаем ответ при x + step и при x - step. Разницу ответов делим на 2*step.
// Так узнаём среднее изменение ответа на единицу входа в маленьком промежутке около x.
// Сравниваем оценку с известной скоростью 2*x для формулы x*x.
// Слишком маленький step может ухудшить ответ из-за округления дробных чисел.

fn main() {
    let input_value: f64 = 3.0;
    let step: f64 = 0.0001;
    let value_after_adding_step: f64 = (input_value + step) * (input_value + step);
    let value_after_subtracting_step: f64 = (input_value - step) * (input_value - step);
    let _estimated_derivative_as_local_output_change_per_input_change: f64 =
        (value_after_adding_step - value_after_subtracting_step) / (2.0 * step);
    let analytical_derivative: f64 = 2.0 * input_value;

    // Выполняем вычисления из примера.
    let _ = (input_value, analytical_derivative);
}
