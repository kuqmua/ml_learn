// Урок 11.3. Полнота положительного класса по счётчикам бинарной классификации.
// Зачем здесь эта тема: Когда пропуск события дорог, важно знать, какую долю истинно положительных
//   нашли.
// Почему код устроен так: Делим TP на TP+FN; знаменатель здесь зависит от истинных меток, а не
//   прогнозов.
// Представь: Из 10 реальных положительных случаев нашли 7: recall равен 7/10, независимо от ложных
//   тревог.
//
// Используем те же четыре счётчика. Если положительных объектов нет,
// значение здесь считаем неопределённым.

fn main() {
    lesson_trace::enable();
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, true_positives, false_negatives, expected) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("найдены все", 8, 0, Some(1.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("найдены не все", 8, 4, Some(2.0 / 3.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("не найден ни один", 0, 4, Some(0.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("положительных объектов нет", 0, 0, None),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(true_positives);
        lesson_trace::trace_step!(false_negatives);
        lesson_trace::trace_step!(expected);
        // Сохраняем результат этого шага в `counts`.
        let counts: part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::BinaryClassificationCounts = part_060_lesson_11_binary_classification_confusion_matrix_from_true_and_predicted_labels::BinaryClassificationCounts {
            // Используем подготовленное значение в следующем шаге примера.
            true_positives,
            // Задаём именованное поле или параметр.
            false_positives: 0,
            // Задаём именованное поле или параметр.
            true_negatives: 0,
            // Используем подготовленное значение в следующем шаге примера.
            false_negatives,
        };
        lesson_trace::trace_step!(counts);
        // Сохраняем результат этого шага в `recall`.
        let recall: Option<f64> = part_062_lesson_11_recall_from_binary_classification_counts::recall_from_binary_classification_counts(counts);
        lesson_trace::trace_step!(recall);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(recall, expected);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: recall={recall:?}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_recall_from_binary_classification_counts();
}

// Строим график по результатам урока.
fn visualize_recall_from_binary_classification_counts() {
    // График величин и зависимостей, изученных в этом уроке.
    let recall_points: Vec<(f64, f64)> = (0..=10)
        // Преобразуем каждый элемент в новое значение.
        .map(|false_negative_count| {
            (
                false_negative_count as f64,
                2.0 / (2.0 + false_negative_count as f64),
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
        "Recall при фиксированном TP=2",
        // Указываем подпись горизонтальной оси.
        "FN",
        // Указываем подпись вертикальной оси.
        "recall",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "recall",
            // Передаём рассчитанные координаты точек.
            points: &recall_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
