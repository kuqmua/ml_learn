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

use l032_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one;

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("все значения одинаковы", &[4.0, 4.0, 4.0], 0.0),
        ("умеренный разброс", &[2.0, 4.0, 6.0], 4.0),
        ("значения раздвинули", &[0.0, 4.0, 8.0], 16.0),
    ];
    for (_description, values, expected) in cases {
        let variance: f64 =
            calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(values)
                .expect("для этой выборки дисперсия определена");
        assert_eq!(variance, expected);
    }
    let _error: &str =
        calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(&[4.0])
            .expect_err("одного значения недостаточно");

    plot_squared_deviations_from_mean();
}

// Строим график по результатам урока.
fn plot_squared_deviations_from_mean() {
    let variance_points: Vec<(f64, f64)> = (0..=80)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (
                horizontal_value,
                (horizontal_value - 4.0) * (horizontal_value - 4.0),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Разброс относительно среднего",
        "значение",
        "квадрат отклонения",
        &[lesson_visualization::Series {
            name: "среднее=4",

            points: &variance_points,
        }],
    )
    .expect("не удалось сохранить график");
}
