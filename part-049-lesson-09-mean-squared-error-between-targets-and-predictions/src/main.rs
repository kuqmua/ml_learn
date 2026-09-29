// Урок 09.1. Средний квадрат разности правильных ответов и прогнозов.
//
// При точном прогнозе MSE равна нулю. Ошибка вдвое больше даёт вклад вчетверо больше.
// Та же общая функция будет использоваться для оценки моделей в следующих уроках.

fn main() {
    // Задаём учебные значения для `targets`.
    let targets = [2.0, 4.0, 6.0];
    // Задаём учебные значения для `cases`.
    let cases: [(&str, &[f64], f64); 3] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, predictions, expected) in cases {
        // Сохраняем результат этого шага в `mean_squared_error_value`.
        let mean_squared_error_value =
            part_049_lesson_09_mean_squared_error_between_targets_and_predictions::mean_squared_error_between_targets_and_predictions(&targets, predictions)
                // Используем результат, ожидая успешного выполнения шага.
                .expect("у каждого прогноза есть правильный ответ");
        // Проверяем ожидаемое свойство учебного примера.
        assert!((mean_squared_error_value - expected).abs() < 1e-10);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: {predictions:?} → MSE {mean_squared_error_value:.3}");
    }
    // Сохраняем результат этого шага в `error`.
    let error = part_049_lesson_09_mean_squared_error_between_targets_and_predictions::mean_squared_error_between_targets_and_predictions(&targets, &[2.0, 4.0])
        // Настраиваем или преобразуем результат предыдущего шага.
        .expect_err("длины должны совпадать");
    // Печатаем рассчитанные значения для проверки примера.
    println!("разная длина: {error}");

    // Построение графика вынесено из основного кода урока.
    visualize_mean_squared_error_between_targets_and_predictions();
}

// Строим график по результатам урока.
fn visualize_mean_squared_error_between_targets_and_predictions() {
    // График величин и зависимостей, изученных в этом уроке.
    let mean_squared_error_points: Vec<(f64, f64)> = (-30..=30)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `prediction_difference`.
            let prediction_difference = plot_step_index as f64 / 10.0;
            (
                // Используем подготовленное значение в следующем шаге примера.
                prediction_difference,
                // Используем подготовленное значение в следующем шаге примера.
                part_049_lesson_09_mean_squared_error_between_targets_and_predictions::mean_squared_error_between_targets_and_predictions(
                    // Передаём ряды или значения для отрисовки графика.
                    &[2.0, 4.0, 6.0],
                    // Передаём ряды или значения для отрисовки графика.
                    &[
                        2.0 + prediction_difference,
                        4.0 + prediction_difference,
                        6.0 + prediction_difference,
                    ],
                )
                // Используем результат, ожидая успешного выполнения шага.
                .unwrap(),
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Среднеквадратичная ошибка",
        // Указываем подпись горизонтальной оси.
        "смещение прогноза",
        // Указываем подпись вертикальной оси.
        "MSE",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "цели [2,4,6]",
            // Передаём рассчитанные координаты точек.
            points: &mean_squared_error_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
