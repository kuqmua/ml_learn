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
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;
use l032_06_calculate_sample_variance_from_squared_differences_from_mean::calculate_sample_variance_from_squared_differences_from_mean_divided_by_count_minus_one;

fn main() {
    let values: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    let mean: f64 = calculate_mean_by_summing_values_and_dividing_by_count(&values).unwrap();

    let standard_error_squared: f64 =
        calculate_sample_variance_from_squared_differences_from_mean_divided_by_count_minus_one(
            &values,
        )
        .unwrap()
            / values.len() as f64;
    let mut standard_error: f64 = standard_error_squared;
    for _ in 0..80 {
        standard_error = (standard_error + standard_error_squared / standard_error) / 2.0;
    }
    let margin: f64 = 1.96 * standard_error;
    let _ = (&(mean - margin), &(mean + margin));

    plot_observations_mean_and_uncertainty_bounds(values, mean, margin);
}

// Строим график по результатам урока.
fn plot_observations_mean_and_uncertainty_bounds(values: [f64; 4], mean: f64, margin: f64) {
    let mean_line: [(f64, f64); 2] = [(1.0, mean), (values.len() as f64, mean)];
    let lower: [(f64, f64); 2] = [(1.0, mean - margin), (values.len() as f64, mean - margin)];
    let upper: [(f64, f64); 2] = [(1.0, mean + margin), (values.len() as f64, mean + margin)];
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
