//! Вычисления и примеры урока part-058-lesson-11-confusion-matrix.

/// Четыре исхода бинарной классификации.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counts {
    pub true_positives: usize,
    pub false_positives: usize,
    pub true_negatives: usize,
    pub false_negatives: usize,
}

/// Сопоставляем метку с прогнозом и считаем четыре исхода.
pub fn count_outcomes(truth: &[bool], predicted: &[bool]) -> Result<Counts, &'static str> {
    if truth.len() != predicted.len() {
        return Err("число прогнозов должно совпадать с числом ответов");
    }
    let mut counts = Counts {
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

/// Порог превращает оценки в метки перед подсчётом исходов.
pub fn count_outcomes_at_threshold(
    truth: &[bool],
    scores: &[f64],
    threshold: f64,
) -> Result<Counts, &'static str> {
    if truth.len() != scores.len() {
        return Err("число оценок должно совпадать с числом ответов");
    }
    let predicted: Vec<bool> = scores.iter().map(|&score| score >= threshold).collect();
    count_outcomes(truth, &predicted)
}

/// Общая доля верных прогнозов.
pub fn accuracy(counts: Counts) -> Option<f64> {
    let total = counts.true_positives
        + counts.false_positives
        + counts.true_negatives
        + counts.false_negatives;
    if total == 0 {
        None
    } else {
        Some((counts.true_positives + counts.true_negatives) as f64 / total as f64)
    }
}

// Урок 11.1. Матрица ошибок классификации.
//
// Каждая пара «истина, прогноз» попадает ровно в одну из четырёх ячеек.
// Эти счётчики затем повторно используются в precision, recall, F1 и сводной практике.

pub fn run() {
    let truth = [true, false, true, false];
    let predicted = [true, true, false, false];
    let counts = crate::count_outcomes(&truth, &predicted).expect("у каждого ответа есть прогноз");
    for index in 0..truth.len() {
        let description = match (truth[index], predicted[index]) {
            (true, true) => "TP: верно найден положительный класс",
            (false, true) => "FP: ложная тревога",
            (true, false) => "FN: положительный класс пропущен",
            (false, false) => "TN: верно найден отрицательный класс",
        };
        println!(
            "истина={}, прогноз={} → {description}",
            truth[index], predicted[index]
        );
    }
    assert_eq!(
        (
            counts.true_positives,
            counts.false_positives,
            counts.false_negatives,
            counts.true_negatives
        ),
        (1, 1, 1, 1)
    );
    println!("итоговые счётчики: {counts:?}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn separates_all_four_classification_outcomes() {
        let counts =
            super::count_outcomes(&[true, false, true, false], &[true, true, false, false])
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
        assert_eq!(super::accuracy(counts), Some(0.5));
        assert!(super::count_outcomes(&[true], &[]).is_err());
    }
}
