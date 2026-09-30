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

fn main() {
    const SAMPLE_COMMA_SEPARATED_VALUES: &str = "value\n2\n4\n\n6\n8\n";
    let mut values: Vec<f64> = Vec::new();
    for (line_index, line) in SAMPLE_COMMA_SEPARATED_VALUES.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let value: f64 = line
            .parse()
            .unwrap_or_else(|_| panic!("строка {}: не число", line_index + 1));
        values.push(value);
    }
    assert!(!values.is_empty(), "для статистики нужны числовые значения");
    values.sort_by(f64::total_cmp);

    let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(&values).unwrap();
    let _sample_variance: f64 =
        calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(&values)
            .unwrap();
    let middle: usize = values.len() / 2;
    let median: f64 = if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    };
    let _ = &(values.len());

    plot_csv_values_mean_and_middle_of_sorted_values(values, mean, median);
}

// Строим график по результатам урока.
fn plot_csv_values_mean_and_middle_of_sorted_values(
    values: std::vec::Vec<f64>,
    mean: f64,
    median: f64,
) {
    let comma_separated_value_observation_points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        .collect();
    let mean_points: Vec<(f64, f64)> = [(1.0, mean), (values.len() as f64, mean)].to_vec();
    let median_points: Vec<(f64, f64)> = [(1.0, median), (values.len() as f64, median)].to_vec();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
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
}
