// Урок 01.4. Расстояние между точками: квадратный корень из суммы квадратов разностей координат.
// Связь с принятой терминологией: Евклидово расстояние между точками.
// Зачем здесь эта тема: Квадрат расстояния из предыдущей части переводим в обычное расстояние.
// Почему код устроен так: Вызываем предыдущую часть и извлекаем корень стандартным sqrt.
//   Проверку размерностей выполняет функция квадрата расстояния.
// Представь: От [1, 2] до [4, 6] нужно пройти на 3 по первой оси и на 4 по второй; расстояние равно
//   5.
//
// Что изучаем: разности по каждой координате возводим в квадрат, складываем и извлекаем корень.
// Для совпадающих точек ответ 0. Порядок точек не влияет на расстояние.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `cases`.
    let cases: [(&str, &[f64], &[f64], f64); 3] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    lesson_trace::trace_step!(cases);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, first_point, second_point, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first_point);
        lesson_trace::trace_step!(second_point);
        lesson_trace::trace_step!(expected);
        // Общая функция проверяет размерности и вычисляет расстояние.
        let distance: f64 = l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            first_point,
            second_point,
        )
        // Используем результат, ожидая успешного выполнения шага.
        .expect("точки в этом примере имеют одинаковую размерность");
        lesson_trace::trace_step!(distance);
        // Проверяем ожидаемое свойство учебного примера.
        assert!((distance - expected).abs() < 1e-10);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: {first_point:?} и {second_point:?} → {distance}");
    }
    // Задаём учебные значения для `first_point`.
    let first_point: [f64; 2] = [0.0, 0.0];
    lesson_trace::trace_step!(first_point);
    // Задаём учебные значения для `too_short`.
    let too_short: [f64; 1] = [3.0];
    lesson_trace::trace_step!(too_short);
    // Сохраняем результат этого шага в `error`.
    let error: &str =
        l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(&first_point, &too_short)
            // Настраиваем или преобразуем результат предыдущего шага.
            .expect_err("точки разной размерности нужно отклонить");
    lesson_trace::trace_step!(error);
    // Печатаем рассчитанные значения для проверки примера.
    println!("разная размерность: {error}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_distance_from_origin_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_distance_from_origin_for_changing_first_coordinate() {
    // Наглядное представление величин из этого урока.
    let distance_points: Vec<(f64, f64)> = (-50..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (
                // Используем подготовленное значение в следующем шаге примера.
                horizontal_value,
                // Задаём именованное поле или параметр.
                l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                    &[0.0, 0.0],
                    &[horizontal_value, 4.0],
                )
                .unwrap(),
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Расстояние до начала координат",
        // Указываем подпись горизонтальной оси.
        "x точки [x, 4]",
        // Указываем подпись вертикальной оси.
        "расстояние",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "расстояние",
            // Передаём рассчитанные координаты точек.
            points: &distance_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
