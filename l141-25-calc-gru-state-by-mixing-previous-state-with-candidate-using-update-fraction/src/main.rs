// Урок 25.4. Состояние ячейки GRU: смешивание предыдущего состояния и нового кандидата с управляемыми долями.
// Зачем здесь эта тема: GRU решает похожую задачу памяти меньшим числом ворот.
// Почему код устроен так: Показываем, как update и reset смешивают старое состояние с новым
//   кандидатом.
// Представь: GRU решает, сколько старого состояния сохранить и сколько нового кандидата принять.
// Update gate выбирает между предыдущим состоянием и новым кандидатом.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.

fn calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(
    input_value: f64,
) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
// Оценка модели до преобразования в вероятность — обычное число, которое затем переводят в диапазон от 0 до 1.
/// Учебная ячейка GRU: (1−update)·previous + update·candidate; кандидат учитывает долю предыдущего состояния.
fn calc_gru_state_by_mixing_previous_state_with_candidate_using_update_fraction(
    input: f64,
    previous: f64,
    update_gate_raw_score: f64,
) -> f64 {
    let candidate_share_where_0_keeps_previous_state_and_1_replaces_it_with_candidate: f64 = calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(
        update_gate_raw_score,
    );
    (1.0 - candidate_share_where_0_keeps_previous_state_and_1_replaces_it_with_candidate) * previous
        + candidate_share_where_0_keeps_previous_state_and_1_replaces_it_with_candidate
            * calc_tanh_as_signed_signal_where_0_means_no_signal_and_large_inputs_approach_1_or_minus_1(input
                + calc_sigmoid_as_one_divided_by_one_plus_e_to_neg_score_where_0_score_means_half_and_larger_scores_approach_1(
                    input,
                ) * previous)
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
fn calc_tanh_as_signed_signal_where_0_means_no_signal_and_large_inputs_approach_1_or_minus_1(
    input: f64,
) -> f64 {
    input.tanh()
}
