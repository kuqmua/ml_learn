// Урок 055. Многократно обновлять вес и постоянную прибавку по средней производной ошибки.
// Считаем ошибку на отдельных примерах и ошибку постоянного прогноза, получая основу для сравнения
// моделей.

use l050_09_calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count::calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    const TRAINING_EXAMPLES: [(f64, f64); 5] =
        [(0.0, 1.0), (1.0, 3.0), (2.0, 5.0), (3.0, 7.0), (4.0, 9.0)];
    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для обучения нужен хотя бы один пример"
    );

    let (weight, constant_input_weight): (f64, f64) = (|| -> (f64, f64) {
        let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
        let (mut weight, mut constant_input_weight): (f64, f64) = (0.0, 0.0);
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
    let test: [(f64, f64); 2] = [(5.0, 11.0), (6.0, 13.0)];

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
    let _ = (&weight, &constant_input_weight, &test);

    let predictions = test.map(|(x, _)| weight * x + constant_input_weight);
    let model_error = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &targets,
        &predictions,
    )
    .unwrap();
    let baseline_error = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &targets,
        &baseline_predictions,
    )
    .unwrap();
    println!("Выученные параметры: вес={weight:.6}, прибавка={constant_input_weight:.6}");
    println!(
        "Тест: прогнозы={predictions:?}, ошибка={model_error:.6}; постоянный ответ: ошибка={baseline_error}"
    );
    assert!(model_error < baseline_error);
    assert!(model_error < 1e-6);
}

// Чему учит этот урок:
// Учимся многократно обновлять вес и постоянную прибавку по средней производной ошибки.
// Считаем ошибку на отдельных примерах и ошибку постоянного прогноза, получая основу для сравнения
// моделей.
