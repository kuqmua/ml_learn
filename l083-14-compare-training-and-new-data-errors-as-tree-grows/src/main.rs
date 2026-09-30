// Урок 14.4. Сравнение ошибок на обучающих и новых данных при увеличении глубины дерева.
// Связь с принятой терминологией: Переобучение из-за чрезмерной глубины дерева решений.
// Зачем здесь эта тема: Дерево может довести train до идеальной чистоты, но запомнить шум вместо
//   закономерности.
// Почему код устроен так: Сравниваем разную глубину на обучающих и проверочных строках.
// Представь: Глубокое дерево может выделить отдельный лист на каждую шумную строку и провалиться на
//   новых данных.
//
// Что изучаем: Переобучение дерева.
// Зачем это нужно: Чем глубже дерево, тем легче запомнить отдельные обучающие точки. Сравниваем ошибку
// train и новых данных.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `training_error` для следующего шага примера.");
    let training_error: [f64; 3] = [0.25, 0.05, 0.0];
    trace_step!(training_error);
    trace_note!("Создаём набор значений `validation_error` для следующего шага примера.");
    let validation_error: [f64; 3] = [0.30, 0.15, 0.35];
    trace_step!(validation_error);
    trace_note!(
        "Сравниваем три заданные глубины: train error падает, но validation error после глубины 2 растёт."
    );
    for depth in 1..=3 {
        trace_step!(depth);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        trace_note!("Присваиваем вычисленное значение соответствующей переменной или полю.");
        trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
        trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
        println!(
            "глубина={depth}, train={}, validation={}",
            training_error[depth - 1],
            validation_error[depth - 1]
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_training_and_validation_errors_for_growing_tree_depth(training_error, validation_error);
}

// Строим график по результатам урока.
fn plot_training_and_validation_errors_for_growing_tree_depth(
    training_error: [f64; 3],
    validation_error: [f64; 3],
) {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let training_error_points: Vec<(f64, f64)> = training_error
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        .collect();
    trace_note!("Собираем значения для `validation_error_points` в коллекцию.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let validation_error_points: Vec<(f64, f64)> = validation_error
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Переобучение дерева",
        "глубина",
        "ошибка",
        &[
            lesson_visualization::Series {
                name: "train",

                points: &training_error_points,
            },
            lesson_visualization::Series {
                name: "validation",

                points: &validation_error_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
