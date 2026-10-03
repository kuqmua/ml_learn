// Урок 23.2. Обновление с накоплением направления: объединение текущего и прошлых изменений.
// Связь с принятой терминологией: Обновление параметра с учётом текущего и прошлых градиентов.
// Зачем здесь эта тема: Шумный SGD может колебаться; momentum накапливает направление прошлых
//   шагов.
// Почему код устроен так: Храним скорость и обновляем её из текущего градиента и предыдущего
//   состояния.
// Представь: Если несколько градиентов подряд указывают вправо, momentum накапливает это
//   направление.
//
// Что изучаем: Импульс momentum.
// Зачем это нужно: Скорость накапливает прежние градиенты и сглаживает последовательность обновлений.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let mut accumulated_gradient_for_momentum_update: f64 = 0.0;
    let mut weight: f64 = 1.0;
    let mut weight_history: [(f64, f64); 4] = [(0.0, weight); 4];
    let mut velocity_history: [(f64, f64); 4] =
        [(0.0, accumulated_gradient_for_momentum_update); 4];
    let rates_of_change: [f64; 3] = [2.0, 1.0, -0.5];
    for (step, rate_of_change) in rates_of_change.into_iter().enumerate() {
        accumulated_gradient_for_momentum_update =
            0.8 * accumulated_gradient_for_momentum_update + rate_of_change;
        weight -= 0.1 * accumulated_gradient_for_momentum_update;

        weight_history[step + 1] = ((step + 1) as f64, weight);
        velocity_history[step + 1] = ((step + 1) as f64, accumulated_gradient_for_momentum_update);
    }

    plot_weight_and_accumulated_update_direction(weight_history, velocity_history);
}

// Строим график по результатам урока.
fn plot_weight_and_accumulated_update_direction(
    weight_history: [(f64, f64); 4],
    velocity_history: [(f64, f64); 4],
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Momentum: накопление скорости",
        "шаг",
        "значение",
        &[
            lesson_visualization::Series {
                name: "вес",

                points: &weight_history,
            },
            lesson_visualization::Series {
                name: "скорость",

                points: &velocity_history,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
