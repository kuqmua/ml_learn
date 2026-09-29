//! Двунаправленное внимание encoder.

/// Полное self-attention: каждая позиция видит обе стороны последовательности.
pub fn bidirectional_self_attention_over_visible_states(
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
    for query in states {
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
        result.push(output);
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    #[test]
    fn masked_padding_does_not_change_real_output() {
        let base: Vec<[f64; 2]> =
            super::bidirectional_self_attention_over_visible_states(&[[1.0, 0.0]], &[true])
                .unwrap();
        // Добавление пустых позиций к последовательности называют padding.
        let input_with_padding: Vec<[f64; 2]> =
            super::bidirectional_self_attention_over_visible_states(
                &[[1.0, 0.0], [999.0, 999.0]],
                &[true, false],
            )
            .unwrap();
        assert_eq!(base[0], input_with_padding[0]);
    }
}
