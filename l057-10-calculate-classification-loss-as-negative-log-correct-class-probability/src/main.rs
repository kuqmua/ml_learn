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

fn main() {
    let cases: [(&str, f64, f64); 4] = [
        ("верный уверенный прогноз", 1.0, 0.9),
        ("неуверенный прогноз", 1.0, 0.5),
        ("неверный уверенный прогноз", 1.0, 0.1),
        ("отрицательный класс предсказан верно", 0.0, 0.1),
    ];
    let mut losses: [f64; 4] = [0.0; 4];
    for (index, (_description, target, probability)) in cases.into_iter().enumerate() {
        assert!(target == 0.0 || target == 1.0);
        assert!(probability > 0.0 && probability < 1.0);
        let chosen_probability: f64 = if target == 1.0 {
            probability
        } else {
            1.0 - probability
        };
        let ratio: f64 = (chosen_probability - 1.0) / (chosen_probability + 1.0);
        let mut term: f64 = ratio;
        let mut logarithm: f64 = 0.0;
        for odd_divisor in (1..=99).step_by(2) {
            logarithm += term / odd_divisor as f64;
            term *= ratio * ratio;
        }
        losses[index] = -2.0 * logarithm;
        let _ = &(losses[index]);
    }

    plot_classification_loss_as_negative_log_probability_for_each_correct_class(losses);
}

// Строим график по результатам урока.
fn plot_classification_loss_as_negative_log_probability_for_each_correct_class(losses: [f64; 4]) {
    assert!(losses[0] < losses[1] && losses[1] < losses[2]);
    assert!((losses[0] - losses[3]).abs() < 1e-10);
    let positive_target_loss_points: Vec<(f64, f64)> = (1..100)
        .map(|plot_step_index| {
            let probability: f64 = plot_step_index as f64 / 100.0;
            (probability, -probability.ln())
        })
        .collect();
    let negative_target_loss_points: Vec<(f64, f64)> = (1..100)
        .map(|plot_step_index| {
            let probability: f64 = plot_step_index as f64 / 100.0;
            (probability, -(1.0 - probability).ln())
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Логарифмическая ошибка",
        "вероятность положительного класса",
        "ошибка",
        &[
            lesson_visualization::Series {
                name: "y=1",

                points: &positive_target_loss_points,
            },
            lesson_visualization::Series {
                name: "y=0",

                points: &negative_target_loss_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
