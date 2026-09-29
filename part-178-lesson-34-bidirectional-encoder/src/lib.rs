//! Двунаправленное внимание encoder.

/// Полное self-attention: каждая позиция видит обе стороны последовательности.
pub fn bidirectional_attention(
    states: &[[f64; 2]],
    visible: &[bool],
) -> Result<Vec<[f64; 2]>, &'static str> {
    if states.is_empty()
        || states.len() != visible.len()
        || !visible.iter().any(|&input_value| input_value)
    {
        return Err("неверная форма или пустая маска");
    }
    let mut result = Vec::new();
    for query in states {
        let scores: Vec<_> = states
            .iter()
            .enumerate()
            .filter(|(item_index, _)| visible[*item_index])
            .map(|(_, key)| (query[0] * key[0] + query[1] * key[1]) / 2.0_f64.sqrt())
            .collect();
        let maximum = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let exponential_values: Vec<_> = scores
            .iter()
            .map(|&input_value| (input_value - maximum).exp())
            .collect();
        let sum: f64 = exponential_values.iter().sum();
        let mut output = [0.0; 2];
        let mut index = 0;
        for (position, value) in states.iter().enumerate() {
            if visible[position] {
                let weight = exponential_values[index] / sum;
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
        let base = super::bidirectional_attention(&[[1.0, 0.0]], &[true]).unwrap();
        // Добавление пустых позиций к последовательности называют padding.
        let input_with_padding =
            super::bidirectional_attention(&[[1.0, 0.0], [999.0, 999.0]], &[true, false]).unwrap();
        assert_eq!(base[0], input_with_padding[0]);
    }
}
