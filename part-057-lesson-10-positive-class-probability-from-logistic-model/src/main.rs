// Урок 10.3. Вероятность положительного класса из логистической модели.
//
// Что изучаем: Вероятность класса.
// Зачем это нужно: Вероятность класса описывает уверенность модели и позволяет менять решение без
// повторного обучения.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Инициализируем значение `probability_positive` начальным состоянием.
    let probability_positive: f64 = 0.7;
    lesson_trace::trace_step!(probability_positive);
    // Комбинируем исходные величины и сохраняем результат в `probability_negative`.
    let probability_negative: f64 = 1.0 - probability_positive;
    lesson_trace::trace_step!(probability_negative);
    // Проверяем обязательное условие до дальнейшего вычисления.
    assert!(probability_positive >= 0.0 && probability_positive <= 1.0);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("P(y=1)={probability_positive}, P(y=0)={probability_negative}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_positive_class_probability_from_logistic_model();
}

// Строим график по результатам урока.
fn visualize_positive_class_probability_from_logistic_model() {
    // График величин и зависимостей, изученных в этом уроке.
    let positive_class_probability_points: Vec<(f64, f64)> = (0..=100)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `probability`.
            let probability: f64 = plot_step_index as f64 / 100.0;
            // Добавляем пару значений для сравнения или построения графика.
            (probability, probability)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `negative_class_probability_points` в коллекцию.
    let negative_class_probability_points: Vec<(f64, f64)> = (0..=100)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `probability`.
            let probability: f64 = plot_step_index as f64 / 100.0;
            // Добавляем пару значений для сравнения или построения графика.
            (probability, 1.0 - probability)
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
        "Вероятности двух классов",
        // Указываем подпись горизонтальной оси.
        "P(y=1)",
        // Указываем подпись вертикальной оси.
        "вероятность",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "положительный",
                // Передаём рассчитанные координаты точек.
                points: &positive_class_probability_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "отрицательный",
                // Передаём рассчитанные координаты точек.
                points: &negative_class_probability_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
