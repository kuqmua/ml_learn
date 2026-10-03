// Урок 46.1. Итоговый проект: сравнение, обучение и запуск модели прогнозирования.
// Связь с принятой терминологией: Сквозной ML проект от базовой модели до инференса.
// Зачем здесь эта тема: Итоговый ML-проект проверяет путь от данных и baseline до модели, оценки и
//   воспроизводимого инференса.
// Почему код устроен так: Оставляем небольшой набор и явные промежуточные результаты для проверки
//   каждого этапа.
// Представь: Сначала сравниваем константный прогноз с линейной моделью на validation, затем один
//   раз оцениваем на test.
//
// Что применяем: постановка задачи, baseline, обучение, оценка, сохранение, инференс.
// Зачем это нужно: Сквозной процесс связывает разделение данных, baseline, обучение, проверку и прогноз в
//   одном примере.
// Что показывает программа: Делим данные на train, validation и test. Считаем константный прогноз только по
//   train. Обучаем линейную модель и сравниваем её с baseline на validation и test.
// Что проверить при изменении примера: Один документ фиксирует метрику, split, baseline, лучший результат,
//   ошибки и команду воспроизведения.
// Возможное расширение: Выбери один открытый или самостоятельно созданный набор данных и собери
//   воспроизводимый pipeline из предыдущих пакетов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;
use l051_09_calculate_mean_absolute_error_as_absolute_error_sum_divided_by_count::calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse;

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
    fn calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets_where_0_means_exact_predictions_and_larger_means_worse(
        data: &[(f64, f64)],

        weight: f64,

        constant_input_weight: f64,
    ) -> f64 {
        calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count_where_0_means_exact_predictions_and_larger_means_worse(
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
        let (mut sum_after_multiplying_joint_differences_from_mean, mut variance_sum): (f64, f64) =
            (0.0, 0.0);
        for &(feature_value, target_value) in data {
            sum_after_multiplying_joint_differences_from_mean +=
                (feature_value - mean_feature) * (target_value - mean_target);
            variance_sum += (|| -> f64 {
                let value: f64 = feature_value - mean_feature;
                value * value
            })();
        }
        let weight: f64 = sum_after_multiplying_joint_differences_from_mean / variance_sum;
        (weight, mean_target - weight * mean_feature)
    })();
    let validation: &[(f64, f64)] = &EXAMPLE_DATA[6..8];
    let _ =
        calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets_where_0_means_exact_predictions_and_larger_means_worse(
            validation,
            0.,
            calculate_mean_by_summing_values_and_dividing_by_count(
                &training_examples
                    .iter()
                    .map(|&(_, target)| target)
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        );
    let _ =
        calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets_where_0_means_exact_predictions_and_larger_means_worse(
            validation,
            weight,
            constant_input_weight,
        );
    let test: &[(f64, f64)] = &EXAMPLE_DATA[8..];
    let _ =
        calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets_where_0_means_exact_predictions_and_larger_means_worse(
            test,
            weight,
            constant_input_weight,
        );
    let _ = weight * 10. + constant_input_weight;

    plot_training_points_and_fitted_prediction_line(
        training_examples,
        weight,
        constant_input_weight,
    );
}

// Строим график по результатам урока.
fn plot_training_points_and_fitted_prediction_line(
    training_examples: &[(f64, f64)],
    weight: f64,
    constant_input_weight: f64,
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Регрессия: данные и модель",
        "признак",
        "целевое значение",
        &[
            lesson_visualization::Series {
                name: "train",

                points: &training_examples
                    .iter()
                    .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "модель",

                points: &(0..=80)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (
                            horizontal_value,
                            weight * horizontal_value + constant_input_weight,
                        )
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
