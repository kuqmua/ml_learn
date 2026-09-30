//! Урок 182. Построение контекста текста по предыдущим и следующим позициям.
//! Связь с принятой терминологией: Двунаправленное внимание encoder.

/// Полное self-attention: каждая позиция видит обе стороны последовательности.
/// Двунаправленное внимание: для каждого состояния считаем произведения координат с видимыми состояниями, делим на sqrt(2), применяем softmax и суммируем состояния с этими весами.
pub fn calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
    states: &[[f64; 2]],
    visible: &[bool],
) -> Result<Vec<[f64; 2]>, &'static str> {
    if states.is_empty()
        || states.len() != visible.len()
        || !visible.iter().any(|&input_value| input_value)
    {
        return Err("неверная форма или пустая маска");
    }
    let mut result: Vec<[f64; 2]> = Vec::new();
    lesson_trace::trace_step!(result);
    for query in states {
        lesson_trace::trace_step!(query);
        lesson_trace::trace_note!("Делим Q·K на √2, потому что у каждого вектора две координаты.");
        let scores: Vec<f64> = states
            .iter()
            .enumerate()
            .filter(|(item_index, _)| visible[*item_index])
            .map(|(_, key)| (query[0] * key[0] + query[1] * key[1]) / 2.0_f64.sqrt())
            .collect();
        lesson_trace::trace_step!(scores);
        let maximum: f64 = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        lesson_trace::trace_step!(maximum);
        let exponential_values: Vec<f64> = scores
            .iter()
            .map(|&input_value| (input_value - maximum).exp())
            .collect();
        lesson_trace::trace_step!(exponential_values);
        let sum: f64 = exponential_values.iter().sum();
        lesson_trace::trace_step!(sum);
        let mut output: [f64; 2] = [0.0; 2];
        lesson_trace::trace_step!(output);
        let mut index: usize = 0;
        lesson_trace::trace_step!(index);
        for (position, value) in states.iter().enumerate() {
            lesson_trace::trace_step!(position);
            lesson_trace::trace_step!(value);
            if visible[position] {
                let weight: f64 = exponential_values[index] / sum;
                lesson_trace::trace_step!(weight);
                output[0] += weight * value[0];
                lesson_trace::trace_step!(output);
                output[1] += weight * value[1];
                lesson_trace::trace_step!(output);
                index += 1;
                lesson_trace::trace_step!(index);
            }
        }
        result.push(output);
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    #[test]
    fn masked_padding_does_not_change_real_output() {
        let base: Vec<[f64; 2]> =
            super::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
                &[[1.0, 0.0]],
                &[true],
            )
            .unwrap();
        lesson_trace::trace_note!(
            "Добавление пустых позиций к последовательности называют padding."
        );
        let input_with_padding: Vec<[f64; 2]> =
            super::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_products(
                &[[1.0, 0.0], [999.0, 999.0]],
                &[true, false],
            )
            .unwrap();
        assert_eq!(base[0], input_with_padding[0]);
    }
}
