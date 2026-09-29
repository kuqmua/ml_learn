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
    // Выбираем дальнейший шаг по выполнению условия.
    if truth.len() != predicted.len() {
        // Прерываем вычисление и возвращаем причину ошибки.
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    // Сохраняем результат этого шага в `counts`.
    let mut counts: BinaryClassificationCounts = BinaryClassificationCounts {
        // Задаём именованное поле или параметр.
        true_positives: 0,
        // Задаём именованное поле или параметр.
        false_positives: 0,
        // Задаём именованное поле или параметр.
        true_negatives: 0,
        // Задаём именованное поле или параметр.
        false_negatives: 0,
    };
    lesson_trace::trace_step!(counts);
    // Повторяем расчёт для каждого элемента последовательности.
    for index in 0..truth.len() {
        lesson_trace::trace_step!(index);
        // Разбираем результат по его возможным вариантам.
        match (truth[index], predicted[index]) {
            // Выполняем действие для этого варианта данных.
            (true, true) => counts.true_positives += 1,
            // Выполняем действие для этого варианта данных.
            (false, true) => counts.false_positives += 1,
            // Выполняем действие для этого варианта данных.
            (false, false) => counts.true_negatives += 1,
            // Выполняем действие для этого варианта данных.
            (true, false) => counts.false_negatives += 1,
        }
    }
    // Возвращаем успешный результат.
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
        // Сохраняем результат этого шага в `counts`.
        let counts: crate::BinaryClassificationCounts =
            // Используем подготовленное значение в следующем шаге примера.
            super::count_binary_classification_outcomes_from_true_and_predicted_labels(&[true, false, true, false], &[true, true, false, false])
                // Используем результат, ожидая успешного выполнения шага.
                .unwrap();
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(
            (
                // Используем подготовленное значение в следующем шаге примера.
                counts.true_positives,
                // Используем подготовленное значение в следующем шаге примера.
                counts.false_positives,
                // Используем подготовленное значение в следующем шаге примера.
                counts.false_negatives,
                // Используем подготовленное значение в следующем шаге примера.
                counts.true_negatives
            ),
            // Добавляем пару значений для сравнения или построения графика.
            (1, 1, 1, 1)
        );
        // Проверяем ожидаемое свойство учебного примера.
        assert!(
            super::count_binary_classification_outcomes_from_true_and_predicted_labels(
                &[true],
                &[]
            )
            .is_err()
        );
    }
}
