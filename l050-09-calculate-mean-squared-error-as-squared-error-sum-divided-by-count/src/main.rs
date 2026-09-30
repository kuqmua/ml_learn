// Урок 09.1. Средняя квадратичная ошибка прогноза: сумма квадратов ошибок, делённая на число примеров.
// Связь с принятой терминологией: Средний квадрат разности правильных ответов и прогнозов.
// Зачем здесь эта тема: Регрессии нужна мера ошибки; квадрат сильнее штрафует крупные промахи.
// Почему код устроен так: Считаем разности прогнозов и целей поэлементно, затем усредняем квадраты.
// Представь: Промах на 3 даёт квадрат ошибки 9, а промах на 1 — только 1.
//
// При точном прогнозе MSE равна нулю. Ошибка вдвое больше даёт вклад вчетверо больше.
// Та же общая функция будет использоваться для оценки моделей в следующих уроках.

use l050_09_calculate_mean_squared_error_as_squared_error_sum_divided_by_count::calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 3] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    for (_description, predictions, expected) in cases {
        let mean_squared_error_value: f64 =
            calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
                &targets,
                predictions,
            )
            .expect("у каждого прогноза есть правильный ответ");
        assert!((mean_squared_error_value - expected).abs() < 1e-10);
    }
    let _error: &str =
        calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
            &targets,
            &[2.0, 4.0],
        )
        .expect_err("длины должны совпадать");

    plot_average_squared_prediction_error_for_changing_offset();
}

// Строим график по результатам урока.
fn plot_average_squared_prediction_error_for_changing_offset() {
    let mean_squared_error_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            let prediction_difference: f64 = plot_step_index as f64 / 10.0;
            (
                prediction_difference,
                calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
                    &[2.0, 4.0, 6.0],
                    &[
                        2.0 + prediction_difference,
                        4.0 + prediction_difference,
                        6.0 + prediction_difference,
                    ],
                )
                .unwrap(),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Среднеквадратичная ошибка",
        "смещение прогноза",
        "MSE",
        &[lesson_visualization::Series {
            name: "цели [2,4,6]",

            points: &mean_squared_error_points,
        }],
    )
    .expect("не удалось сохранить график");
}
