// Урок 09.1. Средняя квадратичная ошибка прогноза: сумма квадратов ошибок, делённая на число примеров.
// Связь с принятой терминологией: Средний квадрат разности правильных ответов и прогнозов.
// Зачем здесь эта тема: Регрессии нужна мера ошибки; квадрат сильнее штрафует крупные промахи.
// Почему код устроен так: Считаем разности прогнозов и целей поэлементно, затем усредняем квадраты.
// Представь: Промах на 3 даёт квадрат ошибки 9, а промах на 1 — только 1.
//
// При точном прогнозе MSE равна нулю. Ошибка вдвое больше даёт вклад вчетверо больше.
// Та же общая функция будет использоваться для оценки моделей в следующих уроках.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l050_09_calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse::calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse;

fn main() {
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 3] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    for (_description, predictions, expected) in cases {
        assert!(
            check_f64_eq_1e_minus_10(calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse(
                &targets,
                predictions,
            )
            .expect("нужен непустой набор прогнозов и правильных ответов одинаковой длины"), expected)
        );
    }
    let _: &str = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse(
        &targets,
        &[2.0, 4.0],
    )
    .expect_err("ожидалась ошибка: число прогнозов и ответов различается");

    plot_average_squared_prediction_error_for_changing_offset();
}

// Строим график по результатам урока.
fn plot_average_squared_prediction_error_for_changing_offset() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Среднеквадратичная ошибка",
        "смещение прогноза",
        "MSE",
        &[lesson_visualization::Series {
            name: "цели [2,4,6]",

            points: &(-30..=30)
                .map(|plot_step_index| {
                    let prediction_diff: f64 = plot_step_index as f64 / 10.0;
                    (
                prediction_diff,
                calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse(
                    &[2.0, 4.0, 6.0],
                    &[
                        2.0 + prediction_diff,
                        4.0 + prediction_diff,
                        6.0 + prediction_diff,
                    ],
                )
                .unwrap(),
            )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
