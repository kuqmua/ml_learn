// Урок 09.2. Средняя абсолютная ошибка прогноза: сумма модулей ошибок, делённая на число примеров.
// Связь с принятой терминологией: Средний модуль разности правильных ответов и прогнозов.
// Зачем здесь эта тема: Квадратная ошибка подчёркивает выбросы; абсолютная показывает среднюю
//   величину промаха в исходных единицах.
// Почему код устроен так: Берём модули тех же разностей и сравниваем обе метрики на одинаковых
//   примерах.
// Представь: Для промахов на 3 и на 1 абсолютные вклады равны 3 и 1; крупный промах здесь не
//   возводится в квадрат.
//
// Ошибки разных знаков не сокращаются; удвоение промаха удваивает вклад в MAE.
// Общая библиотека проверяет, что у каждого прогноза есть правильный ответ.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l051_09_calculate_mean_absolute_error_as_absolute_error_sum_divided_by_count::calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse;

fn main() {
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 4] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка выше ответа", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка ниже ответа", &[2.0, 3.0, 6.0], 1.0 / 3.0),
        ("ошибка вдвое больше", &[2.0, 6.0, 6.0], 2.0 / 3.0),
    ];
    for (_description, predictions, expected) in cases {
        assert!(
            check_f64_eq_1e_minus_10(calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse(
                &targets,
                predictions,
            )
            .expect("нужен непустой набор прогнозов и правильных ответов одинаковой длины"), expected)
        );
    }

    plot_average_absolute_prediction_error_for_changing_offset();
}

// Строим график по результатам урока.
fn plot_average_absolute_prediction_error_for_changing_offset() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Средняя абсолютная ошибка",
        "смещение прогноза",
        "MAE",
        &[lesson_visualization::Series {
            name: "цели [2,4,6]",

            points: &(-30..=30)
                .map(|plot_step_index| {
                    let prediction_difference: f64 = plot_step_index as f64 / 10.0;
                    (
                prediction_difference,
                calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse(
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
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
