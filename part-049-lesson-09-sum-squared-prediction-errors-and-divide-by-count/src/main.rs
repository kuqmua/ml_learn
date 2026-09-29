// Урок 09.1. Сумма квадратов ошибок прогноза, делённая на число примеров.
// Связь с принятой терминологией: Средний квадрат разности правильных ответов и прогнозов.
// Зачем здесь эта тема: Регрессии нужна мера ошибки; квадрат сильнее штрафует крупные промахи.
// Почему код устроен так: Считаем разности прогнозов и целей поэлементно, затем усредняем квадраты.
// Представь: Промах на 3 даёт квадрат ошибки 9, а промах на 1 — только 1.
//
// При точном прогнозе MSE равна нулю. Ошибка вдвое больше даёт вклад вчетверо больше.
// Та же общая функция будет использоваться для оценки моделей в следующих уроках.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `targets`.
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    lesson_trace::trace_step!(targets);
    // Задаём учебные значения для `cases`.
    let cases: [(&str, &[f64], f64); 3] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    lesson_trace::trace_step!(cases);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, predictions, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(predictions);
        lesson_trace::trace_step!(expected);
        // Сохраняем результат этого шага в `mean_squared_error_value`.
        let mean_squared_error_value: f64 =
            part_049_lesson_09_sum_squared_prediction_errors_and_divide_by_count::sum_squared_prediction_errors_and_divide_by_count(&targets, predictions)
                // Используем результат, ожидая успешного выполнения шага.
                .expect("у каждого прогноза есть правильный ответ");
        lesson_trace::trace_step!(mean_squared_error_value);
        // Проверяем ожидаемое свойство учебного примера.
        assert!((mean_squared_error_value - expected).abs() < 1e-10);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: {predictions:?} → MSE {mean_squared_error_value:.3}");
    }
    // Сохраняем результат этого шага в `error`.
    let error: &str = part_049_lesson_09_sum_squared_prediction_errors_and_divide_by_count::sum_squared_prediction_errors_and_divide_by_count(&targets, &[2.0, 4.0])
        // Настраиваем или преобразуем результат предыдущего шага.
        .expect_err("длины должны совпадать");
    lesson_trace::trace_step!(error);
    // Печатаем рассчитанные значения для проверки примера.
    println!("разная длина: {error}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_average_squared_prediction_error_for_changing_offset();
}

// Строим график по результатам урока.
fn plot_average_squared_prediction_error_for_changing_offset() {
    // График величин и зависимостей, изученных в этом уроке.
    let mean_squared_error_points: Vec<(f64, f64)> = (-30..=30)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `prediction_difference`.
            let prediction_difference: f64 = plot_step_index as f64 / 10.0;
            (
                // Используем подготовленное значение в следующем шаге примера.
                prediction_difference,
                // Используем подготовленное значение в следующем шаге примера.
                part_049_lesson_09_sum_squared_prediction_errors_and_divide_by_count::sum_squared_prediction_errors_and_divide_by_count(
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
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
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
