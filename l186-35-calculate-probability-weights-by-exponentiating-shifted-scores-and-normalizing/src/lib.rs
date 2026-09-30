//! Урок 186. Устойчивые вероятностные веса: сдвиг оценок, экспоненты и нормировка.

/// Устойчивый softmax для конечных логитов.
// Оценка модели до преобразования в вероятность — обычное число, которое затем переводят в диапазон от 0 до 1.
/// Softmax: вычитаем максимальную оценку, вычисляем экспоненты и делим каждую на их сумму.
/// Число весов совпадает с числом входных оценок, которое определяется во время выполнения.

pub fn calculate_probability_weights_by_exponentiating_shifted_scores_then_dividing_by_their_sum(
    raw_model_scores: &[f64],
) -> Vec<f64> {
    let maximum: f64 = raw_model_scores
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    let weights: Vec<f64> = raw_model_scores
        .iter()
        .map(|&value| (value - maximum).exp())
        .collect();
    let total: f64 = weights.iter().sum();
    weights.into_iter().map(|weight| weight / total).collect()
}
