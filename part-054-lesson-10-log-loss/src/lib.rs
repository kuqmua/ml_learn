//! Вычисления и примеры урока part-054-lesson-10-log-loss.

// Урок 10.2. Логарифмическая ошибка.
//
// Уверенный правильный прогноз имеет малую ошибку; уверенный неверный — большую.
// Вероятности 0 и 1 дают бесконечную ошибку для неверного класса, поэтому пример
// считает только строго внутренние вероятности.

pub fn run() {
    let cases = [
        ("верный уверенный прогноз", 1.0, 0.9),
        ("неуверенный прогноз", 1.0, 0.5),
        ("неверный уверенный прогноз", 1.0, 0.1),
        ("отрицательный класс предсказан верно", 0.0, 0.1),
    ];
    let mut losses = [0.0; 4];
    for (index, (description, target, probability)) in cases.into_iter().enumerate() {
        assert!(target == 0.0 || target == 1.0);
        assert!(probability > 0.0 && probability < 1.0);
        let chosen_probability = if target == 1.0 {
            probability
        } else {
            1.0 - probability
        };
        // ln(x) ≈ 2·(t+t³/3+t⁵/5+...), t=(x−1)/(x+1).
        let ratio = (chosen_probability - 1.0) / (chosen_probability + 1.0);
        let mut term = ratio;
        let mut logarithm = 0.0;
        for odd in (1..=99).step_by(2) {
            logarithm += term / odd as f64;
            term *= ratio * ratio;
        }
        losses[index] = -2.0 * logarithm;
        println!(
            "{description}: target={target}, вероятность={probability}, loss={:.3}",
            losses[index]
        );
    }
    assert!(losses[0] < losses[1] && losses[1] < losses[2]);
    assert!((losses[0] - losses[3]).abs() < 1e-10);
}
