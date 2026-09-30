// Урок 09.6. Практика: подбор прямой и проверка ошибок прогноза.
// Связь с принятой терминологией: Линейная регрессия, ошибки и оценка на отложенных данных.
// Зачем здесь эта тема: Теперь можно проследить полный путь регрессии от прогноза до честной
//   оценки.
// Почему код устроен так: На одних данных соединяем веса, ошибку, ограничение и отложенную
//   проверку.
// Представь: Сначала получаем прогнозы линейной модели, затем считаем ошибку и сравниваем с простым
//   ответом на новых данных.
//
// Что повторяем вместе: MSE, MAE, коэффициенты, регуляризация, качество на отложенных данных.
// Зачем это нужно: Линейная регрессия подбирает вес и смещение для числового прогноза, а baseline
//   показывает пользу обучения.
// Что показывает программа: Обучаем коэффициенты линейной модели на train. Отдельно задаём примеры, которые
//   не участвовали в обучении. Сравниваем ошибку модели с константным baseline.
// Что проверить при изменении примера: Проверь восстановление известных w,b без шума и рост ошибки при
//   сильной регуляризации.
// Дополнительная практика: Обучи y=wx+b градиентным спуском на синтетических данных; сравни с константным
//   baseline.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l050_09_calculate_mean_squared_error_as_squared_error_sum_divided_by_count::calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    const TRAINING_EXAMPLES: [(f64, f64); 5] = [(0., 1.), (1., 3.), (2., 5.), (3., 7.), (4., 9.)];
    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для обучения нужен хотя бы один пример"
    );

    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
        let (mut weight, mut bias): (f64, f64) = (0., 0.);
        for _ in 0..3000 {
            let (weight_loss_rate_of_change, bias_loss_rate_of_change): (f64, f64) =
                (|| -> (f64, f64) {
                    let data: &[(f64, f64)] = data;
                    let weight: f64 = weight;
                    let bias: f64 = bias;
                    let (
                        mut accumulated_weight_loss_rate_of_change,
                        mut accumulated_bias_loss_rate_of_change,
                    ): (f64, f64) = (0.0, 0.0);
                    for &(feature_value, target_value) in data {
                        let prediction_error: f64 = weight * feature_value + bias - target_value;
                        accumulated_weight_loss_rate_of_change +=
                            2.0 * feature_value * prediction_error;
                        accumulated_bias_loss_rate_of_change += 2.0 * prediction_error;
                    }
                    (
                        accumulated_weight_loss_rate_of_change / data.len() as f64,
                        accumulated_bias_loss_rate_of_change / data.len() as f64,
                    )
                })();
            weight -= 0.02 * weight_loss_rate_of_change;
            bias -= 0.02 * bias_loss_rate_of_change;
        }
        (weight, bias)
    })();
    let test: [(f64, f64); 2] = [(5., 11.), (6., 13.)];

    let targets: Vec<f64> = test.iter().map(|&(_, target)| target).collect();
    let model_predictions: Vec<f64> = test
        .iter()
        .map(|&(feature, _)| weight * feature + bias)
        .collect();
    let baseline_predictions: Vec<f64> = test.iter().map(|_| 5.0).collect();
    let _model_mean_squared_error: f64 =
        calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
            &targets,
            &model_predictions,
        )
        .unwrap();
    let _baseline_mean_squared_error: f64 =
        calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
            &targets,
            &baseline_predictions,
        )
        .unwrap();

    plot_prediction_line_and_held_out_points(weight, bias, test);
}

// Строим график по результатам урока.
fn plot_prediction_line_and_held_out_points(weight: f64, bias: f64, test: [(f64, f64); 2]) {
    let model_points: Vec<(f64, f64)> = (0..=60)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (horizontal_value, weight * horizontal_value + bias)
        })
        .collect();
    let test_points: Vec<(f64, f64)> = test
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Линейная регрессия и тест",
        "признак x",
        "целевое значение",
        &[
            lesson_visualization::Series {
                name: "модель",

                points: &model_points,
            },
            lesson_visualization::Series {
                name: "тест",

                points: &test_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
