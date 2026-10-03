//! Урок 138. Обновление памяти по взвешенному входу и предыдущему состоянию.

/// Память здесь — одно число: прибавляем взвешенный вход к взвешенной прошлой памяти.
/// Затем ограничиваем результат между −1 и 1 с помощью tanh.
/// Состояния простой рекуррентной сети: h = tanh(input_weight·x + recurrent_weight·h_previous), начиная с h = 0.
/// Возвращает по одному состоянию на каждый вход; длина последовательности задаётся вызывающим кодом.

pub fn calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
    input_sequence: &[f64],
    input_weight: f64,
    recurrent_weight: f64,
) -> Vec<f64> {
    let mut memory_state_bounded_between_minus_1_and_1: f64 = 0.0;
    input_sequence
        .iter()
        .map(|&input_value| {
            memory_state_bounded_between_minus_1_and_1 = (input_weight * input_value
                + recurrent_weight * memory_state_bounded_between_minus_1_and_1)
                .tanh();
            memory_state_bounded_between_minus_1_and_1
        })
        .collect()
}
#[cfg(test)]
mod tests {
    #[test]
    fn future_does_not_change_past() {
        assert_eq!(super::calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(&[1.0, 2.0], 0.4, 0.6), super::calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
            &[1.0, 2.0, 999.0],
            0.4,
            0.6,
        )[..2]);
    }
}
