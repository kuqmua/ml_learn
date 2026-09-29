// Урок 11.1. Подсчёт верных и ошибочных положительных и отрицательных прогнозов.
// Связь с принятой терминологией: Матрица ошибок бинарной классификации по истинным и прогнозным меткам.
// Зачем здесь эта тема: Одного accuracy мало: нужно различать четыре исхода бинарного прогноза.
// Почему код устроен так: Считаем TP, FP, TN и FN по парам меток, прежде чем выводить следующие
//   метрики.
// Представь: Из 100 верных ответов можно не заметить, что модель пропускает почти все редкие
//   положительные случаи.
//
// Каждая пара «истина, прогноз» попадает ровно в одну из четырёх ячеек.
// Эти счётчики затем повторно используются в precision, recall, F1 и сводной практике.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `truth`.
    let truth: [bool; 4] = [true, false, true, false];
    lesson_trace::trace_step!(truth);
    // Задаём учебные значения для `predicted`.
    let predicted: [bool; 4] = [true, true, false, false];
    lesson_trace::trace_step!(predicted);
    // Сохраняем результат этого шага в `counts`.
    let counts: part_060_lesson_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts = part_060_lesson_11_count_correct_and_incorrect_positive_and_negative_predictions::count_binary_classification_outcomes_from_true_and_predicted_labels(
        &truth, &predicted,
    )
    // Используем результат, ожидая успешного выполнения шага.
    .expect("у каждого ответа есть прогноз");
    lesson_trace::trace_step!(counts);
    // Повторяем расчёт для каждого элемента последовательности.
    for index in 0..truth.len() {
        lesson_trace::trace_step!(index);
        // Сохраняем результат этого шага в `description`.
        let description: &str = match (truth[index], predicted[index]) {
            // Выполняем действие для этого варианта данных.
            (true, true) => "TP: верно найден положительный класс",
            // Выполняем действие для этого варианта данных.
            (false, true) => "FP: ложная тревога",
            // Выполняем действие для этого варианта данных.
            (true, false) => "FN: положительный класс пропущен",
            // Выполняем действие для этого варианта данных.
            (false, false) => "TN: верно найден отрицательный класс",
        };
        lesson_trace::trace_step!(description);
        // Печатаем рассчитанные значения для проверки примера.
        println!(
            // Передаём подпись или текстовое значение для следующего шага.
            "истина={}, прогноз={} → {description}",
            // Используем подготовленное значение в следующем шаге примера.
            truth[index],
            // Печатаем прогноз для того же объекта.
            predicted[index]
        );
    }
    // Проверяем ожидаемое свойство учебного примера.
    assert_eq!(
        (
            // Используем подготовленное значение в следующем шаге примера.
            counts.true_positives,
            // Используем подготовленное значение в следующем шаге примера.
            counts.false_positives,
            // Используем подготовленное значение в следующем шаге примера.
            counts.false_negatives,
            // Используем подготовленное значение в следующем шаге примера.
            counts.true_negatives
        ),
        // Добавляем пару значений для сравнения или построения графика.
        (1, 1, 1, 1)
    );
    // Печатаем рассчитанные значения для проверки примера.
    println!("итоговые счётчики: {counts:?}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_counts_of_correct_and_incorrect_class_predictions(counts);
}

// Строим график по результатам урока.
fn plot_counts_of_correct_and_incorrect_class_predictions(
    counts: part_060_lesson_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts,
) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Матрица ошибок: исходы",
        // Указываем подпись вертикальной оси.
        "количество",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("TP", counts.true_positives as f64),
            // Добавляем пару значений для сравнения или построения графика.
            ("FP", counts.false_positives as f64),
            // Добавляем пару значений для сравнения или построения графика.
            ("TN", counts.true_negatives as f64),
            // Добавляем пару значений для сравнения или построения графика.
            ("FN", counts.false_negatives as f64),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
