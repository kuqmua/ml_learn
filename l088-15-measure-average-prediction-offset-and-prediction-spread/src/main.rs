// Урок 15.4. Измерение среднего смещения прогнозов и их разброса.
// Зачем здесь эта тема: Ансамбль полезен, когда ошибки отдельных моделей меняются от выборки к
//   выборке.
// Почему код устроен так: Сравниваем средний прогноз и его разброс, чтобы различить систематическую
//   ошибку и нестабильность.
// Представь: Если разные обучающие выборки дают сильно разные прогнозы одного объекта, у модели
//   высокий разброс.
//
// Что изучаем: Смещение и разброс.
// Зачем это нужно: Ошибка может происходить из систематического смещения модели или высокой изменчивости
// при разных обучающих наборах.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let predictions: [f64; 3] = [2.0, 4.0, 6.0];
    assert!(
        !predictions.is_empty(),
        "для оценки разброса нужен хотя бы один прогноз"
    );
    let mean: f64 = predictions.iter().sum::<f64>() / predictions.len() as f64;
    let target: f64 = 5.0;
    let average_prediction_minus_target: f64 = mean - target;
    let prediction_variance: f64 = predictions
        .iter()
        .map(|&input_value| (input_value - mean) * (input_value - mean))
        .sum::<f64>()
        / predictions.len() as f64;

    plot_squared_average_error_and_prediction_spread(
        average_prediction_minus_target,
        prediction_variance,
    );
}

// Строим график по результатам урока.
fn plot_squared_average_error_and_prediction_spread(
    average_prediction_minus_target: f64,
    prediction_variance: f64,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Смещение и разброс",
        "вклад в MSE",
        &[
            (
                "смещение²",
                average_prediction_minus_target * average_prediction_minus_target,
            ),
            ("разброс", prediction_variance),
        ],
    )
    .expect("не удалось сохранить график");
}
