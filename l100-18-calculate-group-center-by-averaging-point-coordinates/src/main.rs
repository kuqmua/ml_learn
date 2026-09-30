// Урок 18.1. Центр группы точек: усреднение каждой координаты.
// Связь с принятой терминологией: Вычисление центроида как среднего точек кластера.
// Зачем здесь эта тема: Кластеризация группирует точки без меток; центроид задаёт текущее
//   представление группы.
// Почему код устроен так: Усредняем координаты назначенных точек по отдельности, чтобы увидеть
//   смысл центра.
// Представь: Для точек [0, 0] и [2, 2] центроид находится посередине: [1, 1].
//
// Что изучаем: Центроиды кластеров.
// Зачем это нужно: Центроид — покоординатное среднее точек одной группы. Он представляет центр кластера.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `cluster` для следующего шага примера.");
    let cluster: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    lesson_trace::trace_step!(cluster);
    lesson_trace::trace_note!(
        "Создаём набор значений `cluster_center` для следующего шага примера."
    );
    lesson_trace::trace_note!("Среднее пустого кластера не определено.");
    assert!(!cluster.is_empty(), "для центра нужна хотя бы одна точка");
    lesson_trace::trace_note!("Задаём учебные значения для `cluster_center`.");
    lesson_trace::trace_note!("Центр группы точек называют centroid.");
    let mut cluster_center: [f64; 2] = [0.0, 0.0];
    lesson_trace::trace_step!(cluster_center);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for point in cluster {
        lesson_trace::trace_step!(point);
        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        cluster_center[0] += point[0];
        lesson_trace::trace_step!(cluster_center);
        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        cluster_center[1] += point[1];
        lesson_trace::trace_step!(cluster_center);
    }
    lesson_trace::trace_note!("Масштабируем текущую величину делением.");
    cluster_center[0] /= cluster.len() as f64;
    lesson_trace::trace_step!(cluster_center);
    lesson_trace::trace_note!("Масштабируем текущую величину делением.");
    cluster_center[1] /= cluster.len() as f64;
    lesson_trace::trace_step!(cluster_center);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("центроид = {cluster_center:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_cluster_points_and_coordinate_averages(cluster, cluster_center);
}

// Строим график по результатам урока.
fn plot_cluster_points_and_coordinate_averages(cluster: [[f64; 2]; 2], cluster_center: [f64; 2]) {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    let observation_points: Vec<(f64, f64)> = cluster
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    lesson_trace::trace_note!("Собираем значения для `cluster_center_points` в коллекцию.");
    let cluster_center_points: Vec<(f64, f64)> = vec![(cluster_center[0], cluster_center[1])];
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
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Точки кластера и центроид",
        "x",
        "y",
        &[
            lesson_visualization::Series {
                name: "точки",

                points: &observation_points,
            },
            lesson_visualization::Series {
                name: "центроид",

                points: &cluster_center_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
