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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Создаём набор значений `cluster` для следующего шага примера.");
    let cluster: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    trace_step!(cluster);
    trace_note!("Создаём набор значений `cluster_center` для следующего шага примера.");
    trace_note!("Среднее пустого кластера не определено.");
    assert!(!cluster.is_empty(), "для центра нужна хотя бы одна точка");
    trace_note!("Задаём учебные значения для `cluster_center`.");
    trace_note!("Центр группы точек называют centroid.");
    let mut cluster_center: [f64; 2] = [0.0, 0.0];
    trace_step!(cluster_center);
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    for point in cluster {
        trace_step!(point);
        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        cluster_center[0] += point[0];
        trace_step!(cluster_center);
        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        cluster_center[1] += point[1];
        trace_step!(cluster_center);
    }
    trace_note!("Масштабируем текущую величину делением.");
    cluster_center[0] /= cluster.len() as f64;
    trace_step!(cluster_center);
    trace_note!("Масштабируем текущую величину делением.");
    cluster_center[1] /= cluster.len() as f64;
    trace_step!(cluster_center);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("центроид = {cluster_center:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_cluster_points_and_coordinate_averages(cluster, cluster_center);
}

// Строим график по результатам урока.
fn plot_cluster_points_and_coordinate_averages(cluster: [[f64; 2]; 2], cluster_center: [f64; 2]) {
    trace_note!("Значения из этого урока на графике.");
    let observation_points: Vec<(f64, f64)> = cluster
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    trace_note!("Собираем значения для `cluster_center_points` в коллекцию.");
    let cluster_center_points: Vec<(f64, f64)> = vec![(cluster_center[0], cluster_center[1])];
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
