// Урок 06.7. Практика: среднее, середина и квадраты отклонений для данных из CSV.
// Зачем здесь эта тема: Статистики нужны для реальных строк данных, а не только для готового
//   массива чисел.
// Почему код устроен так: Читаем CSV, разбираем значения и проверяем сводки после загрузки.
// Представь: В файле числа сначала представлены текстом; перед средним их нужно прочитать и
//   проверить.
//
// Разбираем CSV и объединяем вычисления среднего и дисперсии из общей библиотеки.
// Медиану находим после сортировки. Пустые строки пропускаем, неверное число сообщаем явно.

use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;
use l032_06_calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one::calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one;

fn main() {
    const SAMPLE_COMMA_SEPARATED_VALUES: &str = "value\n2\n4\n\n6\n8\n";
    let mut values: Vec<f64> = Vec::new();
    for (line_index, line) in SAMPLE_COMMA_SEPARATED_VALUES.lines().enumerate().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        values.push(
            line.parse::<f64>()
                .unwrap_or_else(|_| panic!("строка {}: не число", line_index + 1)),
        );
    }
    assert!(!values.is_empty(), "для статистики нужны числовые значения");
    values.sort_by(f64::total_cmp);

    let _: f64 =
        calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one(&values)
            .unwrap();
    let middle: usize = values.len() / 2;
    let median: f64 = if values.len() % 2 == 0 {
        (values[middle - 1] + values[middle]) / 2.0
    } else {
        values[middle]
    };
    let _ = &(values.len());

    // Выполняем вычисления из примера.
    let _ = (
        &values,
        calc_mean_by_summing_values_and_dividing_by_count(&values).unwrap(),
        median,
    );
}
