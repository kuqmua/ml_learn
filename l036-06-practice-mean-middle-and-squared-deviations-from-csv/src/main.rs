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

use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;
use l032_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one;

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём неизменяемые учебные данные.");
    const SAMPLE_COMMA_SEPARATED_VALUES: &str = "value\n2\n4\n\n6\n8\n";
    trace_note!("Сохраняем результат этого шага в `values`.");
    let mut values: Vec<f64> = Vec::new();
    trace_step!(values);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (line_index, line) in SAMPLE_COMMA_SEPARATED_VALUES.lines().enumerate().skip(1) {
        trace_step!(line_index);
        trace_step!(line);
        trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if line.trim().is_empty() {
            trace_note!("Переходим к следующему шагу цикла или завершаем его.");
            continue;
        }
        trace_note!("Сохраняем результат этого шага в `value`.");
        trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        let value: f64 = line
            .parse()
            .unwrap_or_else(|_| panic!("строка {}: не число", line_index + 1));
        trace_step!(value);
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        values.push(value);
    }
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!values.is_empty(), "для статистики нужны числовые значения");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    values.sort_by(f64::total_cmp);

    trace_note!("Сохраняем результат этого шага в `mean`.");
    let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(&values).unwrap();
    trace_step!(mean);
    trace_note!("Сохраняем результат этого шага в `sample_variance`.");
    let sample_variance: f64 =
        calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(&values)
            .unwrap();
    trace_step!(sample_variance);
    trace_note!("Определяем размер данных и сохраняем его в `middle`.");
    let middle: usize = values.len() / 2;
    trace_step!(middle);
    trace_note!("Определяем размер данных и сохраняем его в `median`.");
    let median: f64 = if values.len() % 2 == 0 {
        trace_note!("Вычисляем значение по указанной формуле.");
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        values[middle]
    };
    trace_step!(median);
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    println!(
        "n={}, mean={mean}, median={median}, sample variance={sample_variance:.3}",
        values.len()
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_csv_values_mean_and_middle_of_sorted_values(values, mean, median);
}

// Строим график по результатам урока.
fn plot_csv_values_mean_and_middle_of_sorted_values(
    values: std::vec::Vec<f64>,
    mean: f64,
    median: f64,
) {
    trace_note!("Значения из этого урока на графике.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let comma_separated_value_observation_points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        .collect();
    trace_note!("Собираем значения для `mean_points` в коллекцию.");
    let mean_points: Vec<(f64, f64)> = [(1.0, mean), (values.len() as f64, mean)].to_vec();
    trace_note!("Собираем значения для `median_points` в коллекцию.");
    let median_points: Vec<(f64, f64)> = [(1.0, median), (values.len() as f64, median)].to_vec();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
