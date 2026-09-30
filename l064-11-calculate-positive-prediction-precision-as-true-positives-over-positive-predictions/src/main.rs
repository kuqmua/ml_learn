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

use l061_11_count_correct_and_incorrect_positive_and_negative_predictions::BinaryClassificationCounts;
use l064_11_calculate_positive_prediction_precision_as_true_positives_over_positive_predictions::calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions;

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, true_positives, false_positives, expected) in [
        ("все положительные прогнозы верны", 8, 0, Some(1.0)),
        ("часть прогнозов ошибочна", 8, 2, Some(0.8)),
        ("все положительные прогнозы ошибочны", 0, 2, Some(0.0)),
        ("положительных прогнозов нет", 0, 0, None),
    ] {
        trace_step!(description);
        trace_step!(true_positives);
        trace_step!(false_positives);
        trace_step!(expected);
        trace_note!("Сохраняем результат этого шага в `counts`.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Задаём именованное поле или параметр.");
        trace_note!("Задаём именованное поле или параметр.");
        let counts: BinaryClassificationCounts = BinaryClassificationCounts {
            true_positives,

            false_positives,

            true_negatives: 0,

            false_negatives: 0,
        };
        trace_step!(counts);
        trace_note!("Сохраняем результат этого шага в `precision`.");
        let precision: Option<f64> =
            calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions(counts);
        trace_step!(precision);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(precision, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: precision={precision:?}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_true_positive_share_among_positive_predictions();
}

// Строим график по результатам урока.
fn plot_true_positive_share_among_positive_predictions() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let precision_points: Vec<(f64, f64)> = (0..=10)
        .map(|false_positive_count| {
            (
                false_positive_count as f64,
                2.0 / (2.0 + false_positive_count as f64),
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
        "Precision при фиксированном TP=2",
        "FP",
        "precision",
        &[lesson_visualization::Series {
            name: "precision",

            points: &precision_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
