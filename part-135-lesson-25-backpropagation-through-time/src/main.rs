// Урок 25.2. Обратное распространение через время.
// Градиент рекуррентного веса учитывает все предыдущие шаги.

fn calculate_sequence_prediction_loss(
    input: &[f64],
    input_weight: f64,
    recurrent_weight: f64,
    target: f64,
) -> f64 {
    let last = *part_134_lesson_25_recurrent_neural_network_state::calculate_recurrent_states(
        input,
        input_weight,
        recurrent_weight,
    )
    .last()
    .unwrap();
    0.5 * (last - target).powi(2)
}
fn main() {
    let input = [1.0, 0.5, -0.2];
    let input_weight = 0.3;
    let recurrent_weight = 0.4;
    let target = 0.7;
    let history = part_134_lesson_25_recurrent_neural_network_state::calculate_recurrent_states(
        &input,
        input_weight,
        recurrent_weight,
    );
    // Производную функции по параметру или вектор таких производных называют gradient.
    let mut hidden_state_loss_rate_of_change = history.last().unwrap() - target;
    let mut recurrent_weight_loss_rate_of_change = 0.0;
    for time_index in (0..input.len()).rev() {
        let hidden_state = history[time_index];
        let preactivation_loss_rate_of_change =
            hidden_state_loss_rate_of_change * (1.0 - hidden_state * hidden_state);
        let previous = if time_index == 0 {
            0.0
        } else {
            history[time_index - 1]
        };
        recurrent_weight_loss_rate_of_change += preactivation_loss_rate_of_change * previous;
        hidden_state_loss_rate_of_change = preactivation_loss_rate_of_change * recurrent_weight;
    }
    let epsilon = 1e-5;
    let numerically_estimated_rate_of_change = (calculate_sequence_prediction_loss(
        &input,
        input_weight,
        recurrent_weight + epsilon,
        target,
    ) - calculate_sequence_prediction_loss(
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
