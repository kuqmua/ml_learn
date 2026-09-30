// Урок 25.1. Обновление памяти по взвешенному входу и предыдущему состоянию.
// Связь с принятой терминологией: Вычисление скрытого состояния скалярной рекуррентной сети.
// Зачем здесь эта тема: Последовательность требует памяти о предыдущих входах; обычный слой
//   обрабатывает элементы независимо.
// Почему код устроен так: Обновляем одно скрытое состояние по очереди, чтобы прошлые элементы
//   влияли на текущий выход.
// Представь: После входов A и B состояние сети отличается от состояния после одного B: прошлое
//   сохраняется.
// Состояние переносит информацию от предыдущих элементов последовательности.

use l138_25_update_memory_from_weighted_input_and_previous_memory::calculate_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state;

fn main() {
    let input: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
    let history: [f64; 4] =
        calculate_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state(
            &input, 0.8, 0.7,
        )
        .try_into()
        .expect("по одному состоянию на каждый входной шаг");

    plot_state_after_each_weighted_input_and_memory_update(&history);
}
fn plot_state_after_each_weighted_input_and_memory_update(states: &[f64; 4]) {
    let points: Vec<(f64, f64)> = states
        .iter()
        .enumerate()
        .map(|(item_index, &hidden_state)| (item_index as f64, hidden_state))
        .collect();
    let _path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "rnn-state",
        "Память RNN",
        "шаг",
        "h_t",
        &[lesson_visualization::Series {
            name: "состояние",
            points: &points,
        }],
    )
    .expect("график");
}
