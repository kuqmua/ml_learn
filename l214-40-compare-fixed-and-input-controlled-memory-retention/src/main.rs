// Урок 40.3. Сравнение постоянной и управляемой входом доли сохраняемой памяти.
// Связь с принятой терминологией: Сравнение зависящего от входа и постоянного забывания состояния.
// Зачем здесь эта тема: Чтобы понять пользу избирательности, нужен одинаковый сигнал для двух
//   правил памяти.
// Почему код устроен так: Сравниваем фиксированное и зависящее от входа забывание на одной
//   последовательности.
// Представь: Одинаковая последовательность покажет, где гибкое забывание ведёт себя иначе, чем
//   постоянное.
// Сравниваем фиксированное затухание с входозависимым забыванием.

/// Избирательное забывание: прибавляем вход к сохранённой доле состояния; по флагу сброса оставляем только текущий вход.
use l212_40_calculate_memory_states_by_adding_weighted_input_to_retained_previous_state::calc_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state;

fn calc_memory_states_by_adding_input_to_retained_state_or_resetting_to_input<const N: usize>(
    values: &[f64; N],
    reset: &[bool; N],
) -> [f64; N] {
    let mut state: f64 = 0.0;
    std::array::from_fn(|index| {
        let input_value = values[index];
        state = if reset[index] {
            input_value
        } else {
            0.8 * state + input_value
        };
        state
    })
}
fn main() {
    let values: [f64; 4] = [1.0, 0.0, 2.0, 0.0];
    let fixed: [f64; 4] =
        calc_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state(
            &values, 0.8, 1.0,
        );
    let reset: [bool; 4] = [false, false, true, false];
    let dynamic: [f64; 4] =
        calc_memory_states_by_adding_input_to_retained_state_or_resetting_to_input(&values, &reset);
    assert!(fixed[2] > dynamic[2]);

    plot_stored_state_with_constant_retention_and_selective_resets(&fixed, &dynamic);
}
fn plot_stored_state_with_constant_retention_and_selective_resets(
    fixed: &[f64; 4],
    dynamic: &[f64; 4],
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "memory",
        "Постоянное и выборочное забывание",
        "шаг",
        "состояние",
        &[
            lesson_visualization::Series {
                name: "fixed",
                points: &fixed
                    .iter()
                    .enumerate()
                    .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "selective",
                points: &dynamic
                    .iter()
                    .enumerate()
                    .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
