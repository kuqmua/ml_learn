// Урок 01.2. Сложение модулей координат вектора.
// Связь с принятой терминологией: Сумма модулей координат одного вектора (норма L1).
// Зачем здесь эта тема: После произведения векторов нужна величина одного вектора; L1 измеряет
//   общий модуль координат без взаимного сокращения знаков.
// Почему код устроен так: Складываем модули явно, чтобы отрицательные координаты не уменьшали
//   норму.
// Представь: Если координаты равны −3 и 4, общий размер по L1 равен 3+4=7: минус не уменьшает
//   ответ.
//
// Что изучаем: складываем модули всех координат. Отрицательное число даёт положительный вклад,
// поэтому смена знаков не меняет ответ. У нулевого вектора результат равен нулю.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `cases`.
    let cases: [(&str, [f64; 2], f64); 4] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("положительные координаты", [3.0, 4.0], 7.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("смешанные знаки", [3.0, -4.0], 7.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("сменили оба знака", [-3.0, 4.0], 7.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("нулевой вектор", [0.0, 0.0], 0.0),
    ];
    lesson_trace::trace_step!(cases);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, vector, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(vector);
        lesson_trace::trace_step!(expected);
        // Формула из общей библиотеки пригодится и в сводной практике.
        let sum_absolute_values_of_vector_coordinates: f64 =
            part_002_lesson_01_sum_absolute_values_of_vector_coordinates::sum_absolute_values_of_vector_coordinates(
                &vector,
            );
        lesson_trace::trace_step!(sum_absolute_values_of_vector_coordinates);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(sum_absolute_values_of_vector_coordinates, expected);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: {vector:?} → L1 = {sum_absolute_values_of_vector_coordinates}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_sum_of_absolute_coordinates_for_changing_first_coordinate();
}

// Строим график по результатам урока.
fn plot_sum_of_absolute_coordinates_for_changing_first_coordinate() {
    // Наглядное представление величин из этого урока.
    let sum_absolute_values_of_vector_coordinates_points: Vec<(f64, f64)> = (-50..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (
                horizontal_value,
                part_002_lesson_01_sum_absolute_values_of_vector_coordinates::sum_absolute_values_of_vector_coordinates(
                    &[horizontal_value, 4.0],
                ),
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
        "Сумма модулей координат одного вектора (норма L1)",
        // Указываем подпись горизонтальной оси.
        "первая координата",
        // Указываем подпись вертикальной оси.
        "L1",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "вектор [x, 4]",
            // Передаём рассчитанные координаты точек.
            points: &sum_absolute_values_of_vector_coordinates_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
