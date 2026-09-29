//! Упорядоченная статистика категорий.

/// Для строки i использует только метки предыдущих строк в заданном порядке.
pub fn ordered_category_target_mean_using_prior_rows(
    categories: &[&str],
    targets: &[f64],
    prior: f64,
    strength: f64,
) -> Result<Vec<f64>, &'static str> {
    if categories.len() != targets.len() || strength <= 0.0 || !strength.is_finite() {
        return Err("неверные входы");
    }
    let mut stats = std::collections::BTreeMap::<&str, (f64, usize)>::new();
    // Замену категорий числами, рассчитанными по целям, называют target encoding.
    let mut category_target_mean_values = Vec::with_capacity(categories.len());
    for (&category, &target) in categories.iter().zip(targets) {
        if !target.is_finite() {
            return Err("нечисловая метка");
        }
        let &(sum, count) = stats.get(category).unwrap_or(&(0.0, 0));
        category_target_mean_values.push((sum + prior * strength) / (count as f64 + strength));
        let entry = stats.entry(category).or_default();
        entry.0 += target;
        entry.1 += 1;
    }
    Ok(category_target_mean_values)
}

#[cfg(test)]
mod tests {
    #[test]
    fn current_label_cannot_enter_own_encoding() {
        let first_encoding = super::ordered_category_target_mean_using_prior_rows(
            &["a", "a"],
            &[0.0, 1.0],
            0.5,
            1.0,
        )
        .unwrap();
        let second_encoding = super::ordered_category_target_mean_using_prior_rows(
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
