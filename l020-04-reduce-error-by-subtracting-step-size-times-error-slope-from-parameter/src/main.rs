// Урок 04.1. Шаг уменьшения ошибки: вычитание из параметра произведения размера шага и скорости изменения ошибки.
// Связь с принятой терминологией: Скорость обучения в шаге градиентного спуска.
// Зачем здесь эта тема: Градиент показывает направление, но не размер шага; его задаёт скорость
//   обучения.
// Почему код устроен так: Сравниваем обновления одного параметра при разных коэффициентах, чтобы
//   увидеть перескок и медленное движение.
// Представь: Большой шаг может перепрыгнуть минимум ошибки, а слишком маленький потребует много
//   повторений.
//
// Что изучаем: Скорость обучения.
// Зачем это нужно: Один и тот же градиент даёт разные шаги при разных learning rate. Слишком большой шаг
// может перескочить минимум.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let parameter: f64 = 0.0;
    let rate_of_change: f64 = 2.0 * (parameter - 3.0);
    for rate in [0.1, 1.0, 2.0] {
        let updated: f64 = parameter - rate * rate_of_change;
        let _error: f64 = (updated - 3.0) * (updated - 3.0);
    }

    plot_squared_error_after_one_update_for_different_step_sizes(parameter, rate_of_change);
}

// Строим график по результатам урока.
fn plot_squared_error_after_one_update_for_different_step_sizes(
    parameter: f64,
    rate_of_change: f64,
) {
    let learning_rates: [f64; 3] = [0.1, 1.0, 2.0];

    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "learning-rate",
        "Ошибка после одного шага",
        "Скорость обучения",
        "Квадратичная ошибка",
        &[lesson_visualization::Series {
            name: "Ошибка",

            points: &learning_rates
                .into_iter()
                .map(|rate| {
                    let updated: f64 = parameter - rate * rate_of_change;
                    (rate, (updated - 3.0) * (updated - 3.0))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
