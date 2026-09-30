// Урок 18.2. Выбор начальных центров для объединения близких точек в группы.
// Связь с принятой терминологией: Выбор начальных центроидов для кластеризации k-means.
// Зачем здесь эта тема: k-means требует начальных центров до первого назначения точек; от них может
//   зависеть результат.
// Почему код устроен так: Сравниваем варианты начального выбора на малых данных.
// Представь: Если поставить два начальных центра рядом, алгоритму сложнее сразу разделить две
//   далёкие группы.
//
// Что изучаем: Инициализация k-means.
// Зачем это нужно: Начальные центры задают старт итераций и могут менять итоговое разбиение, поэтому их
// нужно фиксировать.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `points` для следующего шага примера.");
    let points: [[f64; 2]; 4] = [[0.0, 0.0], [0.1, 0.0], [5.0, 5.0], [5.1, 5.0]];
    lesson_trace::trace_step!(points);
    lesson_trace::trace_note!("Создаём набор значений `first_start` для следующего шага примера.");
    let first_start: [[f64; 2]; 2] = [points[0], points[2]];
    lesson_trace::trace_step!(first_start);
    lesson_trace::trace_note!("Создаём набор значений `second_start` для следующего шага примера.");
    let second_start: [[f64; 2]; 2] = [points[0], points[1]];
    lesson_trace::trace_step!(second_start);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("разнесённые центры={first_start:?}; соседние центры={second_start:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_initial_cluster_centers(points, first_start, second_start);
}

// Строим график по результатам урока.
fn plot_initial_cluster_centers(
    points: [[f64; 2]; 4],
    first_start: [[f64; 2]; 2],
    second_start: [[f64; 2]; 2],
) {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    let observation_points: Vec<(f64, f64)> = points
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    lesson_trace::trace_note!(
        "Собираем значения для `separated_cluster_center_points` в коллекцию."
    );
    lesson_trace::trace_note!("Центр группы точек называют centroid.");
    let separated_cluster_center_points: Vec<(f64, f64)> = first_start
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    lesson_trace::trace_note!("Собираем значения для `nearby_cluster_center_points` в коллекцию.");
    let nearby_cluster_center_points: Vec<(f64, f64)> = second_start
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Инициализация k-means",
        "x",
        "y",
        &[
            lesson_visualization::Series {
                name: "объекты",

                points: &observation_points,
            },
            lesson_visualization::Series {
                name: "разнесённые центры",

                points: &separated_cluster_center_points,
            },
            lesson_visualization::Series {
                name: "соседние центры",

                points: &nearby_cluster_center_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
