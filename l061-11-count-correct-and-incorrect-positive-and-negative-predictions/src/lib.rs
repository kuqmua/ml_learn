//! Урок 061. Подсчёт верных и ошибочных положительных и отрицательных прогнозов.

/// Четыре исхода бинарной классификации.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Объявляем тип с данными, необходимыми для этого вычисления.
pub struct BinaryClassificationCounts {
    // Используем подготовленное значение в следующем шаге примера.
    pub true_positives: usize,
    // Используем подготовленное значение в следующем шаге примера.
    pub false_positives: usize,
    // Используем подготовленное значение в следующем шаге примера.
    pub true_negatives: usize,
    // Используем подготовленное значение в следующем шаге примера.
    pub false_negatives: usize,
}

/// Сопоставляем метку с прогнозом и считаем четыре исхода.
pub fn count_binary_classification_outcomes_from_true_and_predicted_labels(
    truth: &[bool],
    predicted: &[bool],
) -> Result<BinaryClassificationCounts, &'static str> {
    if truth.len() != predicted.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    let mut counts: BinaryClassificationCounts = BinaryClassificationCounts {
        true_positives: 0,

        false_positives: 0,

        true_negatives: 0,

        false_negatives: 0,
    };
    for index in 0..truth.len() {
        match (truth[index], predicted[index]) {
            (true, true) => counts.true_positives += 1,

            (false, true) => counts.false_positives += 1,

            (false, false) => counts.true_negatives += 1,

            (true, false) => counts.false_negatives += 1,
        }
    }
    Ok(counts)
}

// Добавляем свойство для следующего определения.
#[cfg(test)]
// Используем подготовленное значение в следующем шаге примера.
mod tests {

    // Добавляем свойство для следующего определения.
    #[test]
    // Определяем вычисление `separates_all_four_classification_outcomes` для этого примера.
    fn separates_all_four_classification_outcomes() {
        let counts: crate::BinaryClassificationCounts =
            super::count_binary_classification_outcomes_from_true_and_predicted_labels(
                &[true, false, true, false],
                &[true, true, false, false],
            )
            .unwrap();
        assert_eq!(
            (
                counts.true_positives,
                counts.false_positives,
                counts.false_negatives,
                counts.true_negatives
            ),
            (1, 1, 1, 1)
        );
        assert!(
            super::count_binary_classification_outcomes_from_true_and_predicted_labels(
                &[true],
                &[]
            )
            .is_err()
        );
    }
}
