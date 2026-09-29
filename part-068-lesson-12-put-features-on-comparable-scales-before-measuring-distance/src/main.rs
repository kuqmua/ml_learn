// Урок 12.2. Приведение признаков к сопоставимым масштабам перед измерением расстояния.
// Связь с принятой терминологией: Масштабирование признаков перед вычислением расстояния до соседей.
// Зачем здесь эта тема: Признак с большим числовым диапазоном может захватить расстояние независимо
//   от полезности.
// Почему код устроен так: Масштабируем координаты по train и сравниваем порядок соседей до и после.
// Представь: Разность возраста на 30 лет численно перекроет разность доли 0,2, если признаки не
//   привести к сравнимому масштабу.
//
// Что изучаем: Масштабирование признаков.
// Зачем это нужно: Признак с большими единицами измерения может захватить расстояние. Масштабируем
// координаты перед сравнением соседей.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `first` для следующего шага примера.
    let first: [f64; 2] = [1.0, 1000.0];
    lesson_trace::trace_step!(first);
    // Создаём набор значений `second` для следующего шага примера.
    let second: [f64; 2] = [2.0, 1010.0];
    lesson_trace::trace_step!(second);
    // Создаём набор значений `scale` для следующего шага примера.
    let scale: [f64; 2] = [1.0, 1000.0];
    lesson_trace::trace_step!(scale);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        // Используем подготовленное значение в следующем шаге примера.
        scale.iter().all(|&value| value > 0.0),
        // Передаём подпись или текстовое значение для следующего шага.
        "масштабы должны быть положительными"
    );
    // Сохраняем результат этого шага в `raw_squared`.
    let raw_squared: f64 =
        part_004_lesson_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_squared_point_distance_by_summing_squared_coordinate_differences(&first, &second)
            .unwrap();
    lesson_trace::trace_step!(raw_squared);
    // Задаём учебные значения для `scaled_first`.
    let scaled_first: [f64; 2] = [first[0] / scale[0], first[1] / scale[1]];
    lesson_trace::trace_step!(scaled_first);
    // Задаём учебные значения для `scaled_second`.
    let scaled_second: [f64; 2] = [second[0] / scale[0], second[1] / scale[1]];
    lesson_trace::trace_step!(scaled_second);
    // Сохраняем результат этого шага в `scaled_squared`.
    let scaled_squared: f64 = part_004_lesson_01_calculate_point_distance_as_square_root_of_squared_coordinate_difference_sum::calculate_squared_point_distance_by_summing_squared_coordinate_differences(
        &scaled_first,
        &scaled_second,
    )
    .unwrap();
    lesson_trace::trace_step!(scaled_squared);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("до={raw_squared}, после масштабирования={scaled_squared}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_squared_distances_before_and_after_feature_scaling(raw_squared, scaled_squared);
}

// Строим график по результатам урока.
fn plot_squared_distances_before_and_after_feature_scaling(raw_squared: f64, scaled_squared: f64) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Эффект масштабирования",
        // Указываем подпись вертикальной оси.
        "квадрат расстояния",
        // Передаём ряды или значения для отрисовки графика.
        &[("до", raw_squared), ("после", scaled_squared)],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
