// Урок 11.3. Полнота обнаружения: доля найденных среди всех действительно положительных примеров.
// Связь с принятой терминологией: Полнота положительного класса по счётчикам бинарной классификации.
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
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, true_positives, false_negatives, expected) in [
        ("найдены все", 8, 0, Some(1.0)),
        ("найдены не все", 8, 4, Some(2.0 / 3.0)),
        ("не найден ни один", 0, 4, Some(0.0)),
        ("положительных объектов нет", 0, 0, None),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(true_positives);
        lesson_trace::trace_step!(false_negatives);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `counts`.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Задаём именованное поле или параметр.");
        lesson_trace::trace_note!("Задаём именованное поле или параметр.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        let counts: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts = l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {

            true_positives,

            false_positives: 0,

            true_negatives: 0,

            false_negatives,
        };
        lesson_trace::trace_step!(counts);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `recall`.");
        let recall: Option<f64> = l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts);
        lesson_trace::trace_step!(recall);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(recall, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: recall={recall:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_detected_share_of_actual_positive_examples();
}

// Строим график по результатам урока.
fn plot_detected_share_of_actual_positive_examples() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let recall_points: Vec<(f64, f64)> = (0..=10)
        .map(|false_negative_count| {
            (
                false_negative_count as f64,
                2.0 / (2.0 + false_negative_count as f64),
            )
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Recall при фиксированном TP=2",
        "FN",
        "recall",
        &[lesson_visualization::Series {
            name: "recall",

            points: &recall_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
