// Сводная практика 09. Линейная регрессия, ошибки и оценка на отложенных данных.
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
fn main() {
    lesson_trace::enable();
    // Фиксируем демонстрационные данные на время выполнения программы.
    const TRAINING_EXAMPLES: [(f64, f64); 5] = [(0., 1.), (1., 3.), (2., 5.), (3., 7.), (4., 9.)];
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        // Используем подготовленное значение в следующем шаге примера.
        !TRAINING_EXAMPLES.is_empty(),
        // Передаём подпись или текстовое значение для следующего шага.
        "для обучения нужен хотя бы один пример"
    );

    // Шаг: Обучаем коэффициенты линейной модели на train.
    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        // Используем подготовленное значение в следующем шаге примера.
        /* Обновляем вес и смещение по среднему градиенту квадратичной ошибки. */
        // Сохраняем результат этого шага в `data`.
        let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
        lesson_trace::trace_step!(data);
        // Сохраняем рассчитанное значение `(mut weight, mut bias)` для следующих операций.
        let (mut weight, mut bias): (f64, f64) = (0., 0.);
        lesson_trace::trace_step!(weight);
        lesson_trace::trace_step!(bias);
        // 3000 обновлений дают этой маленькой задаче линейной регрессии время сойтись.
        // Это число итераций учебного обучения, а не число строк данных.
        for _ in 0..3000 {
            // Выполняем встроенный расчёт один раз и сохраняем результат в `(weight_gradient, bias_gradient)`.
            // Производную функции по параметру или вектор таких производных называют gradient.
            let (weight_loss_rate_of_change, bias_loss_rate_of_change): (f64, f64) =
                (|| -> (f64, f64) {
                    // Вычисляем значение по указанной формуле.
                    /* Частные производные MSE по весу и смещению: 2/n * sum(error*x) и 2/n * sum(error). */
                    // Сохраняем результат этого шага в `data`.
                    let data: &[(f64, f64)] = data;
                    lesson_trace::trace_step!(data);
                    // Сохраняем рассчитанное значение `weight` для следующих операций.
                    let weight: f64 = weight;
                    lesson_trace::trace_step!(weight);
                    // Сохраняем рассчитанное значение `bias` для следующих операций.
                    let bias: f64 = bias;
                    lesson_trace::trace_step!(bias);
                    // Сохраняем рассчитанное значение `(mut weight_gradient_sum, mut bias_gradient_sum)` для следующих операций.
                    let (
                        mut accumulated_weight_loss_rate_of_change,
                        mut accumulated_bias_loss_rate_of_change,
                    ): (f64, f64) = (0.0, 0.0);
                    lesson_trace::trace_step!(accumulated_weight_loss_rate_of_change);
                    lesson_trace::trace_step!(accumulated_bias_loss_rate_of_change);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for &(feature_value, target_value) in data {
                        lesson_trace::trace_step!(feature_value);
                        lesson_trace::trace_step!(target_value);
                        // Умножаем значения и сохраняем результат в `prediction_error`.
                        let prediction_error: f64 = weight * feature_value + bias - target_value;
                        lesson_trace::trace_step!(prediction_error);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        accumulated_weight_loss_rate_of_change +=
                            2.0 * feature_value * prediction_error;
                        lesson_trace::trace_step!(accumulated_weight_loss_rate_of_change);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        accumulated_bias_loss_rate_of_change += 2.0 * prediction_error;
                        lesson_trace::trace_step!(accumulated_bias_loss_rate_of_change);
                    }
                    // Составляем результат из вычисленных значений в указанном порядке.
                    (
                        // Делим значения, получая нормированную величину или среднее.
                        accumulated_weight_loss_rate_of_change / data.len() as f64,
                        // Делим значения, получая нормированную величину или среднее.
                        accumulated_bias_loss_rate_of_change / data.len() as f64,
                    )
                })();
            lesson_trace::trace_step!(weight_loss_rate_of_change);
            lesson_trace::trace_step!(bias_loss_rate_of_change);
            // Вычитаем очередной вклад из текущего значения параметра.
            // 0.02 — скорость обучения: вычитаем только 2% рассчитанного градиента веса.
            weight -= 0.02 * weight_loss_rate_of_change;
            lesson_trace::trace_step!(weight);
            // Вычитаем очередной вклад из текущего значения параметра.
            bias -= 0.02 * bias_loss_rate_of_change;
            lesson_trace::trace_step!(bias);
        }
        // Составляем результат из вычисленных значений в указанном порядке.
        (weight, bias)
    })();
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_step!(bias);
    // Шаг: Отдельно задаём примеры, которые не участвовали в обучении.
    let test: [(f64, f64); 2] = [(5., 11.), (6., 13.)];
    lesson_trace::trace_step!(test);

    // Собираем значения для `targets` в коллекцию.
    let targets: Vec<f64> = test.iter().map(|&(_, target)| target).collect();
    lesson_trace::trace_step!(targets);
    // Собираем значения для `model_predictions` в коллекцию.
    let model_predictions: Vec<f64> = test
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Преобразуем каждый элемент в новое значение.
        .map(|&(feature, _)| weight * feature + bias)
        // Собираем результаты в коллекцию.
        .collect();
    lesson_trace::trace_step!(model_predictions);
    // Собираем значения для `baseline_predictions` в коллекцию.
    let baseline_predictions: Vec<f64> = test.iter().map(|_| 5.0).collect();
    lesson_trace::trace_step!(baseline_predictions);
    // Сохраняем результат этого шага в `model_mean_squared_error`.
    let model_mean_squared_error: f64 =
        part_049_lesson_09_mean_squared_error_between_targets_and_predictions::mean_squared_error_between_targets_and_predictions(&targets, &model_predictions)
            .unwrap();
    lesson_trace::trace_step!(model_mean_squared_error);
    // Сохраняем результат этого шага в `baseline_mean_squared_error`.
    let baseline_mean_squared_error: f64 =
        part_049_lesson_09_mean_squared_error_between_targets_and_predictions::mean_squared_error_between_targets_and_predictions(&targets, &baseline_predictions)
            .unwrap();
    lesson_trace::trace_step!(baseline_mean_squared_error);
    // Печатаем рассчитанные значения для проверки примера.
    println!(
        "w={weight:.3}, b={bias:.3}, test MSE={model_mean_squared_error:.6}, baseline MSE={baseline_mean_squared_error:.3}"
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_linear_regression_with_errors_coefficients_and_heldout_evaluation(
        weight, bias, test,
    );
}

// Строим график по результатам урока.
fn visualize_practice_linear_regression_with_errors_coefficients_and_heldout_evaluation(
    weight: f64,
    bias: f64,
    test: [(f64, f64); 2],
) {
    // График величин и зависимостей, изученных в этом уроке.
    let model_points: Vec<(f64, f64)> = (0..=60)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, weight * horizontal_value + bias)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `test_points` в коллекцию.
    let test_points: Vec<(f64, f64)> = test
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Линейная регрессия и тест",
        // Указываем подпись горизонтальной оси.
        "признак x",
        // Указываем подпись вертикальной оси.
        "целевое значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "модель",
                // Передаём рассчитанные координаты точек.
                points: &model_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "тест",
                // Передаём рассчитанные координаты точек.
                points: &test_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
