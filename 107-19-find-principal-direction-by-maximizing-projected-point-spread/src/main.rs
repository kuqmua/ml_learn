// Урок 19.3. Главное направление данных: поиск наибольшего разброса проекций точек.
// Связь с принятой терминологией: Главное направление PCA с наибольшей дисперсией.
// Зачем здесь эта тема: PCA выбирает ось, на которую проекция данных имеет наибольшую дисперсию.
// Почему код устроен так: Для двух признаков связываем ковариационную матрицу с направлением её
//   главного собственного вектора.
// Представь: Для точек вдоль диагонали главная ось идёт примерно по этой диагонали, а не поперёк
//   неё.
//
// Что изучаем: Главное направление PCA.
// Зачем это нужно: Собственное направление с наибольшей дисперсией служит первой осью PCA. На точках вдоль
// x эта ось совпадает с x.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `centered_points` для следующего шага примера.
    let centered_points: [[f64; 2]; 4] = [[-2.0, 0.0], [-1.0, 0.0], [1.0, 0.0], [2.0, 0.0]];
    lesson_trace::trace_step!(centered_points);
    // Создаём набор значений `principal_axis` для следующего шага примера.
    let principal_axis: [f64; 2] = [1.0, 0.0];
    lesson_trace::trace_step!(principal_axis);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for point in centered_points {
        lesson_trace::trace_step!(point);
        // Умножаем значения и сохраняем результат в `projection`.
        let projection: f64 =
            // Используем подготовленное значение в следующем шаге примера.
            l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&point, &principal_axis).unwrap();
        lesson_trace::trace_step!(projection);
        // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
        println!("точка={point:?}, координата на главной оси={projection}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_centered_points_to_show_direction_of_greatest_spread(centered_points);
}

// Строим график по результатам урока.
fn plot_centered_points_to_show_direction_of_greatest_spread(centered_points: [[f64; 2]; 4]) {
    // Значения из этого урока на графике.
    let principal_direction_points: Vec<(f64, f64)> = centered_points
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Проекция на главное направление",
        // Указываем подпись горизонтальной оси.
        "первая координата",
        // Указываем подпись вертикальной оси.
        "вторая координата",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "центрированные точки",
            // Передаём рассчитанные координаты точек.
            points: &principal_direction_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
