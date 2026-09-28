// Урок 34.4. Алерты мониторинга.
//
// Здесь предупреждаем, только когда оценка дрейфа строго выше порога.
// Значение точно на пороге ещё не вызывает предупреждение.

fn main() {
    let alert_threshold = 0.2;
    for (description, drift_score, expected) in [
        ("ниже порога", 0.1, false),
        ("на пороге", 0.2, false),
        ("выше порога", 0.35, true),
    ] {
        assert!(drift_score >= 0.0);
        let alert = drift_score > alert_threshold;
        assert_eq!(alert, expected);
        println!("{description}: дрейф={drift_score}, требуется проверка={alert}");
    }
}
