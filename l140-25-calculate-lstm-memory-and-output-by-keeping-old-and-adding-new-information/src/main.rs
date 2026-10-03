// Урок 25.3. Память и выход ячейки LSTM: сохранение старой информации, добавление новой и управление выходом.
// Связь с принятой терминологией: Вычисление выхода ячейки LSTM по входу и предыдущему состоянию.
// Зачем здесь эта тема: Обычная RNN может быстро терять раннюю информацию; LSTM разделяет память и
//   управляющие ворота.
// Почему код устроен так: Считаем забывание, запись и чтение отдельно для одного шага и малого
//   состояния.
// Представь: LSTM может оставить старую память через ворота забывания и добавить новую через ворота
//   записи.
// Forget/input/output gates отдельно управляют памятью и наблюдаемым состоянием.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.

fn calculate_sigmoid_as_one_divided_by_one_plus_e_to_negative_score_where_0_score_means_half_and_larger_scores_approach_1(
    input_value: f64,
) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
/// Учебная ячейка LSTM: сохраняем долю старой памяти, добавляем долю кандидата и ограничиваем выход отдельным множителем.
fn calculate_lstm_memory_and_output_by_mixing_old_memory_with_candidate_then_gating_output(
    input: f64,
    previous_cell: f64,
    forget_constant_input_weight: f64,
) -> (f64, f64) {
    let memory_after_retaining_old_information_and_adding_candidate: f64 = calculate_sigmoid_as_one_divided_by_one_plus_e_to_negative_score_where_0_score_means_half_and_larger_scores_approach_1(
        forget_constant_input_weight,
    ) * previous_cell
        + calculate_sigmoid_as_one_divided_by_one_plus_e_to_negative_score_where_0_score_means_half_and_larger_scores_approach_1(input)
            * calculate_tanh_as_signed_signal_where_0_means_no_signal_and_large_inputs_approach_1_or_minus_1(input);
    let bounded_memory_multiplied_by_output_share: f64 =
        calculate_sigmoid_as_one_divided_by_one_plus_e_to_negative_score_where_0_score_means_half_and_larger_scores_approach_1(input)
            * calculate_tanh_as_signed_signal_where_0_means_no_signal_and_large_inputs_approach_1_or_minus_1(memory_after_retaining_old_information_and_adding_candidate);
    (
        memory_after_retaining_old_information_and_adding_candidate,
        bounded_memory_multiplied_by_output_share,
    )
}
fn main() {
    let remembered: f64 =
        calculate_lstm_memory_and_output_by_mixing_old_memory_with_candidate_then_gating_output(
            0.0, 1.0, 5.0,
        )
        .0;
    let forgotten: f64 =
        calculate_lstm_memory_and_output_by_mixing_old_memory_with_candidate_then_gating_output(
            0.0, 1.0, -5.0,
        )
        .0;
    assert!(remembered > forgotten);
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calculate_tanh_as_signed_signal_where_0_means_no_signal_and_large_inputs_approach_1_or_minus_1(
    input: f64,
) -> f64 {
    input.tanh()
}
