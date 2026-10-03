// Урок 06.3. Разброс значений (выборочная дисперсия): сумма квадратов отклонений от среднего, делённая на число значений минус один.
// Связь с принятой терминологией: Выборочная дисперсия числовых значений.
// Зачем здесь эта тема: Центр не описывает разброс; дисперсия измеряет средний квадрат отклонений
//   от центра.
// Почему код устроен так: Вычитаем выборочное среднее и делим на n−1, поскольку оцениваем разброс
//   совокупности по выборке.
// Представь: Наборы [4, 4, 4] и [2, 4, 6] имеют одно среднее, но второй заметно сильнее разбросан.
//
// Общая функция использует среднее из урока 06.1. При одинаковых значениях разброс равен нулю;
// для выборочной оценки нужны хотя бы два значения.

use l032_06_calculate_sample_variance_from_squared_differences_from_mean::calculate_sample_variance_from_squared_differences_from_mean_divided_by_count_minus_one_where_0_means_all_values_equal_and_larger_means_more_spread;

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("все значения одинаковы", &[4.0, 4.0, 4.0], 0.0),
        ("умеренный разброс", &[2.0, 4.0, 6.0], 4.0),
        ("значения раздвинули", &[0.0, 4.0, 8.0], 16.0),
    ];
    for (_description, values, expected) in cases {
        assert_eq!(calculate_sample_variance_from_squared_differences_from_mean_divided_by_count_minus_one_where_0_means_all_values_equal_and_larger_means_more_spread(values)
                .expect("для выборочной дисперсии нужны хотя бы два значения"), expected);
    }
    let _: &str =
        calculate_sample_variance_from_squared_differences_from_mean_divided_by_count_minus_one_where_0_means_all_values_equal_and_larger_means_more_spread(&[
            4.0,
        ])
        .expect_err("одного значения недостаточно");

    plot_squared_differences_from_mean();
}

// Строим график по результатам урока.
fn plot_squared_differences_from_mean() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Разброс относительно среднего",
        "значение",
        "квадрат отклонения",
        &[lesson_visualization::Series {
            name: "среднее=4",

            points: &(0..=80)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (
                        horizontal_value,
                        (horizontal_value - 4.0) * (horizontal_value - 4.0),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
