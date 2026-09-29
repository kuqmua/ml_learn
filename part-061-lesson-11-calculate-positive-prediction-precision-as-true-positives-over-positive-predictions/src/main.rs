// Урок 11.2. Точность положительных прогнозов: доля верных среди всех положительных прогнозов.
// Связь с принятой терминологией: Точность положительных прогнозов по счётчикам бинарной классификации.
// Зачем здесь эта тема: Когда ложные тревоги дороги, важно знать долю верных среди положительных
//   прогнозов.
// Почему код устроен так: Делим TP на TP+FP и отдельно обрабатываем отсутствие положительных
//   прогнозов.
// Представь: Из 10 положительных прогнозов 7 верных: precision равен 7/10, независимо от
//   пропущенных случаев.
//
// Используем четыре счётчика из урока 11.1. Если положительных прогнозов нет,
// значение здесь считаем неопределённым.

fn main() {
    lesson_trace::enable();
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, true_positives, false_positives, expected) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("все положительные прогнозы верны", 8, 0, Some(1.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("часть прогнозов ошибочна", 8, 2, Some(0.8)),
        // Добавляем пару значений для сравнения или построения графика.
        ("все положительные прогнозы ошибочны", 0, 2, Some(0.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("положительных прогнозов нет", 0, 0, None),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(true_positives);
        lesson_trace::trace_step!(false_positives);
        lesson_trace::trace_step!(expected);
        // Сохраняем результат этого шага в `counts`.
        let counts: part_060_lesson_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts = part_060_lesson_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {
            // Используем подготовленное значение в следующем шаге примера.
            true_positives,
            // Используем подготовленное значение в следующем шаге примера.
            false_positives,
            // Задаём именованное поле или параметр.
            true_negatives: 0,
            // Задаём именованное поле или параметр.
            false_negatives: 0,
        };
        lesson_trace::trace_step!(counts);
        // Сохраняем результат этого шага в `precision`.
        let precision: Option<f64> =
            part_061_lesson_11_calculate_positive_prediction_precision_as_true_positives_over_positive_predictions::calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(counts);
        lesson_trace::trace_step!(precision);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(precision, expected);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: precision={precision:?}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_true_positive_share_among_positive_predictions();
}

// Строим график по результатам урока.
fn plot_true_positive_share_among_positive_predictions() {
    // График величин и зависимостей, изученных в этом уроке.
    let precision_points: Vec<(f64, f64)> = (0..=10)
        // Преобразуем каждый элемент в новое значение.
        .map(|false_positive_count| {
            (
                false_positive_count as f64,
                2.0 / (2.0 + false_positive_count as f64),
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
        "Precision при фиксированном TP=2",
        // Указываем подпись горизонтальной оси.
        "FP",
        // Указываем подпись вертикальной оси.
        "precision",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "precision",
            // Передаём рассчитанные координаты точек.
            points: &precision_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
