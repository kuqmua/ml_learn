// Урок 25.4. Состояние ячейки GRU: смешивание предыдущего состояния и нового кандидата с управляемыми долями.
// Связь с принятой терминологией: Вычисление выхода ячейки GRU по входу и предыдущему состоянию.
// Зачем здесь эта тема: GRU решает похожую задачу памяти меньшим числом ворот.
// Почему код устроен так: Показываем, как update и reset смешивают старое состояние с новым
//   кандидатом.
// Представь: GRU решает, сколько старого состояния сохранить и сколько нового кандидата принять.
// Update gate выбирает между предыдущим состоянием и новым кандидатом.

/// Сигмоида: 1 / (1 + e^(−score)); число от 0 до 1 — вероятность класса или доля пропускаемого сигнала.
use lesson_trace::{enable, trace_step};

fn calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
    input_value: f64,
) -> f64 {
    1.0 / (1.0 + (-input_value).exp())
}
// Оценку модели до преобразования в вероятность называют logit.
/// Учебная ячейка GRU: (1−update)·previous + update·candidate; кандидат учитывает долю предыдущего состояния.
fn calculate_gru_state_by_mixing_previous_state_with_candidate_using_update_fraction(
    input: f64,
    previous: f64,
    update_gate_raw_score: f64,
) -> f64 {
    let reset: f64 =
        calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(input);
    trace_step!(reset);
    let candidate: f64 = (input + reset * previous).tanh();
    trace_step!(candidate);
    let update: f64 = calculate_zero_to_one_fraction_as_one_divided_by_one_plus_e_to_negative_score(
        update_gate_raw_score,
    );
    trace_step!(update);
    (1.0 - update) * previous + update * candidate
}
fn main() {
    enable();
    let previous: f64 = 0.8;
    trace_step!(previous);
    let keep: f64 =
        calculate_gru_state_by_mixing_previous_state_with_candidate_using_update_fraction(
            -0.2, previous, -5.0,
        );
    trace_step!(keep);
    let replace: f64 =
        calculate_gru_state_by_mixing_previous_state_with_candidate_using_update_fraction(
            -0.2, previous, 5.0,
        );
    trace_step!(replace);
    assert!((keep - previous).abs() < (replace - previous).abs());
    println!("keep={keep:.3}; replace={replace:.3}");
}
