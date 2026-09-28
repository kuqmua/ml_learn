//! Причинная свёртка одномерного сигнала.

/// Фильтр длины два читает только текущий и предыдущий элементы.
/// При генерации прогноз после `input` нельзя использовать будущий элемент.
pub fn causal_conv(
    input: &[f64],
    weight_current: f64,
    weight_previous: f64,
    dilation: usize,
) -> Result<Vec<f64>, &'static str> {
    if dilation == 0 {
        return Err("dilation должен быть положительным");
    }
    Ok(input
        .iter()
        .enumerate()
        .map(|(index, &current)| {
            let previous = index.checked_sub(dilation).map_or(0.0, |past| input[past]);
            weight_current * current + weight_previous * previous
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::causal_conv;
    #[test]
    fn future_does_not_change_past_outputs() {
        let short = causal_conv(&[1.0, 2.0], 1.0, 2.0, 1).unwrap();
        let long = causal_conv(&[1.0, 2.0, 999.0], 1.0, 2.0, 1).unwrap();
        assert_eq!(short, long[..2]);
        assert_eq!(short, [1.0, 4.0]);
    }
}
