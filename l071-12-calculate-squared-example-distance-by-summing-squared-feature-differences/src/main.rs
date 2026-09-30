// Урок 12.1. Квадрат расстояния между примерами: сложение квадратов разностей признаков.
// Связь с принятой терминологией: Квадрат расстояния по признакам для поиска ближайших соседей.
// Зачем здесь эта тема: kNN ищет похожие обучающие точки; сначала нужно определить сравнимое
//   расстояние.
// Почему код устроен так: Суммируем квадраты разностей признаков, пока корень не нужен для выбора
//   ближайшего.
// Представь: Чтобы выбрать ближайшую точку, сравнивать 9 и 16 достаточно; корни 3 и 4 не меняют
//   порядок.
//
// Что изучаем: Расстояния для ближайших соседей.
// Зачем это нужно: Сравниваем объекты по сумме квадратов разностей признаков; корень не нужен, если
// требуется только порядок соседей.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `query` для следующего шага примера.");
    let query: [f64; 2] = [1.0, 2.0];
    lesson_trace::trace_step!(query);
    lesson_trace::trace_note!("Создаём набор значений `candidates` для следующего шага примера.");
    let candidates: [[f64; 2]; 2] = [[2.0, 2.0], [4.0, 6.0]];
    lesson_trace::trace_step!(candidates);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for candidate in candidates {
        lesson_trace::trace_step!(candidate);
        lesson_trace::trace_note!(
            "Для поиска ближайшего кандидата нужен квадрат расстояния из урока 01.4."
        );
        lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let squared_distance: f64 =
            l004_01_calculate_squared_point_distance_by_summing_squared_coordinate_differences::calculate_squared_point_distance_by_summing_squared_coordinate_differences(
                &query, &candidate,
            )

            .expect("запрос и кандидат имеют одинаковое число координат");
        lesson_trace::trace_step!(squared_distance);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("кандидат {candidate:?}: квадрат расстояния={squared_distance}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_distance_from_query_for_changing_coordinate();
}

// Строим график по результатам урока.
fn plot_distance_from_query_for_changing_coordinate() {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let distances_points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                horizontal_value,
                (horizontal_value * horizontal_value + 1.0).sqrt(),
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
        "Расстояние до запроса",
        "первая координата",
        "евклидово расстояние",
        &[lesson_visualization::Series {
            name: "запрос [0,0]",

            points: &distances_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
