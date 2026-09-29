// Урок 09.5. Проверка прогноза по прямой на данных, не использованных для обучения.
// Связь с принятой терминологией: Оценка линейной регрессии на отложенных данных.
// Зачем здесь эта тема: Уменьшение ошибки на train не доказывает обобщение; нужны данные, не
//   участвовавшие в подгонке.
// Почему код устроен так: Фиксируем модель и считаем ту же метрику на отложенных строках.
// Представь: Модель может идеально помнить train и ошибаться на новых строках; отложенная часть
//   показывает это.
//
// Что изучаем: Качество на отложенных данных.
// Зачем это нужно: Train служит для выбора параметров; качество модели оцениваем на новых примерах, не
// участвовавших в обучении.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `training` для следующего шага примера.
    let training: [(f64, f64); 2] = [(1.0, 3.0), (2.0, 5.0)];
    lesson_trace::trace_step!(training);
    // Создаём набор значений `test` для следующего шага примера.
    let test: [(f64, f64); 2] = [(3.0, 7.0), (4.0, 9.0)];
    lesson_trace::trace_step!(test);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        // Обновляем значение результатом текущего вычисления.
        training.len() >= 2,
        // Передаём подпись или текстовое значение для следующего шага.
        "для прямой нужны хотя бы две обучающие точки"
    );
    // Проверяем ожидаемое свойство учебного примера.
    assert_ne!(
        // Используем подготовленное значение в следующем шаге примера.
        training[0].0,
        // Сравниваем признаки второй обучающей точки с первой.
        training[1].0,
        // Передаём подпись или текстовое значение для следующего шага.
        "обучающие точки должны иметь разные значения x"
    );
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        // Используем подготовленное значение в следующем шаге примера.
        !test.is_empty(),
        // Передаём подпись или текстовое значение для следующего шага.
        "для MSE нужен хотя бы один тестовый пример"
    );
    // Нормируем или усредняем величину делением и сохраняем её в `weight`.
    let weight: f64 = (training[1].1 - training[0].1) / (training[1].0 - training[0].0);
    lesson_trace::trace_step!(weight);
    // Умножаем значения и сохраняем результат в `bias`.
    let bias: f64 = training[0].1 - weight * training[0].0;
    lesson_trace::trace_step!(bias);
    // Собираем значения для `targets` в коллекцию.
    let targets: Vec<f64> = test.iter().map(|&(_, target)| target).collect();
    lesson_trace::trace_step!(targets);
    // Собираем значения для `predictions` в коллекцию.
    let predictions: Vec<f64> = test
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Преобразуем каждый элемент в новое значение.
        .map(|&(feature, _)| weight * feature + bias)
        // Собираем результаты в коллекцию.
        .collect();
    lesson_trace::trace_step!(predictions);
    // Сохраняем результат этого шага в `mean_squared_error_value`.
    let mean_squared_error_value: f64 =
        l050_09_calculate_mean_squared_error_as_squared_error_sum_divided_by_count::calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(&targets, &predictions).unwrap();
    lesson_trace::trace_step!(mean_squared_error_value);
    // Печатаем рассчитанные значения для проверки примера.
    println!("test MSE = {mean_squared_error_value}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_training_and_test_points_with_prediction_line(training, test, weight, bias);
}

// Строим график по результатам урока.
fn plot_training_and_test_points_with_prediction_line(
    training: [(f64, f64); 2],
    test: [(f64, f64); 2],
    weight: f64,
    bias: f64,
) {
    // Показываем значения, рассчитанные по данным примера.
    let training_points: Vec<(f64, f64)> = training
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    // Собираем значения для `test_points` в коллекцию.
    let test_points: Vec<(f64, f64)> = test
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    // Собираем значения для `model_points` в коллекцию.
    let model_points: Vec<(f64, f64)> = (0..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, weight * horizontal_value + bias)
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
        "Отложенные данные и прямая",
        // Указываем подпись горизонтальной оси.
        "признак",
        // Указываем подпись вертикальной оси.
        "цель и прогноз",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "обучение",
                // Передаём рассчитанные координаты точек.
                points: &training_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "тест",
                // Передаём рассчитанные координаты точек.
                points: &test_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "модель",
                // Передаём рассчитанные координаты точек.
                points: &model_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
