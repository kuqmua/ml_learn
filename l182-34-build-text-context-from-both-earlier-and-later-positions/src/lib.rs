//! Урок 182. Построение контекста текста по предыдущим и следующим позициям.
//! Связь с принятой терминологией: Двунаправленное внимание encoder.

/// Полное self-attention: каждая позиция видит обе стороны последовательности.
/// Двунаправленное внимание: для каждого состояния считаем произведения координат с видимыми состояниями, делим на sqrt(2), применяем softmax и суммируем состояния с этими весами.

pub fn calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_scores<
    const N: usize,
>(
    states: &[[f64; 2]; N],
    visible: &[bool; N],
) -> Result<[[f64; 2]; N], &'static str> {
    if N == 0 || !visible.iter().any(|&input_value| input_value) {
        return Err("пустая последовательность или маска");
    }
    let result: [[f64; 2]; N] = std::array::from_fn(|query_index| {
        let query = states[query_index];
        let scores: Vec<f64> = states
            .iter()
            .enumerate()
            .filter(|(item_index, _)| visible[*item_index])
            .map(|(_, key)| (query[0] * key[0] + query[1] * key[1]) / 2.0_f64.sqrt())
            .collect();
        let maximum: f64 = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let exponential_values: Vec<f64> = scores
            .iter()
            .map(|&input_value| (input_value - maximum).exp())
            .collect();
        let sum: f64 = exponential_values.iter().sum();
        let mut output: [f64; 2] = [0.0; 2];
        let mut index: usize = 0;
        for (position, value) in states.iter().enumerate() {
            if visible[position] {
                let weight: f64 = exponential_values[index] / sum;
                output[0] += weight * value[0];
                output[1] += weight * value[1];
                index += 1;
            }
        }
        output
    });
    Ok(result)
}
#[cfg(test)]
mod tests {

    #[test]
    fn masked_padding_does_not_change_real_output() {
        let base: [[f64; 2]; 1] =
            super::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_scores(
                &[[1.0, 0.0]],
                &[true],
            )
            .unwrap();
        let input_with_padding: [[f64; 2]; 2] =
            super::calculate_visible_context_by_summing_states_weighted_by_exponentiated_coordinate_scores(
                &[[1.0, 0.0], [999.0, 999.0]],
                &[true, false],
            )
            .unwrap();
        assert_eq!(base[0], input_with_padding[0]);
    }
}
