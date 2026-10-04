// Урок 034. Строить приближённые границы вокруг среднего по оценке его стандартной ошибки.
// Множитель 1.96 даёт нормальное приближение; пример с четырьмя числами не гарантирует точный
// 95-процентный охват.

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

    let half_width = approximate_95_percent_confidence_interval_half_width;
    println!(
        "Среднее={mean}; приближённый интервал=[{}, {}]",
        mean - half_width,
        mean + half_width
    );
    println!(
        "При вчетверо большем числе независимых наблюдений и том же разбросе полуширина стала бы {}",
        half_width / 2.0
    );
    assert!(half_width > 0.0);
    println!("Это нормальное приближение, а не гарантия для выборки из четырёх чисел.");
}

// Чему учит этот урок:
// Учимся строить приближённые границы вокруг среднего по оценке его стандартной ошибки.
// Множитель 1.96 даёт нормальное приближение; пример с четырьмя числами не гарантирует точный
// 95-процентный охват.
