// Урок 04.5. Практика: выбор размера шага, способа обновления и проверка уменьшения ошибки.
// Связь с принятой терминологией: Скорость обучения, сходимость и режимы градиентного спуска.
// Зачем здесь эта тема: Оптимизация требует одновременно выбрать начальную точку, шаг и способ
//   расчёта градиента.
// Почему код устроен так: На одной функции соединяем режимы обновления и наблюдаем всю траекторию,
//   а не только итог.
// Представь: Две траектории могут начать одинаково, но из-за разного шага или режима обновления
//   прийти к разным точкам.
//
// Что повторяем вместе: скорость обучения, сходимость, локальные минимумы, batch и stochastic updates.
// Зачем это нужно: Скорость обучения определяет, приблизится ли последовательность обновлений к минимуму
//   или начнёт расходиться.
// Что показывает программа: Запускаем спуск с тремя скоростями обучения. Сохраняем loss после каждого
//   обновления параметра. По первой и последней ошибке видим сходимость или расходимость.
// Что проверить при изменении примера: Покажи успешный, медленный и расходящийся запуск; критерий остановки
//   не должен зависеть только от числа шагов.
// Дополнительная практика: Минимизируй квадратичную функцию и запиши историю loss для нескольких learning
//   rate.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    fn calculate_squared_distance_of_parameter_from_three_as_loss_where_0_means_parameter_equals_3_and_larger_means_farther(
        parameter: f64,
    ) -> f64 {
        (|| -> f64 {
            let value: f64 = parameter - 3.0;
            value * value
        })()
    }

    for learning_rate in [0.01, 0.2, 1.1] {
        let history: Vec<f64> = (|| -> Vec<f64> {
            let learning_rate: f64 = learning_rate;
            let mut parameter: f64 = 0.0;
            let mut history: Vec<f64> = vec![calculate_squared_distance_of_parameter_from_three_as_loss_where_0_means_parameter_equals_3_and_larger_means_farther(
                parameter,
            )];
            let steps: usize = 30;
            for _ in 0..steps {
                let loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it: f64 = (|| -> f64 {
                    let parameter: f64 = parameter;
                    2.0 * (parameter - 3.0)
                })();
                if (|| -> f64 {
                    let value: f64 = loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it;
                    if value < 0.0 { -value } else { value }
                })() < 1e-8
                {
                    break;
                }
                parameter -= learning_rate * loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it;
                history.push(calculate_squared_distance_of_parameter_from_three_as_loss_where_0_means_parameter_equals_3_and_larger_means_farther(
                    parameter,
                ));
                if !parameter.is_finite() {
                    break;
                }
            }
            history
        })();
        let _ = (
            &(history[0]),
            &(history[history.len() - 1]),
            &(history.len() - 1),
        );
    }

    plot_errors_over_updates_for_different_step_sizes();

    fn plot_errors_over_updates_for_different_step_sizes() {
        let rates: [f64; 3] = [0.01, 0.2, 1.1];
        let histories: [Vec<(f64, f64)>; 3] = std::array::from_fn(|index| {
            let rate = rates[index];
            let mut parameter: f64 = 0.0;
            let mut points: Vec<(f64, f64)> = vec![(
                0.0,
                (calculate_squared_distance_of_parameter_from_three_as_loss_where_0_means_parameter_equals_3_and_larger_means_farther(parameter) + 1e-12).log10(),
            )];
            for step in 1..=30 {
                let loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it: f64 = 2.0 * (parameter - 3.0);
                parameter -= rate * loss_slope_where_positive_calls_for_decreasing_parameter_and_negative_calls_for_increasing_it;
                points.push((
                    step as f64,
                    (calculate_squared_distance_of_parameter_from_three_as_loss_where_0_means_parameter_equals_3_and_larger_means_farther(parameter) + 1e-12).log10(),
                ));
            }
            points
        });
        lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Скорость обучения и ошибка",
            "шаг",
            "log₁₀(loss)",
            &[
                lesson_visualization::Series {
                    name: "η=0.01",

                    points: &histories[0],
                },
                lesson_visualization::Series {
                    name: "η=0.2",

                    points: &histories[1],
                },
                lesson_visualization::Series {
                    name: "η=1.1",

                    points: &histories[2],
                },
            ],
        )
        .expect("не удалось сохранить график");
    }
}
