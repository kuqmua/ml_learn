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

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, true_positives, false_negatives, expected) in [
        ("найдены все", 8, 0, Some(1.0)),
        ("найдены не все", 8, 4, Some(2.0 / 3.0)),
        ("не найден ни один", 0, 4, Some(0.0)),
        ("положительных объектов нет", 0, 0, None),
    ] {
        trace_step!(description);
        trace_step!(true_positives);
        trace_step!(false_negatives);
        trace_step!(expected);
        trace_note!("Сохраняем результат этого шага в `counts`.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Задаём именованное поле или параметр.");
        trace_note!("Задаём именованное поле или параметр.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        let counts: l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts = l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts {

            true_positives,

            false_positives: 0,

            true_negatives: 0,

            false_negatives,
        };
        trace_step!(counts);
        trace_note!("Сохраняем результат этого шага в `recall`.");
        let recall: Option<f64> = l065_11_calculate_positive_detection_recall_as_found_positives_over_actual_positives::calculate_positive_detection_recall_as_true_positives_divided_by_actual_positives(counts);
        trace_step!(recall);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(recall, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: recall={recall:?}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_detected_share_of_actual_positive_examples();
}

// Строим график по результатам урока.
fn plot_detected_share_of_actual_positive_examples() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let recall_points: Vec<(f64, f64)> = (0..=10)
        .map(|false_negative_count| {
            (
                false_negative_count as f64,
                2.0 / (2.0 + false_negative_count as f64),
            )
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
