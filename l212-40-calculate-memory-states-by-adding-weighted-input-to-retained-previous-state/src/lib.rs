//! Урок 212. Последовательность состояний памяти: прибавление взвешенного входа к сохранённой доле прошлого состояния.
//! Связь с принятой терминологией: Рекуррентная модель состояния.

/// Линейная рекуррентная модель состояния h_t = a*h_(t-1) + b*x_t.
// Долю (fraction) предыдущего состояния, сохраняемую на следующем шаге, называют retention.
/// Линейная рекуррентная модель: h = доля_памяти·h_previous + вес_входа·x; начальное состояние нулевое.
use lesson_trace::trace_step;

pub fn calculate_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state(
    input: &[f64],
    previous_state_share_kept: f64,
    input_factor: f64,
) -> Vec<f64> {
    let mut state: f64 = 0.0;
    trace_step!(state);
    input
        .iter()
        .map(|&value| {
            state = previous_state_share_kept * state + input_factor * value;
            trace_step!(state);
            state
        })
        .collect()
}
