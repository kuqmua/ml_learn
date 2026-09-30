// Урок 09.5. Проверка прогноза по прямой на данных, не использованных для обучения.
// Связь с принятой терминологией: Оценка линейной регрессии на отложенных данных.
// Зачем здесь эта тема: Уменьшение ошибки на train не доказывает обобщение; нужны данные, не
//   участвовавшие в подгонке.
// Почему код устроен так: Фиксируем модель и считаем ту же метрику на отложенных строках.
// Представь: Модель может идеально помнить train и ошибаться на новых строках; отложенная часть
//   показывает это.
//
// Что изучаем: Качество на отложенных данных.
// Зачем это нужно: Train служит для выбора параметров; качество модели оцениваем на новых примерах, не
// участвовавших в обучении.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l050_09_calculate_mean_squared_error_as_squared_error_sum_divided_by_count::calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    let training: [(f64, f64); 2] = [(1.0, 3.0), (2.0, 5.0)];
    let test: [(f64, f64); 2] = [(3.0, 7.0), (4.0, 9.0)];
    assert!(
        training.len() >= 2,
        "для прямой нужны хотя бы две обучающие точки"
    );
    assert_ne!(
        training[0].0, training[1].0,
        "обучающие точки должны иметь разные значения x"
    );
    assert!(
        !test.is_empty(),
        "для MSE нужен хотя бы один тестовый пример"
    );
    let weight: f64 = (training[1].1 - training[0].1) / (training[1].0 - training[0].0);
    let bias: f64 = training[0].1 - weight * training[0].0;
    let targets: [f64; 2] = test.map(|(_, target)| target);
    let predictions: [f64; 2] = test.map(|(feature, _)| weight * feature + bias);
    let _mean_squared_error_value: f64 =
        calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
            &targets,
            &predictions,
        )
        .unwrap();

    plot_training_and_test_points_with_prediction_line(training, test, weight, bias);
}

// Строим график по результатам урока.
fn plot_training_and_test_points_with_prediction_line(
    training: [(f64, f64); 2],
    test: [(f64, f64); 2],
    weight: f64,
    bias: f64,
) {
    let training_points: Vec<(f64, f64)> = training
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    let test_points: Vec<(f64, f64)> = test
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    let model_points: Vec<(f64, f64)> = (0..=50)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            (horizontal_value, weight * horizontal_value + bias)
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Отложенные данные и прямая",
        "признак",
        "цель и прогноз",
        &[
            lesson_visualization::Series {
                name: "обучение",

                points: &training_points,
            },
            lesson_visualization::Series {
                name: "тест",

                points: &test_points,
            },
            lesson_visualization::Series {
                name: "модель",

                points: &model_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
