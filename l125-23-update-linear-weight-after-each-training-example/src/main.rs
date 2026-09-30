// Урок 23.1. Обновление веса линейной модели после каждого обучающего примера.
// Связь с принятой терминологией: Обновление веса линейной модели после каждого примера методом SGD.
// Зачем здесь эта тема: Полный градиент дорог на большом наборе; SGD обновляет параметры после
//   отдельных строк.
// Почему код устроен так: Показываем порядок примеров и изменение веса после каждого, а не только
//   после эпохи.
// Представь: После первой строки вес изменился, поэтому вторая строка уже видит обновлённую модель.
//
// Что изучаем: Стохастический градиентный спуск.
// Зачем это нужно: SGD обновляет параметр после отдельного примера, поэтому шаги зависят от порядка
// обучающих объектов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `examples` для следующего шага примера.");
    let examples: [(f64, f64); 2] = [(1.0, 2.0), (2.0, 4.0)];
    trace_step!(examples);
    trace_note!("Инициализируем изменяемый накопитель `weight` начальным состоянием.");
    let mut weight: f64 = 0.0;
    trace_step!(weight);
    trace_note!("Собираем значения для `weight_history` в коллекцию.");
    let mut weight_history: Vec<(f64, f64)> = vec![(0.0, weight)];
    trace_step!(weight_history);
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    for (step, (feature, target)) in examples.into_iter().enumerate() {
        trace_step!(step);
        trace_step!(feature);
        trace_step!(target);
        trace_note!("Умножаем значения и сохраняем результат в `rate_of_change`.");
        trace_note!(
            "Производную функции по параметру или вектор таких производных называют gradient."
        );
        let rate_of_change: f64 = 2.0 * (weight * feature - target) * feature;
        trace_step!(rate_of_change);
        trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
        weight -= 0.1 * rate_of_change;
        trace_step!(weight);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        println!("после x={feature}: вес={weight}");
        trace_note!("Вычисляем значение по указанной формуле.");
        weight_history.push(((step + 1) as f64, weight));
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_weight_after_each_single_example_update(weight_history);
}

// Строим график по результатам урока.
fn plot_weight_after_each_single_example_update(weight_history: std::vec::Vec<(f64, f64)>) {
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
        "Шаги SGD",
        "шаг",
        "вес",
        &[lesson_visualization::Series {
            name: "вес",

            points: &weight_history,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
