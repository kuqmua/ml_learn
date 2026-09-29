// Урок 25.2. Передача изменений ошибки назад через предыдущие состояния памяти.
// Связь с принятой терминологией: Обратное распространение градиента через рекуррентные состояния во времени.
// Зачем здесь эта тема: Параметр рекуррентной сети влияет на позднюю ошибку через цепочку
//   состояний.
// Почему код устроен так: Разворачиваем вычисление во времени и передаём градиент назад по каждому
//   переходу.
// Представь: Ранний вход влияет на позднюю ошибку через несколько обновлений состояния.
// Градиент рекуррентного веса учитывает все предыдущие шаги.

fn half_squared_error_of_last_recurrent_state_against_target(
    input: &[f64],
    input_weight: f64,
    recurrent_weight: f64,
    target: f64,
) -> f64 {
    let last: f64 = *part_134_lesson_25_update_memory_from_weighted_input_and_previous_memory::apply_tanh_to_weighted_input_plus_weighted_previous_state(
        input,
        input_weight,
        recurrent_weight,
    )
    .last()
    .unwrap();
    lesson_trace::trace_step!(last);
    0.5 * (last - target).powi(2)
}
fn main() {
    lesson_trace::enable();
    let input: [f64; 3] = [1.0, 0.5, -0.2];
    lesson_trace::trace_step!(input);
    let input_weight: f64 = 0.3;
    lesson_trace::trace_step!(input_weight);
    let recurrent_weight: f64 = 0.4;
    lesson_trace::trace_step!(recurrent_weight);
    let target: f64 = 0.7;
    lesson_trace::trace_step!(target);
    let history: Vec<f64> = part_134_lesson_25_update_memory_from_weighted_input_and_previous_memory::apply_tanh_to_weighted_input_plus_weighted_previous_state(
        &input,
        input_weight,
        recurrent_weight,
    );
    lesson_trace::trace_step!(history);
    // Производную функции по параметру или вектор таких производных называют gradient.
    let mut hidden_state_loss_rate_of_change: f64 = history.last().unwrap() - target;
    lesson_trace::trace_step!(hidden_state_loss_rate_of_change);
    let mut recurrent_weight_loss_rate_of_change: f64 = 0.0;
    lesson_trace::trace_step!(recurrent_weight_loss_rate_of_change);
    for time_index in (0..input.len()).rev() {
        lesson_trace::trace_step!(time_index);
        let hidden_state: f64 = history[time_index];
        lesson_trace::trace_step!(hidden_state);
        let preactivation_loss_rate_of_change: f64 =
            hidden_state_loss_rate_of_change * (1.0 - hidden_state * hidden_state);
        lesson_trace::trace_step!(preactivation_loss_rate_of_change);
        let previous: f64 = if time_index == 0 {
            0.0
        } else {
            history[time_index - 1]
        };
        lesson_trace::trace_step!(previous);
        recurrent_weight_loss_rate_of_change += preactivation_loss_rate_of_change * previous;
        lesson_trace::trace_step!(recurrent_weight_loss_rate_of_change);
        hidden_state_loss_rate_of_change = preactivation_loss_rate_of_change * recurrent_weight;
        lesson_trace::trace_step!(hidden_state_loss_rate_of_change);
    }
    // ε=10⁻⁵ сдвигает рекуррентный вес в обе стороны для численной проверки градиента.
    let epsilon: f64 = 1e-5;
    lesson_trace::trace_step!(epsilon);
    let numerically_estimated_rate_of_change: f64 =
        (half_squared_error_of_last_recurrent_state_against_target(
            &input,
            input_weight,
            recurrent_weight + epsilon,
            target,
        ) - half_squared_error_of_last_recurrent_state_against_target(
            &input,
            input_weight,
            recurrent_weight - epsilon,
            target,
        )) / (2.0 * epsilon);
    lesson_trace::trace_step!(numerically_estimated_rate_of_change);
    assert!(
        (recurrent_weight_loss_rate_of_change - numerically_estimated_rate_of_change).abs() < 1e-8
    );
    println!(
        "BPTT gradient={recurrent_weight_loss_rate_of_change:.6}; численная проверка={numerically_estimated_rate_of_change:.6}"
    );
}
