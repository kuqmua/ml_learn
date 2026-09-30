// Урок 12.4. Подсчёт сравнений координат при поиске среди всех обучающих точек.
// Связь с принятой терминологией: Стоимость прогноза kNN при сравнении со всеми обучающими точками.
// Зачем здесь эта тема: kNN хранит обучающие точки и сравнивает с ними каждый новый запрос.
// Почему код устроен так: Подсчитываем число сравнений при росте train, чтобы увидеть цену простого
//   обучения.
// Представь: Для одного нового объекта kNN сравнивает его с каждой строкой train: при 1000 строках
//   это 1000 сравнений.
//
// Что изучаем: Стоимость прогноза kNN.
// Зачем это нужно: Простой kNN сравнивает запрос с каждой обучающей точкой: число вычислений расстояния
// растёт вместе с набором.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    for training_size in [10, 100, 1000] {
        trace_step!(training_size);
        trace_note!("Сохраняем рассчитанное значение `feature_count` для следующих операций.");
        let feature_count: i32 = 4;
        trace_step!(feature_count);
        trace_note!("Умножаем значения и сохраняем результат в `coordinate_comparisons`.");
        let coordinate_comparisons: i32 = training_size * feature_count;
        trace_step!(coordinate_comparisons);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        println!("объектов={training_size}, сравнений координат={coordinate_comparisons}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_coordinate_comparison_count_for_growing_training_set();
}

// Строим график по результатам урока.
fn plot_coordinate_comparison_count_for_growing_training_set() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let prediction_cost_points: Vec<(f64, f64)> = (1..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `sample_count`.");
            let sample_count: f64 = (plot_step_index * 10) as f64;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (sample_count, 2.0 * sample_count)
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
        "Стоимость kNN",
        "число обучающих объектов",
        "сравнения координат",
        &[lesson_visualization::Series {
            name: "4 признака",

            points: &prediction_cost_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
