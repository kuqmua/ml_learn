// Урок 18.3. Разброс внутри групп: сумма квадратов расстояний от точек до центров их групп.
// Связь с принятой терминологией: Сумма квадратов расстояний до назначенных центроидов.
// Зачем здесь эта тема: Чтобы улучшать кластеры, нужна величина, уменьшаемая после переназначения и
//   пересчёта центров.
// Почему код устроен так: Суммируем квадраты расстояний до назначенных центроидов без лишнего
//   извлечения корня.
// Представь: Точка на расстоянии 3 от своего центра добавляет к инерции 9.
//
// Что изучаем: Инерция кластеризации.
// Зачем это нужно: Инерция складывает квадраты расстояний точек до назначенных центров; меньшая означает
// более плотные группы.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `points` для следующего шага примера.");
    let points: [[f64; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [5.0, 0.0], [6.0, 0.0]];
    lesson_trace::trace_step!(points);
    lesson_trace::trace_note!("Создаём набор значений `centers` для следующего шага примера.");
    let centers: [[f64; 2]; 2] = [[0.5, 0.0], [5.5, 0.0]];
    lesson_trace::trace_step!(centers);
    lesson_trace::trace_note!("Создаём набор значений `assignments` для следующего шага примера.");
    let assignments: [usize; 4] = [0, 0, 1, 1];
    lesson_trace::trace_step!(assignments);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `total_squared_distance_to_cluster_centers` начальным состоянием."
    );
    lesson_trace::trace_note!("Каждой точке нужен индекс существующего центра.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert_eq!(
        points.len(),
        assignments.len(),
        "каждой точке нужен номер центра"
    );
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        assignments.iter().all(|&index| index < centers.len()),
        "номер центра выходит за границы списка"
    );
    lesson_trace::trace_note!(
        "Сохраняем результат этого шага в `total_squared_distance_to_cluster_centers`."
    );
    lesson_trace::trace_note!("Сумму квадратов расстояний до центров кластеров называют inertia.");
    let mut total_squared_distance_to_cluster_centers: f64 = 0.0;
    lesson_trace::trace_step!(total_squared_distance_to_cluster_centers);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for index in 0..points.len() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_note!("Комбинируем исходные величины и сохраняем результат в `delta`.");
        let delta: f64 = points[index][0] - centers[assignments[index]][0];
        lesson_trace::trace_step!(delta);
        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        total_squared_distance_to_cluster_centers += delta * delta;
        lesson_trace::trace_step!(total_squared_distance_to_cluster_centers);
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("инерция = {total_squared_distance_to_cluster_centers}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_squared_distance_to_nearest_fixed_center_for_changing_point();
}

// Строим график по результатам урока.
fn plot_squared_distance_to_nearest_fixed_center_for_changing_point() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let squared_distance_sum_points: Vec<(f64, f64)> = (0..=60)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                horizontal_value,
                ((horizontal_value - 0.5) * (horizontal_value - 0.5))
                    .min((horizontal_value - 5.5) * (horizontal_value - 5.5)),
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
        "Инерция для двух центров",
        "точка x",
        "квадрат расстояния",
        &[lesson_visualization::Series {
            name: "ближайший из 0.5 и 5.5",

            points: &squared_distance_sum_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
