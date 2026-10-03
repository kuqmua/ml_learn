// Урок 055. Подбираем вес и постоянную прибавку в формуле «вход × вес + прибавка».
// Меняем их небольшими шагами так, чтобы прогнозы приближались к известным ответам.
// После обучения считаем ошибки на примерах, которые не использовали для подбора весов.
// Сравниваем с простым способом: всегда выдавать средний ответ обучающих примеров.

use l050_09_calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count::calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    const TRAINING_EXAMPLES: [(f64, f64); 5] = [(0., 1.), (1., 3.), (2., 5.), (3., 7.), (4., 9.)];
    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для обучения нужен хотя бы один пример"
    );

    let (weight, constant_input_weight): (f64, f64) = (|| -> (f64, f64) {
        let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
        let (mut weight, mut constant_input_weight): (f64, f64) = (0., 0.);
        for _ in 0..3000 {
            let (weight_loss_rate_of_change, constant_input_weight_loss_rate_of_change): (
                f64,
                f64,
            ) = (|| -> (f64, f64) {
                let data: &[(f64, f64)] = data;
                let weight: f64 = weight;
                let constant_input_weight: f64 = constant_input_weight;
                let (
                    mut accumulated_weight_loss_rate_of_change,
                    mut accumulated_constant_input_weight_loss_rate_of_change,
                ): (f64, f64) = (0.0, 0.0);
                for &(feature_value, target_value) in data {
                    let prediction_error: f64 =
                        weight * feature_value + constant_input_weight - target_value;
                    accumulated_weight_loss_rate_of_change +=
                        2.0 * feature_value * prediction_error;
                    accumulated_constant_input_weight_loss_rate_of_change += 2.0 * prediction_error;
                }
                (
                    accumulated_weight_loss_rate_of_change / data.len() as f64,
                    accumulated_constant_input_weight_loss_rate_of_change / data.len() as f64,
                )
            })();
            weight -= 0.02 * weight_loss_rate_of_change;
            constant_input_weight -= 0.02 * constant_input_weight_loss_rate_of_change;
        }
        (weight, constant_input_weight)
    })();
    let test: [(f64, f64); 2] = [(5., 11.), (6., 13.)];

    let targets: [f64; 2] = test.map(|(_, target)| target);
    let _: f64 = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &targets,
        &test.map(|(feature, _)| weight * feature + constant_input_weight),
    )
    .unwrap();
    let baseline_predictions: [f64; 2] = [5.0; 2];
    let _: f64 = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &targets,
        &baseline_predictions,
    )
    .unwrap();

    // Выполняем вычисления из примера.
    let _ = (weight, constant_input_weight, test);
}
