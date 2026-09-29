// Урок 07.4. Заполнение пропусков признака медианой обучающих данных.
// Почему этот урок сейчас: После разбиения пропуски нужно заполнить; медиана validation или test внесла бы в обучение чужую информацию.
// Почему пример устроен так: Считаем медиану только на train и применяем её к остальным частям без пересчёта.
// Медиану вычисляем только по train, затем применяем к validation.

fn median_of_observed_training_values(values: &[f64]) -> f64 {
    let mut sorted: Vec<f64> = values.to_vec();
    lesson_trace::trace_step!(sorted);
    sorted.sort_by(f64::total_cmp);
    sorted[sorted.len() / 2]
}
fn main() {
    lesson_trace::enable();
    let training_data: [Option<f64>; 4] = [Some(1.0), None, Some(3.0), Some(5.0)];
    lesson_trace::trace_step!(training_data);
    let validation: [Option<f64>; 2] = [None, Some(100.0)];
    lesson_trace::trace_step!(validation);
    let observed: Vec<f64> = training_data.iter().flatten().copied().collect();
    lesson_trace::trace_step!(observed);
    let replacement: f64 = median_of_observed_training_values(&observed);
    lesson_trace::trace_step!(replacement);
    let training_filled: Vec<f64> = training_data
        .iter()
        .map(|input_value| input_value.unwrap_or(replacement))
        .collect();
    lesson_trace::trace_step!(training_filled);
    let validation_filled: Vec<f64> = validation
        .iter()
        .map(|input_value| input_value.unwrap_or(replacement))
        .collect();
    lesson_trace::trace_step!(validation_filled);
    assert_eq!(replacement, 3.0);
    assert_eq!(validation_filled, [3.0, 100.0]);
    println!(
        "train median={replacement}; train={training_filled:?}; validation={validation_filled:?}"
    );
}
