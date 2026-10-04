// Урок 25.4. Состояние ячейки GRU: смешивание предыдущего состояния и нового кандидата с управляемыми долями.
// Зачем здесь эта тема: GRU решает похожую задачу памяти меньшим числом ворот.
// Почему код устроен так: Показываем, как update и reset смешивают старое состояние с новым
//   кандидатом.
// Представь: GRU решает, сколько старого состояния сохранить и сколько нового кандидата принять.
// Update gate выбирает между предыдущим состоянием и новым кандидатом.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.

fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(input_value: f64) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
// Оценка модели до преобразования в вероятность — обычное число, которое затем переводят в диапазон от 0 до 1.
/// Учебная ячейка GRU: (1−update)·previous + update·candidate; кандидат учитывает долю предыдущего состояния.
fn calc_gru_state_by_mixing_previous_state_with_candidate_using_update_fraction(
    input: f64,
    previous: f64,
    update_gate_raw_score: f64,
) -> f64 {
    let candidate_share: f64 =
        calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(update_gate_raw_score);
    (1.0 - candidate_share) * previous
        + candidate_share
            * calc_tanh(
                input + calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score(input) * previous,
            )
}
fn main() {
    let previous: f64 = 0.8;

    assert!(
        (calc_gru_state_by_mixing_previous_state_with_candidate_using_update_fraction(
            -0.2, previous, -5.0,
        ) - previous)
            .abs()
            < (calc_gru_state_by_mixing_previous_state_with_candidate_using_update_fraction(
                -0.2, previous, 5.0,
            ) - previous)
                .abs()
    );
}

/// tanh сохраняет знак, равен 0 при нулевом входе и насыщается к −1 или 1.
fn calc_tanh(input: f64) -> f64 {
    input.tanh()
}

// Чему учит этот урок:
// Учимся смешивать прежнее состояние с новым кандидатом по управляемой доле в упрощённой ячейке
// GRU.
// Проверяем, что малая доля обновления оставляет состояние ближе к предыдущему.
