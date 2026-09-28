// Урок 14.1. Энтропия классов.
//
// У чистого узла энтропия равна нулю, при долях 50/50 она максимальна.
// Нулевую долю пропускаем: предел p·log(p) при p→0 равен нулю.

fn main() {
    for (description, positive_fraction) in [
        ("только отрицательный класс", 0.0),
        ("четверть положительных", 0.25),
        ("классы поровну", 0.5),
        ("только положительный класс", 1.0),
    ] {
        assert!((0.0..=1.0).contains(&positive_fraction));
        let negative_fraction = 1.0 - positive_fraction;
        let mut entropy = 0.0;
        for probability in [positive_fraction, negative_fraction] {
            if probability > 0.0 {
                let ratio = (probability - 1.0) / (probability + 1.0);
                let mut term = ratio;
                let mut logarithm = 0.0;
                for odd in (1..=99).step_by(2) {
                    logarithm += term / odd as f64;
                    term *= ratio * ratio;
                }
                // Переводим натуральный логарифм в логарифм по основанию 2.
                entropy -= probability * (2.0 * logarithm) / std::f64::consts::LN_2;
            }
        }
        assert!(entropy >= -1e-10 && entropy <= 1.0 + 1e-10);
        if positive_fraction == 0.0 || positive_fraction == 1.0 {
            assert!(entropy.abs() < 1e-10);
        }
        if positive_fraction == 0.5 {
            assert!((entropy - 1.0).abs() < 1e-10);
        }
        println!("{description}: доля={positive_fraction}, энтропия={entropy:.3}");
    }
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (1..100)
        .map(|i| {
            let p = i as f64 / 100.0;
            (p, -p * p.log2() - (1.0 - p) * (1.0 - p).log2())
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Энтропия бинарного класса",
        "доля положительных",
        "энтропия",
        &[lesson_visualization::Series {
            name: "H(p)",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
