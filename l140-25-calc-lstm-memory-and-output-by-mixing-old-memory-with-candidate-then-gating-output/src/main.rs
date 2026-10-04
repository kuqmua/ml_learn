// Урок 25.3. Память и выход ячейки LSTM: сохранение старой информации, добавление новой и управление выходом.
// Зачем здесь эта тема: Обычная RNN может быстро терять раннюю информацию; LSTM разделяет память и
//   управляющие ворота.
// Почему код устроен так: Считаем забывание, запись и чтение отдельно для одного шага и малого
//   состояния.
// Представь: LSTM может оставить старую память через ворота забывания и добавить новую через ворота
//   записи.
// Forget/input/output gates отдельно управляют памятью и наблюдаемым состоянием.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.

fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
/// Учебная ячейка LSTM: сохраняем долю старой памяти, добавляем долю кандидата и ограничиваем выход отдельным множителем.
fn calc_lstm_memory_and_output_by_mixing_old_memory_with_candidate_then_gating_output(
    input: f64,
    previous_cell: f64,
    forget_constant_input_weight: f64,
) -> (f64, f64) {
    let memory_after_retaining_old_information_and_adding_candidate: f64 =
        calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(forget_constant_input_weight)
            * previous_cell
            + calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(input) * calc_tanh(input);
    let bounded_memory_multiplied_by_output_share: f64 =
        calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(input)
            * calc_tanh(memory_after_retaining_old_information_and_adding_candidate);
    (
        memory_after_retaining_old_information_and_adding_candidate,
        bounded_memory_multiplied_by_output_share,
    )
}
fn main() {
    let remembered: f64 =
        calc_lstm_memory_and_output_by_mixing_old_memory_with_candidate_then_gating_output(
            0.0, 1.0, 5.0,
        )
        .0;
    let forgotten: f64 =
        calc_lstm_memory_and_output_by_mixing_old_memory_with_candidate_then_gating_output(
            0.0, 1.0, -5.0,
        )
        .0;
    assert!(remembered > forgotten);
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calc_tanh(input: f64) -> f64 {
    input.tanh()
}

// Чему учит этот урок:
// Учимся управлять долями старой памяти, нового содержимого и выдаваемого сигнала в упрощённой
// ячейке LSTM.
// Изменяя управление забыванием, проверяем, сколько прежней памяти остаётся после шага.
