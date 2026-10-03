// Урок 10.2. Ошибка классификации: отрицательный логарифм вероятности правильного класса.
// Связь с принятой терминологией: Логарифмическая ошибка по правильному ответу и прогнозной вероятности.
// Зачем здесь эта тема: Для обучения вероятностного классификатора нужна ошибка, сильно штрафующая
//   уверенный неверный ответ.
// Почему код устроен так: Берём логарифм вероятности правильного класса и явно проверяем поведение
//   у границ 0 и 1.
// Представь: Прогноз 0,99 для неверного класса должен штрафоваться сильнее, чем неуверенный прогноз
//   0,55.
//
// Уверенный правильный прогноз имеет малую ошибку; уверенный неверный — большую.
// Вероятности 0 и 1 дают бесконечную ошибку для неверного класса, поэтому пример
// считает только строго внутренние вероятности.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

fn main() {
    let cases: [(&str, f64, f64); 4] = [
        ("верный уверенный прогноз", 1.0, 0.9),
        ("неуверенный прогноз", 1.0, 0.5),
        ("неверный уверенный прогноз", 1.0, 0.1),
        ("отрицательный класс предсказан верно", 0.0, 0.1),
    ];
    let mut negative_log_correct_class_probabilities_where_closer_to_0_means_better: [f64; 4] =
        [0.0; 4];
    for (index, (_description, target, probability)) in cases.into_iter().enumerate() {
        assert!(target == 0.0 || target == 1.0);
        assert!(probability > 0.0 && probability < 1.0);
        let probability_assigned_to_correct_class: f64 = if target == 1.0 {
            probability
        } else {
            1.0 - probability
        };
        let ratio: f64 = (probability_assigned_to_correct_class - 1.0)
            / (probability_assigned_to_correct_class + 1.0);
        let mut term: f64 = ratio;
        let mut logarithm: f64 = 0.0;
        for odd_divisor in (1..=99).step_by(2) {
            logarithm += term / odd_divisor as f64;
            term *= ratio * ratio;
        }
        negative_log_correct_class_probabilities_where_closer_to_0_means_better[index] =
            -2.0 * logarithm;
        let _ = &(negative_log_correct_class_probabilities_where_closer_to_0_means_better[index]);
    }

    plot_classification_loss_as_negative_log_probability_for_each_correct_class(
        negative_log_correct_class_probabilities_where_closer_to_0_means_better,
    );
}

// Строим график по результатам урока.
fn plot_classification_loss_as_negative_log_probability_for_each_correct_class(
    negative_log_correct_class_probabilities_where_closer_to_0_means_better: [f64; 4],
) {
    assert!(
        negative_log_correct_class_probabilities_where_closer_to_0_means_better[0]
            < negative_log_correct_class_probabilities_where_closer_to_0_means_better[1]
            && negative_log_correct_class_probabilities_where_closer_to_0_means_better[1]
                < negative_log_correct_class_probabilities_where_closer_to_0_means_better[2]
    );
    assert!(check_f64_eq_1e_minus_10(
        negative_log_correct_class_probabilities_where_closer_to_0_means_better[0],
        negative_log_correct_class_probabilities_where_closer_to_0_means_better[3]
    ));

    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Логарифмическая ошибка",
        "вероятность положительного класса",
        "ошибка",
        &[
            lesson_visualization::Series {
                name: "y=1",

                points: &(1..100)
                    .map(|plot_step_index| {
                        let probability: f64 = plot_step_index as f64 / 100.0;
                        (probability, -probability.ln())
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "y=0",

                points: &(1..100)
                    .map(|plot_step_index| {
                        let probability: f64 = plot_step_index as f64 / 100.0;
                        (probability, -(1.0 - probability).ln())
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
