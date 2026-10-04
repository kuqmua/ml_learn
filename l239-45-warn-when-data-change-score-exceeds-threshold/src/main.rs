// Урок 239. Превращать числовую оценку изменения данных в условие предупреждения.
// Проверяем строгое превышение порога: равенство порогу в этом примере ещё не вызывает
// срабатывание.

fn main() {
    let alert_threshold: f64 = 0.2;
    for (_description, distribution_change_score, expected) in [
        ("ниже порога", 0.1, false),
        ("на пороге", 0.2, false),
        ("выше порога", 0.35, true),
    ] {
        assert!(distribution_change_score >= 0.0);
        let distribution_change_exceeds_alert_threshold: bool =
            distribution_change_score > alert_threshold;
        assert_eq!(distribution_change_exceeds_alert_threshold, expected);
        println!(
            "Изменение={distribution_change_score}, порог={alert_threshold}: {}",
            if distribution_change_exceeds_alert_threshold {
                "предупреждение: проверьте новые данные"
            } else {
                "порог не превышен"
            }
        );
    }

    // Выполняем вычисления из примера.
    let _ = alert_threshold;
}

// Чему учит этот урок:
// Учимся превращать числовую оценку изменения данных в условие предупреждения.
// Проверяем строгое превышение порога: равенство порогу в этом примере ещё не вызывает
// срабатывание.
