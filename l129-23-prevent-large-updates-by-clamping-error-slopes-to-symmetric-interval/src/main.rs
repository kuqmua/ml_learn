// Урок 23.5. Защита от слишком больших обновлений: ограничение скоростей изменения ошибки заданным интервалом.
// Связь с принятой терминологией: Ограничение координат градиента симметричным интервалом.
// Зачем здесь эта тема: Редкий очень большой градиент способен испортить устойчивость шага.
// Почему код устроен так: Ограничиваем координаты заданным интервалом и сравниваем обновление до и
//   после.
// Представь: Если градиент равен 100, а предел 5, в обновление пойдёт 5.
//
// Значения внутри интервала [-limit, limit] не меняются. Выходящие за границу
// заменяются ближайшей границей с сохранением знака.

fn main() {
    let limit: f64 = 1.0;
    assert!(limit > 0.0);
    for (_description, rate_of_change, expected) in [
        ("слишком большой положительный", 12.0, 1.0),
        ("положительный внутри интервала", 0.5, 0.5),
        ("нулевой", 0.0, 0.0),
        ("отрицательный внутри интервала", -0.5, -0.5),
        ("слишком большой отрицательный", -12.0, -1.0),
    ] {
        let clipped: f64 = if rate_of_change > limit {
            limit
        } else if rate_of_change < -limit {
            -limit
        } else {
            rate_of_change
        };
        assert_eq!(clipped, expected);
    }

    plot_rate_of_change_clamped_to_symmetric_interval();
}

// Строим график по результатам урока.
fn plot_rate_of_change_clamped_to_symmetric_interval() {
    let bounded_value_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            let rate_of_change_value: f64 = plot_step_index as f64 / 10.0;
            (rate_of_change_value, rate_of_change_value.clamp(-1.0, 1.0))
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ограничение градиента",
        "градиент до",
        "градиент после",
        &[lesson_visualization::Series {
            name: "порог ±1",

            points: &bounded_value_points,
        }],
    )
    .expect("не удалось сохранить график");
}
