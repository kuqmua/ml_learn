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
fn calculate_memory_states_by_adding_input_to_retained_state_or_resetting_to_input(
    values: &[f64],
    reset: &[bool],
) -> Vec<f64> {
    let mut state: f64 = 0.0;
    lesson_trace::trace_step!(state);
    values
        .iter()
        .zip(reset)
        .map(|(&input_value, &clear)| {
            state = if clear {
                input_value
            } else {
                0.8 * state + input_value
            };
            lesson_trace::trace_step!(state);
            state
        })
        .collect()
}
fn main() {
    lesson_trace::enable();
    let values: [f64; 4] = [1.0, 0.0, 2.0, 0.0];
    lesson_trace::trace_step!(values);
    let reset: [bool; 4] = [false, false, true, false];
    lesson_trace::trace_step!(reset);
    let fixed: Vec<f64> =
        l206_40_calculate_memory_states_by_adding_weighted_input_to_retained_previous_state::calculate_memory_states_by_repeatedly_adding_weighted_input_to_retained_previous_state(
            &values, 0.8, 1.0,
        );
    lesson_trace::trace_step!(fixed);
    let dynamic: Vec<f64> =
        calculate_memory_states_by_adding_input_to_retained_state_or_resetting_to_input(
            &values, &reset,
        );
    lesson_trace::trace_step!(dynamic);
    assert!(fixed[2] > dynamic[2]);
    println!("fixed={fixed:?}; selective={dynamic:?}");
    lesson_trace::disable();
    plot_stored_state_with_constant_retention_and_selective_resets(&fixed, &dynamic);
}
fn plot_stored_state_with_constant_retention_and_selective_resets(fixed: &[f64], dynamic: &[f64]) {
    let first_plot_points: Vec<(f64, f64)> = fixed
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let second_plot_points: Vec<(f64, f64)> = dynamic
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "memory",
        "Постоянное и выборочное забывание",
        "шаг",
        "состояние",
        &[
            lesson_visualization::Series {
                name: "fixed",
                points: &first_plot_points,
            },
            lesson_visualization::Series {
                name: "selective",
                points: &second_plot_points,
            },
        ],
    )
    .expect("график");
    println!("график: {}", path.display());
}
