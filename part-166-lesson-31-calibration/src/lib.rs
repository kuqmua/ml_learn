//! Вычисления и примеры урока part-166-lesson-31-calibration.

// Урок 31.2. Калибровка вероятностей.
//
// Если модель сообщает 0.8 многим объектам, событие должно происходить примерно в 80% случаев.
// Сравниваем совпадение прогноза с наблюдаемой частотой и чрезмерную уверенность.

pub fn run() {
    let observed_labels = [true, true, true, false, true];
    assert!(
        !observed_labels.is_empty(),
        "для частоты нужна хотя бы одна метка"
    );
    let observed_frequency = observed_labels.iter().filter(|&&label| label).count() as f64
        / observed_labels.len() as f64;
    for (description, predicted_probability, expected_gap) in [
        ("калиброванный прогноз", 0.8, 0.0),
        ("слишком уверенный", 1.0, 0.2),
        ("недооценка", 0.6, 0.2),
    ] {
        assert!((0.0..=1.0).contains(&predicted_probability));
        let gap = (predicted_probability - observed_frequency).abs();
        assert!((gap - expected_gap).abs() < 1e-10);
        println!(
            "{description}: прогноз={predicted_probability}, частота={observed_frequency}, разница={gap:.2}"
        );
    }
}
