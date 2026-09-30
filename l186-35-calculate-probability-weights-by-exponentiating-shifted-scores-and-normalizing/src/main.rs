// Урок 35.2. Устойчивый softmax перед расчётом причинного внимания.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Без вычитания максимума экспоненты таких оценок переполнились бы.");
    let scores = [1000.0, 1001.0, 1002.0];
    lesson_trace::trace_step!(scores);
    let probabilities = l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(&scores);
    lesson_trace::trace_step!(probabilities);
    println!("Вероятностные веса: {probabilities:?}");
    assert!((probabilities.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    let shifted = l186_35_calculate_probability_weights_by_exponentiating_shifted_scores_and_normalizing::calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(&[0.0, 1.0, 2.0]);
    assert_eq!(probabilities, shifted);
    println!("Общий сдвиг оценок не меняет веса.");
}
