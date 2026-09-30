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
    lesson_trace::trace_note!("Задаём неизменяемые учебные данные.");
    const SAMPLE_COMMA_SEPARATED_VALUES: &str = "value\n2\n4\n\n6\n8\n";
    lesson_trace::trace_note!("Сохраняем результат этого шага в `values`.");
    let mut values: Vec<f64> = Vec::new();
    lesson_trace::trace_step!(values);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (line_index, line) in SAMPLE_COMMA_SEPARATED_VALUES.lines().enumerate().skip(1) {
        lesson_trace::trace_step!(line_index);
        lesson_trace::trace_step!(line);
        lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if line.trim().is_empty() {
            lesson_trace::trace_note!("Переходим к следующему шагу цикла или завершаем его.");
            continue;
        }
        lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        let value: f64 = line
            .parse()
            .unwrap_or_else(|_| panic!("строка {}: не число", line_index + 1));
        lesson_trace::trace_step!(value);
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        values.push(value);
    }
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!values.is_empty(), "для статистики нужны числовые значения");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    values.sort_by(f64::total_cmp);

    lesson_trace::trace_note!("Сохраняем результат этого шага в `mean`.");
    let mean: f64 =
        l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(&values)
            .unwrap();
    lesson_trace::trace_step!(mean);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `sample_variance`.");
    let sample_variance: f64 =
        l032_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(
            &values,
        )
        .unwrap();
    lesson_trace::trace_step!(sample_variance);
    lesson_trace::trace_note!("Определяем размер данных и сохраняем его в `middle`.");
    let middle: usize = values.len() / 2;
    lesson_trace::trace_step!(middle);
    lesson_trace::trace_note!("Определяем размер данных и сохраняем его в `median`.");
    let median: f64 = if values.len() % 2 == 0 {
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        lesson_trace::trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        values[middle]
    };
    lesson_trace::trace_step!(median);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    println!(
        "n={}, mean={mean}, median={median}, sample variance={sample_variance:.3}",
        values.len()
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_csv_values_mean_and_middle_of_sorted_values(values, mean, median);
}

// Строим график по результатам урока.
fn plot_csv_values_mean_and_middle_of_sorted_values(
    values: std::vec::Vec<f64>,
    mean: f64,
    median: f64,
) {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let comma_separated_value_observation_points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        .collect();
    lesson_trace::trace_note!("Собираем значения для `mean_points` в коллекцию.");
    let mean_points: Vec<(f64, f64)> = [(1.0, mean), (values.len() as f64, mean)].to_vec();
    lesson_trace::trace_note!("Собираем значения для `median_points` в коллекцию.");
    let median_points: Vec<(f64, f64)> = [(1.0, median), (values.len() as f64, median)].to_vec();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Статистика выборки",
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "CSV",

                points: &comma_separated_value_observation_points,
            },
            lesson_visualization::Series {
                name: "среднее",

                points: &mean_points,
            },
            lesson_visualization::Series {
                name: "медиана",

                points: &median_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
