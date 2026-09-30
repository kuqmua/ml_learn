// Урок 25.2. Влияние параметров памяти на ошибку: обратный проход по предыдущим состояниям.
// Связь с принятой терминологией: Обратное распространение градиента через рекуррентные состояния во времени.
// Зачем здесь эта тема: Параметр рекуррентной сети влияет на позднюю ошибку через цепочку
//   состояний.
// Почему код устроен так: Разворачиваем вычисление во времени и передаём градиент назад по каждому
//   переходу.
// Представь: Ранний вход влияет на позднюю ошибку через несколько обновлений состояния.
// Градиент рекуррентного веса учитывает все предыдущие шаги.

use l138_25_update_memory_from_weighted_input_and_previous_memory::calculate_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state;

fn half_squared_error_of_last_recurrent_state_against_target(
    input: &[f64],
    input_weight: f64,
    recurrent_weight: f64,
    target: f64,
) -> f64 {
    let last: f64 =
        *calculate_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
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
    let history: Vec<f64> =
        calculate_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
            &input,
            input_weight,
            recurrent_weight,
        );
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
}
