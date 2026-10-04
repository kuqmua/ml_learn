//! Урок 092. Кодирование категории: среднее предыдущих ответов без ответа текущей строки.

/// Для строки i использует только метки предыдущих строк в заданном порядке.
/// Упорядоченная статистика категории: (сумма предыдущих ответов + prior·strength) / (их число + strength).
/// Массивы категорий и ответов содержат по `N` строк; это проверяет компилятор.

pub fn encode_categories_as_average_previous_targets_with_prior_weight<const N: usize>(
    categories: &[&str; N],
    targets: &[f64; N],
    prior: f64,
    strength: f64,
) -> Result<[f64; N], &'static str> {
    if strength <= 0.0 || !strength.is_finite() {
        return Err("strength должен быть положительным конечным числом");
    }
    let mut stats: std::collections::BTreeMap<&str, (f64, usize)> =
        std::collections::BTreeMap::<&str, (f64, usize)>::new();
    let mut category_target_mean_values: Vec<f64> = Vec::with_capacity(categories.len());
    for (&category, &target) in categories.iter().zip(targets) {
        if !target.is_finite() {
            return Err("метка должна быть конечным числом");
        }
        let &(sum, count) = stats.get(category).unwrap_or(&(0.0, 0));
        category_target_mean_values.push((sum + prior * strength) / (count as f64 + strength));
        let entry: &mut (f64, usize) = stats.entry(category).or_default();
        entry.0 += target;
        entry.1 += 1;
    }
    Ok(category_target_mean_values
        .try_into()
        .expect("ожидалось ровно N значений статистики по категориям"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn current_target_cannot_enter_own_encoding() {
        let encoding1: [f64; 2] =
            super::encode_categories_as_average_previous_targets_with_prior_weight(
                &["a", "a"],
                &[0.0, 1.0],
                0.5,
                1.0,
            )
            .unwrap();
        assert_eq!(encoding1[0], 0.5);
        assert_eq!(encoding1[1], 0.25);
        assert_eq!(
            encoding1[1],
            super::encode_categories_as_average_previous_targets_with_prior_weight(
                &["a", "a"],
                &[0.0, 0.0],
                0.5,
                1.0,
            )
            .unwrap()[1]
        );
    }
}
