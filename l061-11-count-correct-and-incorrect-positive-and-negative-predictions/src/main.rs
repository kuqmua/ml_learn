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

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::{
    BinaryClassificationCounts, count_binary_classification_outcomes_from_true_and_predicted_labels,
};

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `truth`.");
    let truth: [bool; 4] = [true, false, true, false];
    trace_step!(truth);
    trace_note!("Задаём учебные значения для `predicted`.");
    let predicted: [bool; 4] = [true, true, false, false];
    trace_step!(predicted);
    trace_note!("Сохраняем результат этого шага в `counts`.");
    trace_note!("Используем результат, ожидая успешного выполнения шага.");
    let counts: BinaryClassificationCounts =
        count_binary_classification_outcomes_from_true_and_predicted_labels(&truth, &predicted)
            .expect("у каждого ответа есть прогноз");
    trace_step!(counts);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for index in 0..truth.len() {
        trace_step!(index);
        trace_note!("Сохраняем результат этого шага в `description`.");
        trace_note!("Выполняем действие для этого варианта данных.");
        trace_note!("Выполняем действие для этого варианта данных.");
        trace_note!("Выполняем действие для этого варианта данных.");
        trace_note!("Выполняем действие для этого варианта данных.");
        let description: &str = match (truth[index], predicted[index]) {
            (true, true) => "TP: верно найден положительный класс",

            (false, true) => "FP: ложная тревога",

            (true, false) => "FN: положительный класс пропущен",

            (false, false) => "TN: верно найден отрицательный класс",
        };
        trace_step!(description);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Печатаем прогноз для того же объекта.");
        println!(
            "истина={}, прогноз={} → {description}",
            truth[index], predicted[index]
        );
    }
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    assert_eq!(
        (
            counts.true_positives,
            counts.false_positives,
            counts.false_negatives,
            counts.true_negatives
        ),
        (1, 1, 1, 1)
    );
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("итоговые счётчики: {counts:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_counts_of_correct_and_incorrect_class_predictions(counts);
}

// Строим график по результатам урока.
fn plot_counts_of_correct_and_incorrect_class_predictions(counts: BinaryClassificationCounts) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Матрица ошибок: исходы",
        "количество",
        &[
            ("TP", counts.true_positives as f64),
            ("FP", counts.false_positives as f64),
            ("TN", counts.true_negatives as f64),
            ("FN", counts.false_negatives as f64),
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
