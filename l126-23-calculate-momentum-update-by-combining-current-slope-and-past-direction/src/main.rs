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
    let rates_of_change: [f64; 3] = [2.0, 1.0, -0.5];
    let mut velocity: f64 = 0.0;
    let mut weight: f64 = 1.0;
    let mut weight_history: Vec<(f64, f64)> = vec![(0.0, weight)];
    let mut velocity_history: Vec<(f64, f64)> = vec![(0.0, velocity)];
    for (step, rate_of_change) in rates_of_change.into_iter().enumerate() {
        velocity = 0.8 * velocity + rate_of_change;
        weight -= 0.1 * velocity;

        weight_history.push(((step + 1) as f64, weight));
        velocity_history.push(((step + 1) as f64, velocity));
    }

    plot_weight_and_accumulated_update_direction(weight_history, velocity_history);
}

// Строим график по результатам урока.
fn plot_weight_and_accumulated_update_direction(
    weight_history: std::vec::Vec<(f64, f64)>,
    velocity_history: std::vec::Vec<(f64, f64)>,
) {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
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
