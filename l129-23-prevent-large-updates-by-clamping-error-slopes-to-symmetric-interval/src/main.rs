// Урок 23.5. Защита от слишком больших обновлений: ограничение скоростей изменения ошибки заданным интервалом.
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
    for (
        _description,
        loss_slope_where_pos_calls_for_decreasing_parameter_and_neg_calls_for_increasing_it,
        expected,
    ) in [
        ("слишком большой положительный", 12.0, 1.0),
        ("положительный внутри интервала", 0.5, 0.5),
        ("нулевой", 0.0, 0.0),
        ("отрицательный внутри интервала", -0.5, -0.5),
        ("слишком большой отрицательный", -12.0, -1.0),
    ] {
        let gradient_with_magnitude_limited_and_sign_preserved: f64 = if loss_slope_where_pos_calls_for_decreasing_parameter_and_neg_calls_for_increasing_it > limit {
            limit
        } else if loss_slope_where_pos_calls_for_decreasing_parameter_and_neg_calls_for_increasing_it < -limit {
            -limit
        } else {
            loss_slope_where_pos_calls_for_decreasing_parameter_and_neg_calls_for_increasing_it
        };
        assert_eq!(gradient_with_magnitude_limited_and_sign_preserved, expected);
    }

    plot_rate_of_change_clamped_to_symmetric_interval();
}

// Строим график по результатам урока.
fn plot_rate_of_change_clamped_to_symmetric_interval() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ограничение градиента",
        "градиент до",
        "градиент после",
        &[lesson_visualization::Series {
            name: "порог ±1",

            points: &(-30..=30)
                .map(|plot_step_index| {
                    let rate_of_change_value: f64 = plot_step_index as f64 / 10.0;
                    (rate_of_change_value, rate_of_change_value.clamp(-1.0, 1.0))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
