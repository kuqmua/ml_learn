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
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], &[f64], f64); 3] = [
        ("разные точки", &[0.0, 0.0], &[3.0, 4.0], 5.0),
        ("поменяли точки местами", &[3.0, 4.0], &[0.0, 0.0], 5.0),
        ("точки совпадают", &[3.0, 4.0], &[3.0, 4.0], 0.0),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, first_point, second_point, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first_point);
        lesson_trace::trace_step!(second_point);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Общая функция проверяет размерности и вычисляет расстояние.");
        lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let distance: f64 = l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
            first_point,
            second_point,
        )

        .expect("точки в этом примере имеют одинаковую размерность");
        lesson_trace::trace_step!(distance);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((distance - expected).abs() < 1e-10);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {first_point:?} и {second_point:?} → {distance}");
    }
    lesson_trace::trace_note!("Задаём учебные значения для `first_point`.");
    let first_point: [f64; 2] = [0.0, 0.0];
    lesson_trace::trace_step!(first_point);
    lesson_trace::trace_note!("Задаём учебные значения для `too_short`.");
    let too_short: [f64; 1] = [3.0];
    lesson_trace::trace_step!(too_short);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `error`.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let error: &str =
        l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(&first_point, &too_short)

            .expect_err("точки разной размерности нужно отклонить");
    lesson_trace::trace_step!(error);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("разная размерность: {error}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_distance_from_origin_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_distance_from_origin_for_changing_first_coordinate() {
    lesson_trace::trace_note!("Наглядное представление величин из этого урока.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let distance_points: Vec<(f64, f64)> = (-50..=50)

        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
            lesson_trace::trace_note!("Задаём именованное поле или параметр.");
            (

                horizontal_value,

                l005_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_point_distance_as_square_root_of_sum_of_squared_coordinate_differences(
                    &[0.0, 0.0],
                    &[horizontal_value, 4.0],
                )
                .unwrap(),
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
        "Расстояние до начала координат",
        "x точки [x, 4]",
        "расстояние",
        &[lesson_visualization::Series {
            name: "расстояние",

            points: &distance_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
