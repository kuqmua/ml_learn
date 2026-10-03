// Урок 25.1. Обновление памяти по взвешенному входу и предыдущему состоянию.
// Зачем здесь эта тема: Последовательность требует памяти о предыдущих входах; обычный слой
//   обрабатывает элементы независимо.
// Почему код устроен так: Обновляем одно скрытое состояние по очереди, чтобы прошлые элементы
//   влияли на текущий выход.
// Представь: После входов A и B состояние сети отличается от состояния после одного B: прошлое
//   сохраняется.
// Состояние переносит информацию от предыдущих элементов последовательности.

use l138_25_calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state_to_bound_each_state_between_minus_1_and_1::calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state_to_bound_each_state_between_minus_1_and_1;

fn main() {
    let input: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
    plot_state_after_each_weighted_input_and_memory_update(
        &std::convert::TryInto::<[f64; 4]>::try_into(
            calc_memory_states_by_applying_tanh_to_weighted_input_plus_weighted_previous_state_to_bound_each_state_between_minus_1_and_1(
                &input, 0.8, 0.7,
            ),
        )
        .expect("ожидалось по одному состоянию на каждый входной шаг"),
    );
}
fn plot_state_after_each_weighted_input_and_memory_update(states: &[f64; 4]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "rnn-state",
        "Память RNN",
        "шаг",
        "h_t",
        &[lesson_visualization::Series {
            name: "состояние",
            points: &states
                .iter()
                .enumerate()
                .map(
                    |(item_index, &memory_state_bounded_between_minus_1_and_1)| {
                        (
                            item_index as f64,
                            memory_state_bounded_between_minus_1_and_1,
                        )
                    },
                )
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
