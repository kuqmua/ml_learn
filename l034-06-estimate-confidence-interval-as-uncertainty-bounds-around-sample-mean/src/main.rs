// Урок 06.5. Доверительный интервал среднего: оценка границ неопределённости по выборке.
// Связь с принятой терминологией: Доверительный интервал для среднего генеральной совокупности.
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
use l032_06_calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one_where_0_means_all_values_equal_and_larger_means_more_spread::calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one_where_0_means_all_values_equal_and_larger_means_more_spread;

fn main() {
    let values: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    let mean: f64 = calc_mean_by_summing_values_and_dividing_by_count(&values).unwrap();

    let estimated_variance_of_sample_mean: f64 =
        calc_sample_variance_from_squared_diffs_from_mean_divided_by_count_minus_one_where_0_means_all_values_equal_and_larger_means_more_spread(
            &values,
        )
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

    plot_observations_mean_and_uncertainty_bounds(
        values,
        mean,
        approximate_95_percent_confidence_interval_half_width,
    );
}

// Строим график по результатам урока.
fn plot_observations_mean_and_uncertainty_bounds(
    values: [f64; 4],
    mean: f64,
    approximate_95_percent_confidence_interval_half_width: f64,
) {
    let mean_line: [(f64, f64); 2] = [(1.0, mean), (values.len() as f64, mean)];
    let lower: [(f64, f64); 2] = [
        (
            1.0,
            mean - approximate_95_percent_confidence_interval_half_width,
        ),
        (
            values.len() as f64,
            mean - approximate_95_percent_confidence_interval_half_width,
        ),
    ];
    let upper: [(f64, f64); 2] = [
        (
            1.0,
            mean + approximate_95_percent_confidence_interval_half_width,
        ),
        (
            values.len() as f64,
            mean + approximate_95_percent_confidence_interval_half_width,
        ),
    ];
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Приближённый доверительный интервал",
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "выборка",

                points: &values
                    .iter()
                    .enumerate()
                    .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "среднее",

                points: &mean_line,
            },
            lesson_visualization::Series {
                name: "нижняя граница",

                points: &lower,
            },
            lesson_visualization::Series {
                name: "верхняя граница",

                points: &upper,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
