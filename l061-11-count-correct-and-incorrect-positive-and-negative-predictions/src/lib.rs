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
    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if truth.len() != predicted.len() {
        lesson_trace::trace_note!("Прерываем вычисление и возвращаем причину ошибки.");
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    lesson_trace::trace_note!("Сохраняем результат этого шага в `counts`.");
    lesson_trace::trace_note!("Задаём именованное поле или параметр.");
    lesson_trace::trace_note!("Задаём именованное поле или параметр.");
    lesson_trace::trace_note!("Задаём именованное поле или параметр.");
    lesson_trace::trace_note!("Задаём именованное поле или параметр.");
    let mut counts: BinaryClassificationCounts = BinaryClassificationCounts {
        true_positives: 0,

        false_positives: 0,

        true_negatives: 0,

        false_negatives: 0,
    };
    lesson_trace::trace_step!(counts);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for index in 0..truth.len() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_note!("Разбираем результат по его возможным вариантам.");
        lesson_trace::trace_note!("Выполняем действие для этого варианта данных.");
        lesson_trace::trace_note!("Выполняем действие для этого варианта данных.");
        lesson_trace::trace_note!("Выполняем действие для этого варианта данных.");
        lesson_trace::trace_note!("Выполняем действие для этого варианта данных.");
        match (truth[index], predicted[index]) {
            (true, true) => counts.true_positives += 1,

            (false, true) => counts.false_positives += 1,

            (false, false) => counts.true_negatives += 1,

            (true, false) => counts.false_negatives += 1,
        }
    }
    lesson_trace::trace_note!("Возвращаем успешный результат.");
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
        lesson_trace::trace_note!("Сохраняем результат этого шага в `counts`.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let counts: crate::BinaryClassificationCounts =
            super::count_binary_classification_outcomes_from_true_and_predicted_labels(
                &[true, false, true, false],
                &[true, true, false, false],
            )
            .unwrap();
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
        assert_eq!(
            (
                counts.true_positives,
                counts.false_positives,
                counts.false_negatives,
                counts.true_negatives
            ),
            (1, 1, 1, 1)
        );
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(
            super::count_binary_classification_outcomes_from_true_and_predicted_labels(
                &[true],
                &[]
            )
            .is_err()
        );
    }
}
