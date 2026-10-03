// Урок 45.4. Предупреждение при превышении порога оценки изменения данных.
// Связь с принятой терминологией: Предупреждение при превышении порога оценкой дрейфа.
// Зачем здесь эта тема: Мониторинг полезен только тогда, когда изменение приводит к действию.
// Почему код устроен так: Сравниваем оценку дрейфа с заранее выбранным порогом и формируем
//   предупреждение.
// Представь: Если доля новых значений в корзине выросла выше порога, мониторинг выдаёт
//   предупреждение.
//
// Здесь предупреждаем, только когда оценка дрейфа строго выше порога.
// Значение точно на пороге ещё не вызывает предупреждение.

fn main() {
    let alert_threshold: f64 = 0.2;
    for (_description, distribution_change_score_where_larger_means_more_change, expected) in [
        ("ниже порога", 0.1, false),
        ("на пороге", 0.2, false),
        ("выше порога", 0.35, true),
    ] {
        assert!(distribution_change_score_where_larger_means_more_change >= 0.0);
        let distribution_change_exceeds_alert_threshold: bool =
            distribution_change_score_where_larger_means_more_change > alert_threshold;
        assert_eq!(distribution_change_exceeds_alert_threshold, expected);
    }

    plot_distribution_change_scores_and_alert_threshold(alert_threshold);
}

// Строим график по результатам урока.
fn plot_distribution_change_scores_and_alert_threshold(alert_threshold: f64) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Порог алерта и два случая",
        "оценка дрейфа",
        &[("ниже", 0.1), ("порог", alert_threshold), ("выше", 0.35)],
    )
    .expect("не удалось сохранить график");
}
