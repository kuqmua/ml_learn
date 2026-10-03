//! Урок 212. Последовательность состояний памяти: прибавление взвешенного входа к сохранённой доле прошлого состояния.
//! Связь с принятой терминологией: Рекуррентная модель состояния.

/// Линейная рекуррентная модель состояния h_t = a*h_(t-1) + b*x_t.
// Долю (fraction) предыдущего состояния, сохраняемую на следующем шаге, называют retention.
/// Линейная рекуррентная модель: h = доля_памяти·h_previous + вес_входа·x; начальное состояние нулевое.

pub fn calc_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state<
    const N: usize,
>(
    input_sequence: &[f64; N],
    previous_state_share_kept_where_0_forgets_and_1_retains_all: f64,
    input_weight: f64,
) -> [f64; N] {
    let mut state: f64 = 0.0;
    std::array::from_fn(|index| {
        let value = input_sequence[index];
        state = previous_state_share_kept_where_0_forgets_and_1_retains_all * state
            + input_weight * value;
        state
    })
}
