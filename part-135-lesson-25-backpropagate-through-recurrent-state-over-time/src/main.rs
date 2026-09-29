// Урок 25.2. Обратное распространение градиента через рекуррентные состояния во времени.
// Градиент рекуррентного веса учитывает все предыдущие шаги.

fn half_squared_error_of_last_recurrent_state_against_target(
    input: &[f64],
    input_weight: f64,
    recurrent_weight: f64,
    target: f64,
) -> f64 {
    let last: f64 = *part_134_lesson_25_calculate_hidden_state_of_scalar_recurrent_neural_network::calculate_recurrent_hidden_states_from_input_sequence(
        input,
        input_weight,
        recurrent_weight,
    )
    .last()
    .unwrap();
    0.5 * (last - target).powi(2)
}
fn main() {
    let input: [f64; 3] = [1.0, 0.5, -0.2];
    let input_weight: f64 = 0.3;
    let recurrent_weight: f64 = 0.4;
    let target: f64 = 0.7;
    let history: Vec<f64> = part_134_lesson_25_calculate_hidden_state_of_scalar_recurrent_neural_network::calculate_recurrent_hidden_states_from_input_sequence(
        &input,
        input_weight,
        recurrent_weight,
    );
    // Производную функции по параметру или вектор таких производных называют gradient.
    let mut hidden_state_loss_rate_of_change: f64 = history.last().unwrap() - target;
    let mut recurrent_weight_loss_rate_of_change: f64 = 0.0;
    for time_index in (0..input.len()).rev() {
        let hidden_state: f64 = history[time_index];
        let preactivation_loss_rate_of_change: f64 =
            hidden_state_loss_rate_of_change * (1.0 - hidden_state * hidden_state);
        let previous: f64 = if time_index == 0 {
            0.0
        } else {
            history[time_index - 1]
        };
        recurrent_weight_loss_rate_of_change += preactivation_loss_rate_of_change * previous;
        hidden_state_loss_rate_of_change = preactivation_loss_rate_of_change * recurrent_weight;
    }
    // ε=10⁻⁵ сдвигает рекуррентный вес в обе стороны для численной проверки градиента.
    let epsilon: f64 = 1e-5;
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
    assert!(
        (recurrent_weight_loss_rate_of_change - numerically_estimated_rate_of_change).abs() < 1e-8
    );
    println!(
        "BPTT gradient={recurrent_weight_loss_rate_of_change:.6}; численная проверка={numerically_estimated_rate_of_change:.6}"
    );
}
