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

fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
    input_value: f64,
) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
/// Учебная ячейка LSTM: сохраняем долю старой памяти, добавляем долю кандидата и ограничиваем выход отдельным множителем.
fn calculate_lstm_memory_and_output_by_mixing_old_memory_with_candidate_then_gating_output(
    input: f64,
    previous_cell: f64,
    forget_bias: f64,
) -> (f64, f64) {
    let forget: f64 =
        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(forget_bias);
    let insert: f64 =
        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(input);
    let candidate: f64 = input.tanh();
    let cell: f64 = forget * previous_cell + insert * candidate;
    let hidden: f64 =
        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(input)
            * cell.tanh();
    (cell, hidden)
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
