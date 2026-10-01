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

fn calculate_memory_states_by_adding_input_to_retained_state_or_resetting_to_input<
    const N: usize,
>(
    input: &[(f64, bool); N],
) -> [f64; N] {
    let mut state: f64 = 0.0;
    std::array::from_fn(|index| {
        let (value, reset) = input[index];
        let previous_state_share_kept: f64 = if reset { 0.0 } else { 0.9 };
        state = previous_state_share_kept * state + value;
        state
    })
}
fn main() {
    let sequence: [(f64, bool); 4] = [(1.0, false), (0.0, false), (2.0, true), (0.0, false)];
    let states: [f64; 4] =
        calculate_memory_states_by_adding_input_to_retained_state_or_resetting_to_input(&sequence);
    assert_eq!(states[0], 1.0);
    assert_eq!(states[2], 2.0);

    plot_stored_state_with_reset_on_third_step(&states);
}

fn plot_stored_state_with_reset_on_third_step(states: &[f64; 4]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "selective-state",
        "Сброс состояния на третьем шаге",
        "шаг",
        "h_t",
        &[lesson_visualization::Series {
            name: "состояние",
            points: &states
                .iter()
                .enumerate()
                .map(|(item_index, &horizontal_value)| (item_index as f64, horizontal_value))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
