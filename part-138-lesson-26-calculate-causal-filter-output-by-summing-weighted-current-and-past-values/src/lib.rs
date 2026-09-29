//! Урок 138. Отклик фильтра без будущих данных: сложение взвешенных текущего и прошлого значений сигнала.
//! Связь с принятой терминологией: Причинная свёртка одномерного сигнала.

/// Фильтр длины два читает только текущий и предыдущий элементы.
/// При генерации прогноз после `input` нельзя использовать будущий элемент.
/// Причинная свёртка: вес текущего отсчёта умножаем на него и прибавляем взвешенный прошлый отсчёт с заданным отступом.
pub fn calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
    input: &[f64],
    weight_current: f64,
    weight_previous: f64,
    // Промежуток между используемыми точками фильтра называют dilation.
    filter_spacing: usize,
) -> Result<Vec<f64>, &'static str> {
    if filter_spacing == 0 {
        return Err("dilation должен быть положительным");
    }
    Ok(input
        .iter()
        .enumerate()
        .map(|(index, &current)| {
            let previous: f64 = index
                .checked_sub(filter_spacing)
                .map_or(0.0, |past| input[past]);
            lesson_trace::trace_step!(previous);
            weight_current * current + weight_previous * previous
        })
        .collect())
}

#[cfg(test)]
mod tests {
    #[test]
    fn future_does_not_change_past_outputs() {
        let short: Vec<f64> =
            super::calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(&[1.0, 2.0], 1.0, 2.0, 1)
                .unwrap();
        let long: Vec<f64> = super::calculate_causal_filter_output_by_summing_weighted_current_and_spaced_past_values(
            &[1.0, 2.0, 999.0],
            1.0,
            2.0,
            1,
        )
        .unwrap();
        assert_eq!(short, long[..2]);
        assert_eq!(short, [1.0, 4.0]);
    }
}
