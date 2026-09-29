// Урок 25.3. Сохранение части старой памяти, добавление новой информации и управление выходом.
// Связь с принятой терминологией: Вычисление выхода ячейки LSTM по входу и предыдущему состоянию.
// Зачем здесь эта тема: Обычная RNN может быстро терять раннюю информацию; LSTM разделяет память и
//   управляющие ворота.
// Почему код устроен так: Считаем забывание, запись и чтение отдельно для одного шага и малого
//   состояния.
// Представь: LSTM может оставить старую память через ворота забывания и добавить новую через ворота
//   записи.
// Forget/input/output gates отдельно управляют памятью и наблюдаемым состоянием.

/// Сигмоида: 1 / (1 + e^(−score)); превращает оценку модели в число от 0 до 1.
fn one_divided_by_one_plus_e_to_negative_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
/// Учебная ячейка LSTM: сохраняем долю старой памяти, добавляем долю кандидата и ограничиваем выход отдельным множителем.
fn mix_old_memory_with_new_candidate_then_gate_memory_output(
    input: f64,
    previous_cell: f64,
    forget_bias: f64,
) -> (f64, f64) {
    let forget: f64 = one_divided_by_one_plus_e_to_negative_score(forget_bias);
    lesson_trace::trace_step!(forget);
    let insert: f64 = one_divided_by_one_plus_e_to_negative_score(input);
    lesson_trace::trace_step!(insert);
    let candidate: f64 = input.tanh();
    lesson_trace::trace_step!(candidate);
    let cell: f64 = forget * previous_cell + insert * candidate;
    lesson_trace::trace_step!(cell);
    let hidden: f64 = one_divided_by_one_plus_e_to_negative_score(input) * cell.tanh();
    lesson_trace::trace_step!(hidden);
    (cell, hidden)
}
fn main() {
    lesson_trace::enable();
    let remembered: f64 =
        mix_old_memory_with_new_candidate_then_gate_memory_output(0.0, 1.0, 5.0).0;
    lesson_trace::trace_step!(remembered);
    let forgotten: f64 =
        mix_old_memory_with_new_candidate_then_gate_memory_output(0.0, 1.0, -5.0).0;
    lesson_trace::trace_step!(forgotten);
    assert!(remembered > forgotten);
    println!("ячейка при открытом forget={remembered:.3}, при закрытом={forgotten:.3}");
}
