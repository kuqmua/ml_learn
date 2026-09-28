//! Вычисления и примеры урока part-061-lesson-11-f1.

/// Гармоническое среднее precision и recall.
pub fn f1(precision: Option<f64>, recall: Option<f64>) -> Option<f64> {
    match (precision, recall) {
        (Some(p), Some(r)) if p + r > 0.0 => Some(2.0 * p * r / (p + r)),
        _ => None,
    }
}

/// Получаем обе метрики из счётчиков предыдущих уроков.
pub fn f1_from_counts(counts: lesson_058::Counts) -> Option<f64> {
    f1(lesson_059::precision(counts), lesson_060::recall(counts))
}

// Урок 11.4. Мера F1.
//
// Объединяем precision и recall из двух предыдущих уроков.
// Если обе равны нулю, формула даёт 0/0, поэтому возвращаем None.

pub fn run() {
    for (description, precision, recall, expected) in [
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        let f1 = crate::f1(Some(precision), Some(recall));
        assert_eq!(f1, expected);
        println!("{description}: precision={precision}, recall={recall}, F1={f1:?}");
    }
}
