//! Вычисления и примеры урока part-052-lesson-09-linear-regression.

// Сводная практика 09. Линейная регрессия.
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
pub fn run() {
    // Фиксируем демонстрационные данные на время выполнения программы.
    const TRAINING_EXAMPLES: [(f64, f64); 5] = [(0., 1.), (1., 3.), (2., 5.), (3., 7.), (4., 9.)];
    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для обучения нужен хотя бы один пример"
    );

    // Шаг: Обучаем коэффициенты линейной модели на train.
    let (weight, bias) = (|| -> (f64, f64) {
        /* Обновляем вес и смещение по среднему градиенту квадратичной ошибки. */
        let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
        // Сохраняем рассчитанное значение `(mut weight, mut bias)` для следующих операций.
        let (mut weight, mut bias) = (0., 0.);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for _ in 0..3000 {
            // Выполняем встроенный расчёт один раз и сохраняем результат в `(weight_gradient, bias_gradient)`.
            let (weight_gradient, bias_gradient) = (|| -> (f64, f64) {
                /* Частные производные MSE по весу и смещению: 2/n * sum(error*x) и 2/n * sum(error). */
                let data: &[(f64, f64)] = data;
                // Сохраняем рассчитанное значение `weight` для следующих операций.
                let weight: f64 = weight;
                // Сохраняем рассчитанное значение `bias` для следующих операций.
                let bias: f64 = bias;
                // Сохраняем рассчитанное значение `(mut weight_gradient_sum, mut bias_gradient_sum)` для следующих операций.
                let (mut weight_gradient_sum, mut bias_gradient_sum) = (0.0, 0.0);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for &(feature_value, target_value) in data {
                    // Умножаем значения и сохраняем результат в `prediction_error`.
                    let prediction_error = weight * feature_value + bias - target_value;
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    weight_gradient_sum += 2.0 * feature_value * prediction_error;
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    bias_gradient_sum += 2.0 * prediction_error;
                }
                // Составляем результат из вычисленных значений в указанном порядке.
                (
                    // Делим значения, получая нормированную величину или среднее.
                    weight_gradient_sum / data.len() as f64,
                    // Делим значения, получая нормированную величину или среднее.
                    bias_gradient_sum / data.len() as f64,
                )
            })();
            // Вычитаем очередной вклад из текущего значения параметра.
            weight -= 0.02 * weight_gradient;
            // Вычитаем очередной вклад из текущего значения параметра.
            bias -= 0.02 * bias_gradient;
        }
        // Составляем результат из вычисленных значений в указанном порядке.
        (weight, bias)
    })();
    // Шаг: Отдельно задаём примеры, которые не участвовали в обучении.
    let test = [(5., 11.), (6., 13.)];

    let targets: Vec<_> = test.iter().map(|&(_, target)| target).collect();
    let model_predictions: Vec<_> = test
        .iter()
        .map(|&(feature, _)| weight * feature + bias)
        .collect();
    let baseline_predictions: Vec<_> = test.iter().map(|_| 5.0).collect();
    let model_mse = lesson_047::mean_squared_error(&targets, &model_predictions).unwrap();
    let baseline_mse = lesson_047::mean_squared_error(&targets, &baseline_predictions).unwrap();
    println!("w={weight:.3}, b={bias:.3}, test MSE={model_mse:.6}, baseline MSE={baseline_mse:.3}");
}
