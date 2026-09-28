// Урок 25.2. Обратное распространение через время.
// Градиент рекуррентного веса учитывает все предыдущие шаги.

use part_134_lesson_25_rnn_state::states;
fn loss(input: &[f64], input_weight: f64, recurrent_weight: f64, target: f64) -> f64 {
    let last = *states(input, input_weight, recurrent_weight)
        .last()
        .unwrap();
    0.5 * (last - target).powi(2)
}
fn main() {
    let input = [1.0, 0.5, -0.2];
    let input_weight = 0.3;
    let recurrent_weight = 0.4;
    let target = 0.7;
    let history = states(&input, input_weight, recurrent_weight);
    let mut hidden_gradient = history.last().unwrap() - target;
    let mut recurrent_weight_gradient = 0.0;
    for time_index in (0..input.len()).rev() {
        let hidden_state = history[time_index];
        let preactivation_gradient = hidden_gradient * (1.0 - hidden_state * hidden_state);
        let previous = if time_index == 0 {
            0.0
        } else {
            history[time_index - 1]
        };
        recurrent_weight_gradient += preactivation_gradient * previous;
        hidden_gradient = preactivation_gradient * recurrent_weight;
    }
    let epsilon = 1e-5;
    let numerical_gradient = (loss(&input, input_weight, recurrent_weight + epsilon, target)
        - loss(&input, input_weight, recurrent_weight - epsilon, target))
        / (2.0 * epsilon);
    assert!((recurrent_weight_gradient - numerical_gradient).abs() < 1e-8);
    println!(
        "BPTT gradient={recurrent_weight_gradient:.6}; численная проверка={numerical_gradient:.6}"
    );
}
