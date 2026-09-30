// Урок 42.2. Проверка вероятностей: сравнение прогнозов с наблюдаемой частотой событий.
// Связь с принятой терминологией: Сравнение прогнозных вероятностей с частотой события.
// Зачем здесь эта тема: Вероятность 0,8 полезна лишь если среди таких прогнозов событие происходит
//   примерно в 80% случаев.
// Почему код устроен так: Группируем прогнозы по интервалам и сравниваем среднюю вероятность с
//   частотой метки.
// Представь: Среди прогнозов около 0,8 событие должно встречаться примерно в 8 случаях из 10.
//
// Если модель сообщает 0.8 многим объектам, событие должно происходить примерно в 80% случаев.
// Сравниваем совпадение прогноза с наблюдаемой частотой и чрезмерную уверенность.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `observed_labels`.");
    let observed_labels: [bool; 5] = [true, true, true, false, true];
    lesson_trace::trace_step!(observed_labels);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !observed_labels.is_empty(),
        "для частоты нужна хотя бы одна метка"
    );
    lesson_trace::trace_note!("Вычисляем `observed_frequency` по элементам исходной коллекции.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    let observed_frequency: f64 = observed_labels.iter().filter(|&&label| label).count() as f64
        / observed_labels.len() as f64;
    lesson_trace::trace_step!(observed_frequency);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, predicted_probability, expected_gap) in [
        ("калиброванный прогноз", 0.8, 0.0),
        ("слишком уверенный", 1.0, 0.2),
        ("недооценка", 0.6, 0.2),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(predicted_probability);
        lesson_trace::trace_step!(expected_gap);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((0.0..=1.0).contains(&predicted_probability));
        lesson_trace::trace_note!("Сохраняем результат этого шага в `gap`.");
        let gap: f64 = (predicted_probability - observed_frequency).abs();
        lesson_trace::trace_step!(gap);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((gap - expected_gap).abs() < 1e-10);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        println!(
            "{description}: прогноз={predicted_probability}, частота={observed_frequency}, разница={gap:.2}"
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_predicted_probabilities_and_observed_event_frequencies(observed_frequency);
}

// Строим график по результатам урока.
fn plot_predicted_probabilities_and_observed_event_frequencies(observed_frequency: f64) {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!(
        "Соответствие вероятностей модели реальным частотам называют calibration."
    );
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let ideal_probability_frequency_points: Vec<(f64, f64)> = (0..=10)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (probability, probability)
        })
        .collect();
    lesson_trace::trace_note!(
        "Собираем значения для `observed_probability_frequency_points` в коллекцию."
    );
    lesson_trace::trace_note!("Задаём значения следующей строки или последовательности.");
    let observed_probability_frequency_points: Vec<(f64, f64)> =
        [(0.0, observed_frequency), (1.0, observed_frequency)].to_vec();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Калибровка вероятностей",
        "прогноз",
        "наблюдаемая частота",
        &[
            lesson_visualization::Series {
                name: "идеальная",

                points: &ideal_probability_frequency_points,
            },
            lesson_visualization::Series {
                name: "частота в примере",

                points: &observed_probability_frequency_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
