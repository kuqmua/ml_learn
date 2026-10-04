// Урок 015. Когда ответ зависит от двух чисел, рассматриваем влияние каждого отдельно.
// Для x*x + 3*y*y при изменении x удерживаем y на месте: скорость изменения равна 2*x.
// При изменении y удерживаем x на месте: скорость изменения равна 6*y.
// Так можно узнать, какой вход и насколько влияет на ответ рядом с выбранной точкой.

fn main() {
    let (input_value, input2_value): (f64, f64) = (2.0, -1.0);
    let _partial_derivative_as_output_change_per_input1_change_with_input2_fixed: f64 =
        2.0 * input_value;
    let _partial_derivative_as_output_change_per_input2_change_with_input1_fixed: f64 =
        6.0 * input2_value;
}
