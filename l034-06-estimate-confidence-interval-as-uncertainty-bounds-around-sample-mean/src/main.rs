// Урок 06.5. Доверительный интервал среднего: оценка границ неопределённости по выборке.
// Зачем здесь эта тема: Выборочное среднее меняется от выборки к выборке; интервал отражает эту
//   неопределённость.
// Почему код устроен так: Соединяем среднее с оценкой стандартной ошибки и множителем 1,96; это
//   учебное нормальное приближение.
// Представь: Среднее из четырёх наблюдений — оценка; другой набор из той же совокупности мог бы
//   дать немного другое число.
//
// Что изучаем: Доверительный интервал среднего.
// Зачем это нужно: Интервал показывает неопределённость оценки среднего. Демонстрируем приближение mean ±
// 1.96·SE для небольшого учебного набора.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;
use l032_06_calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one::calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one;

fn main() {
    let values: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    let mean: f64 = calc_mean_by_summing_values_and_dividing_by_count(&values).unwrap();

    let estimated_variance_of_sample_mean: f64 =
        calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one(&values)
            .unwrap()
            / values.len() as f64;
    let mut estimated_standard_deviation_of_sample_mean: f64 = estimated_variance_of_sample_mean;
    for _ in 0..80 {
        estimated_standard_deviation_of_sample_mean = (estimated_standard_deviation_of_sample_mean
            + estimated_variance_of_sample_mean / estimated_standard_deviation_of_sample_mean)
            / 2.0;
    }
    let approximate_95_percent_confidence_interval_half_width: f64 =
        1.96 * estimated_standard_deviation_of_sample_mean;
    let _ = (
        &(mean - approximate_95_percent_confidence_interval_half_width),
        &(mean + approximate_95_percent_confidence_interval_half_width),
    );

    // Выполняем вычисления из примера.
    let _ = (
        values,
        mean,
        approximate_95_percent_confidence_interval_half_width,
    );
}
