//! Урок 092. Кодирование категории: среднее предыдущих ответов без ответа текущей строки.
//! Связь с принятой терминологией: Упорядоченная статистика категорий.

/// Для строки i использует только метки предыдущих строк в заданном порядке.
/// Упорядоченная статистика категории: (сумма предыдущих ответов + prior·strength) / (их число + strength).
pub fn encode_categories_as_average_previous_targets_with_prior_weight(
    categories: &[&str],
    targets: &[f64],
    prior: f64,
    strength: f64,
) -> Result<Vec<f64>, &'static str> {
    if categories.len() != targets.len() || strength <= 0.0 || !strength.is_finite() {
        return Err("неверные входы");
    }
    let mut stats: std::collections::BTreeMap<&str, (f64, usize)> =
        std::collections::BTreeMap::<&str, (f64, usize)>::new();
    lesson_trace::trace_step!(stats);
    lesson_trace::trace_note!(
        "Замену категорий числами, рассчитанными по целям, называют target encoding."
    );
    let mut category_target_mean_values: Vec<f64> = Vec::with_capacity(categories.len());
    lesson_trace::trace_step!(category_target_mean_values);
    for (&category, &target) in categories.iter().zip(targets) {
        lesson_trace::trace_step!(category);
        lesson_trace::trace_step!(target);
        if !target.is_finite() {
            return Err("нечисловая метка");
        }
        let &(sum, count) = stats.get(category).unwrap_or(&(0.0, 0));
        lesson_trace::trace_step!(sum);
        lesson_trace::trace_step!(count);
        category_target_mean_values.push((sum + prior * strength) / (count as f64 + strength));
        let entry: &mut (f64, usize) = stats.entry(category).or_default();
        lesson_trace::trace_step!(entry);
        entry.0 += target;
        lesson_trace::trace_step!(entry);
        entry.1 += 1;
        lesson_trace::trace_step!(entry);
    }
    Ok(category_target_mean_values)
}

#[cfg(test)]
mod tests {
    #[test]
    fn current_label_cannot_enter_own_encoding() {
        let first_encoding: Vec<f64> =
            super::encode_categories_as_average_previous_targets_with_prior_weight(
                &["a", "a"],
                &[0.0, 1.0],
                0.5,
                1.0,
            )
            .unwrap();
        let second_encoding: Vec<f64> =
            super::encode_categories_as_average_previous_targets_with_prior_weight(
                &["a", "a"],
                &[0.0, 0.0],
                0.5,
                1.0,
            )
            .unwrap();
        assert_eq!(first_encoding[0], 0.5);
        assert_eq!(first_encoding[1], 0.25);
        assert_eq!(first_encoding[1], second_encoding[1]);
    }
}
