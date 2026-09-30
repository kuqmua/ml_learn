// Урок 35.2. Устойчивый softmax перед расчётом причинного внимания.

use l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum;

use lesson_trace::{enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Без вычитания максимума экспоненты таких оценок переполнились бы.");
    let scores = [1000.0, 1001.0, 1002.0];
    trace_step!(scores);
    let probabilities =
        calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &scores,
        );
    trace_step!(probabilities);
    println!("Вероятностные веса: {probabilities:?}");
    assert!((probabilities.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    let shifted =
        calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
            &[0.0, 1.0, 2.0],
        );
    assert_eq!(probabilities, shifted);
    println!("Общий сдвиг оценок не меняет веса.");
}
