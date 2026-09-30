// Урок 12.3. Выбор числа ближайших примеров, голосующих за класс.
// Связь с принятой терминологией: Выбор числа соседей для классификации kNN.
// Зачем здесь эта тема: Один сосед чувствителен к шуму, а слишком большое k размывает локальную
//   структуру.
// Почему код устроен так: Сравниваем голосование нескольких ближайших точек при разных k.
// Представь: Один сосед может ошибаться из-за шума; голоса трёх соседей могут изменить решение.
//
// Что изучаем: Выбор числа соседей k.
// Зачем это нужно: Малое k делает решение чувствительным к одной точке, большое сглаживает локальные
// различия.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Соседи отсортированы от ближайшего к дальнему.");
    let neighbor_labels: [bool; 5] = [true, false, false, true, true];
    trace_step!(neighbor_labels);
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    for neighbor_count in [1, 3, 5] {
        trace_step!(neighbor_count);
        trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `positive`.");
        let positive: usize = neighbor_labels[..neighbor_count]
            .iter()
            .filter(|&&label| label)
            .count();
        trace_step!(positive);
        trace_note!("Умножаем значения и сохраняем результат в `prediction`.");
        let prediction: bool = positive * 2 > neighbor_count;
        trace_step!(prediction);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        println!("k={neighbor_count}: прогноз={prediction}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_positive_class_share_among_nearest_examples(neighbor_labels);
}

// Строим график по результатам урока.
fn plot_positive_class_share_among_nearest_examples(neighbor_labels: [bool; 5]) {
    trace_note!("Показываем значения, рассчитанные по данным примера.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let positive_neighbor_count_points: Vec<(f64, f64)> = [1usize, 3, 5]
        .iter()
        .map(|&neighbor_count| {
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Вычисляем значение по указанной формуле.");
            (
                neighbor_count as f64,
                neighbor_labels[..neighbor_count]
                    .iter()
                    .filter(|&&element_value| element_value)
                    .count() as f64
                    / neighbor_count as f64,
            )
        })
        .collect();
    trace_note!("Собираем значения для `decision_boundary_points` в коллекцию.");
    let decision_boundary_points: Vec<(f64, f64)> = [(1.0, 0.5), (5.0, 0.5)].to_vec();
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
        "Соседи и доля положительных",
        "k",
        "доля",
        &[
            lesson_visualization::Series {
                name: "положительные среди k",

                points: &positive_neighbor_count_points,
            },
            lesson_visualization::Series {
                name: "граница решения",

                points: &decision_boundary_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
