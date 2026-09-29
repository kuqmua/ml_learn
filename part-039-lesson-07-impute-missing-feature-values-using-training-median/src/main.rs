// Урок 07.4. Заполнение пропусков признака медианой обучающих данных.
// Медиану вычисляем только по train, затем применяем к validation.

fn median_of_observed_training_values(values: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[sorted.len() / 2]
}
fn main() {
    let training_data: [Option<f64>; 4] = [Some(1.0), None, Some(3.0), Some(5.0)];
    let validation: [Option<f64>; 2] = [None, Some(100.0)];
    let observed: Vec<f64> = training_data.iter().flatten().copied().collect();
    let replacement: f64 = median_of_observed_training_values(&observed);
    let training_filled: Vec<f64> = training_data
        .iter()
        .map(|input_value| input_value.unwrap_or(replacement))
        .collect();
    let validation_filled: Vec<f64> = validation
        .iter()
        .map(|input_value| input_value.unwrap_or(replacement))
        .collect();
    assert_eq!(replacement, 3.0);
    assert_eq!(validation_filled, [3.0, 100.0]);
    println!(
        "train median={replacement}; train={training_filled:?}; validation={validation_filled:?}"
    );
}
