// Урок 42.2. Сравнение прогнозных вероятностей с частотой события.
//
// Если модель сообщает 0.8 многим объектам, событие должно происходить примерно в 80% случаев.
// Сравниваем совпадение прогноза с наблюдаемой частотой и чрезмерную уверенность.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `observed_labels`.
    let observed_labels: [bool; 5] = [true, true, true, false, true];
    lesson_trace::trace_step!(observed_labels);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        // Используем подготовленное значение в следующем шаге примера.
        !observed_labels.is_empty(),
        // Передаём подпись или текстовое значение для следующего шага.
        "для частоты нужна хотя бы одна метка"
    );
    // Вычисляем `observed_frequency` по элементам исходной коллекции.
    let observed_frequency: f64 = observed_labels.iter().filter(|&&label| label).count() as f64
        // Используем подготовленное значение в следующем шаге примера.
        / observed_labels.len() as f64;
    lesson_trace::trace_step!(observed_frequency);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, predicted_probability, expected_gap) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("калиброванный прогноз", 0.8, 0.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("слишком уверенный", 1.0, 0.2),
        // Добавляем пару значений для сравнения или построения графика.
        ("недооценка", 0.6, 0.2),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(predicted_probability);
        lesson_trace::trace_step!(expected_gap);
        // Проверяем ожидаемое свойство учебного примера.
        assert!((0.0..=1.0).contains(&predicted_probability));
        // Сохраняем результат этого шага в `gap`.
        let gap: f64 = (predicted_probability - observed_frequency).abs();
        lesson_trace::trace_step!(gap);
        // Проверяем ожидаемое свойство учебного примера.
        assert!((gap - expected_gap).abs() < 1e-10);
        // Печатаем рассчитанные значения для проверки примера.
        println!(
            // Передаём подпись или текстовое значение для следующего шага.
            "{description}: прогноз={predicted_probability}, частота={observed_frequency}, разница={gap:.2}"
        );
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_compare_predicted_probabilities_with_observed_event_rates(observed_frequency);
}

// Строим график по результатам урока.
fn visualize_compare_predicted_probabilities_with_observed_event_rates(observed_frequency: f64) {
    // График величин и зависимостей, изученных в этом уроке.
    // Соответствие вероятностей модели реальным частотам называют calibration.
    let ideal_probability_frequency_points: Vec<(f64, f64)> = (0..=10)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `probability`.
            let probability: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (probability, probability)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `observed_probability_frequency_points` в коллекцию.
    let observed_probability_frequency_points: Vec<(f64, f64)> =
        // Задаём значения следующей строки или последовательности.
        [(0.0, observed_frequency), (1.0, observed_frequency)].to_vec();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Калибровка вероятностей",
        // Указываем подпись горизонтальной оси.
        "прогноз",
        // Указываем подпись вертикальной оси.
        "наблюдаемая частота",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "идеальная",
                // Передаём рассчитанные координаты точек.
                points: &ideal_probability_frequency_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "частота в примере",
                // Передаём рассчитанные координаты точек.
                points: &observed_probability_frequency_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
