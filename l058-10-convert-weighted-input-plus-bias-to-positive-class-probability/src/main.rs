// Урок 10.3. Преобразование взвешенного входа со смещением в вероятность положительного класса.
// Связь с принятой терминологией: Вероятность положительного класса из логистической модели.
// Зачем здесь эта тема: Логистическая модель связывает признаки с вероятностью через линейный логит
//   и сигмоиду.
// Почему код устроен так: Показываем оба шага отдельно, чтобы вес признака не смешивался с порогом
//   решения.
// Представь: Вес превращает признак в логит, сигмоида — логит в вероятность; это два разных шага.
//
// Что изучаем: Вероятность класса.
// Зачем это нужно: Вероятность класса описывает уверенность модели и позволяет менять решение без
// повторного обучения.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Инициализируем значение `probability_positive` начальным состоянием."
    );
    let probability_positive: f64 = 0.7;
    lesson_trace::trace_step!(probability_positive);
    lesson_trace::trace_note!(
        "Комбинируем исходные величины и сохраняем результат в `probability_negative`."
    );
    let probability_negative: f64 = 1.0 - probability_positive;
    lesson_trace::trace_step!(probability_negative);
    lesson_trace::trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
    assert!(probability_positive >= 0.0 && probability_positive <= 1.0);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("P(y=1)={probability_positive}, P(y=0)={probability_negative}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_positive_and_negative_class_probabilities();
}

// Строим график по результатам урока.
fn plot_positive_and_negative_class_probabilities() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let positive_class_probability_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (probability, probability)
        })
        .collect();
    lesson_trace::trace_note!(
        "Собираем значения для `negative_class_probability_points` в коллекцию."
    );
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let negative_class_probability_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (probability, 1.0 - probability)
        })
        .collect();
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
        "Вероятности двух классов",
        "P(y=1)",
        "вероятность",
        &[
            lesson_visualization::Series {
                name: "положительный",

                points: &positive_class_probability_points,
            },
            lesson_visualization::Series {
                name: "отрицательный",

                points: &negative_class_probability_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
