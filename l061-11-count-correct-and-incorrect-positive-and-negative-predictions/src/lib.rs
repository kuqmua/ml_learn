//! Урок 061. Подсчёт верных и ошибочных положительных и отрицательных прогнозов.

/// Четыре исхода бинарной классификации.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Объявляем тип с данными, необходимыми для этого вычисления.
pub struct BinaryClassificationCounts {
    // Используем подготовленное значение в следующем шаге примера.
    pub true_positives_as_correctly_detected_positive_cases: usize,
    // Используем подготовленное значение в следующем шаге примера.
    pub false_positives_as_false_alarms_on_negative_cases: usize,
    // Используем подготовленное значение в следующем шаге примера.
    pub true_negatives_as_correctly_rejected_negative_cases: usize,
    // Используем подготовленное значение в следующем шаге примера.
    pub false_negatives_as_missed_positive_cases: usize,
}

/// Сопоставляем метку с прогнозом и считаем четыре исхода.
pub fn count_binary_classification_outcomes_from_targets_and_predictions(
    truth: &[bool],
    predicted: &[bool],
) -> Result<BinaryClassificationCounts, &'static str> {
    if truth.len() != predicted.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    let mut counts: BinaryClassificationCounts = BinaryClassificationCounts {
        true_positives_as_correctly_detected_positive_cases: 0,

        false_positives_as_false_alarms_on_negative_cases: 0,

        true_negatives_as_correctly_rejected_negative_cases: 0,

        false_negatives_as_missed_positive_cases: 0,
    };
    for index in 0..truth.len() {
        match (truth[index], predicted[index]) {
            (true, true) => counts.true_positives_as_correctly_detected_positive_cases += 1,

            (false, true) => counts.false_positives_as_false_alarms_on_negative_cases += 1,

            (false, false) => counts.true_negatives_as_correctly_rejected_negative_cases += 1,

            (true, false) => counts.false_negatives_as_missed_positive_cases += 1,
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
            super::count_binary_classification_outcomes_from_targets_and_predictions(
                &[true, false, true, false],
                &[true, true, false, false],
            )
            .unwrap();
        assert_eq!(
            (
                counts.true_positives_as_correctly_detected_positive_cases,
                counts.false_positives_as_false_alarms_on_negative_cases,
                counts.false_negatives_as_missed_positive_cases,
                counts.true_negatives_as_correctly_rejected_negative_cases
            ),
            (1, 1, 1, 1)
        );
        assert!(
            super::count_binary_classification_outcomes_from_targets_and_predictions(&[true], &[])
                .is_err()
        );
    }
}
