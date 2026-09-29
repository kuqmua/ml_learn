// Урок 06.7. Практика: среднее, середина и квадраты отклонений для данных из CSV.
// Связь с принятой терминологией: Среднее, медиана и выборочная дисперсия из CSV.
// Зачем здесь эта тема: Статистики нужны для реальных строк данных, а не только для готового
//   массива чисел.
// Почему код устроен так: Читаем CSV, разбираем значения и проверяем сводки после загрузки.
// Представь: В файле числа сначала представлены текстом; перед средним их нужно прочитать и
//   проверить.
//
// Разбираем CSV и объединяем вычисления среднего и дисперсии из общей библиотеки.
// Медиану находим после сортировки. Пустые строки пропускаем, неверное число сообщаем явно.

fn main() {
    lesson_trace::enable();
    // Задаём неизменяемые учебные данные.
    const SAMPLE_COMMA_SEPARATED_VALUES: &str = "value\n2\n4\n\n6\n8\n";
    // Сохраняем результат этого шага в `values`.
    let mut values: Vec<f64> = Vec::new();
    lesson_trace::trace_step!(values);
    // Повторяем расчёт для каждого элемента последовательности.
    for (line_index, line) in SAMPLE_COMMA_SEPARATED_VALUES.lines().enumerate().skip(1) {
        lesson_trace::trace_step!(line_index);
        lesson_trace::trace_step!(line);
        // Выбираем дальнейший шаг по выполнению условия.
        if line.trim().is_empty() {
            // Переходим к следующему шагу цикла или завершаем его.
            continue;
        }
        // Сохраняем результат этого шага в `value`.
        let value: f64 = line
            // Настраиваем или преобразуем результат предыдущего шага.
            .parse()
            // Настраиваем или преобразуем результат предыдущего шага.
            .unwrap_or_else(|_| panic!("строка {}: не число", line_index + 1));
        lesson_trace::trace_step!(value);
        // Используем подготовленное значение в следующем шаге примера.
        values.push(value);
    }
    // Проверяем ожидаемое свойство учебного примера.
    assert!(!values.is_empty(), "для статистики нужны числовые значения");
    // Используем подготовленное значение в следующем шаге примера.
    values.sort_by(f64::total_cmp);

    // Сохраняем результат этого шага в `mean`.
    let mean: f64 =
        part_029_lesson_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(&values)
            .unwrap();
    lesson_trace::trace_step!(mean);
    // Сохраняем результат этого шага в `sample_variance`.
    let sample_variance: f64 =
        part_031_lesson_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(
            &values,
        )
        .unwrap();
    lesson_trace::trace_step!(sample_variance);
    // Определяем размер данных и сохраняем его в `middle`.
    let middle: usize = values.len() / 2;
    lesson_trace::trace_step!(middle);
    // Определяем размер данных и сохраняем его в `median`.
    let median: f64 = if values.len() % 2 == 0 {
        // Вычисляем значение по указанной формуле.
        (values[middle - 1] + values[middle]) / 2.0
    // Обрабатываем случай, когда предыдущее условие не выполнено.
    } else {
        // Используем подготовленное значение в следующем шаге примера.
        values[middle]
    };
    lesson_trace::trace_step!(median);
    // Печатаем рассчитанные значения для проверки примера.
    println!(
        // Передаём подпись или текстовое значение для следующего шага.
        "n={}, mean={mean}, median={median}, sample variance={sample_variance:.3}",
        // Используем подготовленное значение в следующем шаге примера.
        values.len()
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_csv_values_mean_and_middle_of_sorted_values(values, mean, median);
}

// Строим график по результатам урока.
fn plot_csv_values_mean_and_middle_of_sorted_values(
    values: std::vec::Vec<f64>,
    mean: f64,
    median: f64,
) {
    // Значения из этого урока на графике.
    let comma_separated_value_observation_points: Vec<(f64, f64)> = values
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `mean_points` в коллекцию.
    let mean_points: Vec<(f64, f64)> = [(1.0, mean), (values.len() as f64, mean)].to_vec();
    // Собираем значения для `median_points` в коллекцию.
    let median_points: Vec<(f64, f64)> = [(1.0, median), (values.len() as f64, median)].to_vec();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Статистика выборки",
        // Указываем подпись горизонтальной оси.
        "номер наблюдения",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "CSV",
                // Передаём рассчитанные координаты точек.
                points: &comma_separated_value_observation_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "среднее",
                // Передаём рассчитанные координаты точек.
                points: &mean_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "медиана",
                // Передаём рассчитанные координаты точек.
                points: &median_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
