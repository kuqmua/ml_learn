// Урок 241. Проходим путь от данных до готового прогноза.
// Сначала считаем простой ответ для сравнения, затем подбираем веса модели на обучающих примерах.
// Отдельные примеры используем для выбора решения, а заключительный набор — для итоговой оценки.
// Сравниваем ошибки и проверяем, стало ли лучше после обучения.
// Важно не использовать правильные ответы итоговой проверки при подборе весов.

use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;
use l051_09_calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count::calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count;

fn main() {
    const EXAMPLE_DATA: [(f64, f64); 10] = [
        (0., 1.),
        (1., 3.),
        (2., 5.),
        (3., 7.),
        (4., 9.),
        (5., 11.),
        (6., 13.),
        (7., 15.),
        (8., 17.),
        (9., 19.),
    ];

    assert!(
        EXAMPLE_DATA.len() >= 9,
        "для разделения нужны train, validation и test"
    );
    /// Средняя абсолютная ошибка линейной модели: для каждого x считаем weight·x+constant_input_weight, сравниваем с ответом и усредняем модули ошибок.
    fn calc_linear_model_error_as_average_absolute_diff_between_predictions_and_targets(
        data: &[(f64, f64)],

        weight: f64,

        constant_input_weight: f64,
    ) -> f64 {
        calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(
            &data.iter().map(|&(_, target)| target).collect::<Vec<_>>(),
            &data
                .iter()
                .map(|&(feature, _)| weight * feature + constant_input_weight)
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }

    let training_examples: &[(f64, f64)] = &EXAMPLE_DATA[..6];
    let (weight, constant_input_weight): (f64, f64) = (|| -> (f64, f64) {
        let data: &[(f64, f64)] = training_examples;
        let sample_count: f64 = data.len() as f64;
        let (mut feature_sum, mut target_sum): (f64, f64) = (0.0, 0.0);
        for &(feature_value, target_value) in data {
            feature_sum += feature_value;
            target_sum += target_value;
        }
        let mean_feature: f64 = feature_sum / sample_count;
        let mean_target: f64 = target_sum / sample_count;
        let (mut sum_after_multiplying_joint_diffs_from_mean, mut variance_sum): (f64, f64) =
            (0.0, 0.0);
        for &(feature_value, target_value) in data {
            sum_after_multiplying_joint_diffs_from_mean +=
                (feature_value - mean_feature) * (target_value - mean_target);
            variance_sum += (|| -> f64 {
                let value: f64 = feature_value - mean_feature;
                value * value
            })();
        }
        let weight: f64 = sum_after_multiplying_joint_diffs_from_mean / variance_sum;
        (weight, mean_target - weight * mean_feature)
    })();
    let validation: &[(f64, f64)] = &EXAMPLE_DATA[6..8];
    let _ = calc_linear_model_error_as_average_absolute_diff_between_predictions_and_targets(
        validation,
        0.,
        calc_mean_by_summing_values_and_dividing_by_count(
            &training_examples
                .iter()
                .map(|&(_, target)| target)
                .collect::<Vec<_>>(),
        )
        .unwrap(),
    );
    let _ = calc_linear_model_error_as_average_absolute_diff_between_predictions_and_targets(
        validation,
        weight,
        constant_input_weight,
    );
    let test: &[(f64, f64)] = &EXAMPLE_DATA[8..];
    let _ = calc_linear_model_error_as_average_absolute_diff_between_predictions_and_targets(
        test,
        weight,
        constant_input_weight,
    );
    let _ = weight * 10. + constant_input_weight;

    // Выполняем вычисления из примера.
    let _ = (training_examples, weight, constant_input_weight);
}
