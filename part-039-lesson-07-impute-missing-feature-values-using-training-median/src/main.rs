// Урок 07.4. Заполнение пропусков признака медианой обучающих данных.
// Медиану вычисляем только по train, затем применяем к validation.

fn median_of_observed_training_values(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[sorted.len() / 2]
}
fn main() {
    let training_data = [Some(1.0), None, Some(3.0), Some(5.0)];
    let validation = [None, Some(100.0)];
    let observed: Vec<_> = training_data.iter().flatten().copied().collect();
    let replacement = median_of_observed_training_values(&observed);
    let training_filled: Vec<_> = training_data
        .iter()
        .map(|input_value| input_value.unwrap_or(replacement))
        .collect();
    let validation_filled: Vec<_> = validation
        .iter()
        .map(|input_value| input_value.unwrap_or(replacement))
        .collect();
    assert_eq!(replacement, 3.0);
    assert_eq!(validation_filled, [3.0, 100.0]);
    println!(
        "train median={replacement}; train={training_filled:?}; validation={validation_filled:?}"
    );
}
