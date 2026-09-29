// Урок 40.2. Управляемое забывание памяти: выбор сохраняемой доли прошлого состояния по текущему входу.
// Связь с принятой терминологией: Забывание состояния с коэффициентом, зависящим от входа.
// Зачем здесь эта тема: Постоянная память одинаково забывает всё; полезность прошлого зависит от
//   текущего входа.
// Почему код устроен так: Делаем коэффициент забывания функцией входа и сравниваем состояния после
//   переключения.
// Представь: При важном новом входе модель может сильнее сохранить его и иначе забыть старое
//   состояние.
// Вход управляет коэффициентом забывания; это учебная идея selective SSM, не реализация Mamba.

/// Избирательное забывание: прибавляем вход к сохранённой доле состояния; по флагу сброса оставляем только текущий вход.
fn calculate_memory_states_by_adding_input_to_retained_state_or_resetting_to_input(
    input: &[(f64, bool)],
) -> Vec<f64> {
    let mut state: f64 = 0.0;
    lesson_trace::trace_step!(state);
    input
        .iter()
        .map(|&(value, reset)| {
            // Долю (fraction) предыдущего состояния, сохраняемую на следующем шаге, называют retention.
            // При reset полностью забываем прошлое (0); иначе сохраняем 90% прежнего состояния.
            let previous_state_share_kept: f64 = if reset { 0.0 } else { 0.9 };
            lesson_trace::trace_step!(previous_state_share_kept);
            state = previous_state_share_kept * state + value;
            lesson_trace::trace_step!(state);
            state
        })
        .collect()
}
fn main() {
    lesson_trace::enable();
    let sequence: [(f64, bool); 4] = [(1.0, false), (0.0, false), (2.0, true), (0.0, false)];
    lesson_trace::trace_step!(sequence);
    let states: Vec<f64> =
        calculate_memory_states_by_adding_input_to_retained_state_or_resetting_to_input(&sequence);
    lesson_trace::trace_step!(states);
    assert_eq!(states[0], 1.0);
    assert_eq!(states[2], 2.0); // reset удаляет прошлый контекст.
    println!("селективное состояние: {states:?}");
    lesson_trace::disable();
    plot_stored_state_with_reset_on_third_step(&states);
}

fn plot_stored_state_with_reset_on_third_step(states: &[f64]) {
    let points: Vec<(f64, f64)> = states
        .iter()
        .enumerate()
        .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
        .collect();
    let path: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "selective-state",
        "Сброс состояния на третьем шаге",
        "шаг",
        "h_t",
        &[lesson_visualization::Series {
            name: "состояние",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
