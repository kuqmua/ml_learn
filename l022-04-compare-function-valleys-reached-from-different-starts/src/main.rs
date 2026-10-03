// Урок 04.3. Сравнение минимумов функции при движении из разных начальных точек.
// Связь с принятой терминологией: Локальные минимумы невыпуклой функции.
// Зачем здесь эта тема: Для невыпуклой ошибки разные начальные точки могут привести к разным
//   минимумам.
// Почему код устроен так: Показываем несколько впадин, чтобы не принимать найденный минимум за
//   глобальный.
// Представь: Шарик в рельефе с двумя ямами останется в ближайшей яме, хотя другая может быть
//   глубже.
//
// Что изучаем: Локальные минимумы.
// Зачем это нужно: У невыпуклой функции разные начальные точки могут привести к разным минимумам.
// Исследуем f(x)=x⁴−2x².

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    for start in [-0.5, 0.5] {
        let mut input_value: f64 = start;
        for _ in 0..100 {
            let loss_slope_where_pos_calls_for_decreasing_parameter_and_neg_calls_for_increasing_it: f64 =
                4.0 * input_value * input_value * input_value - 4.0 * input_value;
            input_value -= 0.1 * loss_slope_where_pos_calls_for_decreasing_parameter_and_neg_calls_for_increasing_it;
        }
        let _: f64 =
            input_value * input_value * input_value * input_value - 2.0 * input_value * input_value;
    }

    plot_function_with_two_valleys();
}

// Строим график по результатам урока.
fn plot_function_with_two_valleys() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Два локальных минимума",
        "x",
        "f(x)",
        &[lesson_visualization::Series {
            name: "x⁴−2x²",

            points: &(-150..=150)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 100.0;
                    (
                        horizontal_value,
                        horizontal_value * horizontal_value * horizontal_value * horizontal_value
                            - 2.0 * horizontal_value * horizontal_value,
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
