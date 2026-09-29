// Урок 11.2. Точность положительных прогнозов по счётчикам бинарной классификации.
//
// Используем четыре счётчика из урока 11.1. Если положительных прогнозов нет,
// значение здесь считаем неопределённым.

fn main() {
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
        // Сохраняем результат этого шага в `counts`.
        let counts: part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::BinaryClassificationCounts = part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::BinaryClassificationCounts {
            // Используем подготовленное значение в следующем шаге примера.
            true_positives,
            // Используем подготовленное значение в следующем шаге примера.
            false_positives,
            // Задаём именованное поле или параметр.
            true_negatives: 0,
            // Задаём именованное поле или параметр.
            false_negatives: 0,
        };
        // Сохраняем результат этого шага в `precision`.
        let precision: Option<f64> =
            part_061_lesson_11_precision_from_binary_classification_counts::precision_from_binary_classification_counts(counts);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(precision, expected);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: precision={precision:?}");
    }

    // Построение графика вынесено из основного кода урока.
    visualize_precision_from_binary_classification_counts();
}

// Строим график по результатам урока.
fn visualize_precision_from_binary_classification_counts() {
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
