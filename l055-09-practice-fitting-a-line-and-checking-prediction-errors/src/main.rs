// Урок 09.6. Практика: подбор прямой и проверка ошибок прогноза.
// Связь с принятой терминологией: Линейная регрессия, ошибки и оценка на отложенных данных.
// Зачем здесь эта тема: Теперь можно проследить полный путь регрессии от прогноза до честной
//   оценки.
// Почему код устроен так: На одних данных соединяем веса, ошибку, ограничение и отложенную
//   проверку.
// Представь: Сначала получаем прогнозы линейной модели, затем считаем ошибку и сравниваем с простым
//   ответом на новых данных.
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
    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    const TRAINING_EXAMPLES: [(f64, f64); 5] = [(0., 1.), (1., 3.), (2., 5.), (3., 7.), (4., 9.)];
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !TRAINING_EXAMPLES.is_empty(),
        "для обучения нужен хотя бы один пример"
    );

    lesson_trace::trace_note!("Шаг: Обучаем коэффициенты линейной модели на train.");
    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!(
            "Обновляем вес и смещение по среднему градиенту квадратичной ошибки."
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
        let data: &[(f64, f64)] = &TRAINING_EXAMPLES;
        lesson_trace::trace_step!(data);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `(mut weight, mut bias)` для следующих операций."
        );
        let (mut weight, mut bias): (f64, f64) = (0., 0.);
        lesson_trace::trace_step!(weight);
        lesson_trace::trace_step!(bias);
        lesson_trace::trace_note!(
            "3000 обновлений дают этой маленькой задаче линейной регрессии время сойтись."
        );
        lesson_trace::trace_note!("Это число итераций учебного обучения, а не число строк данных.");
        for _ in 0..3000 {
            lesson_trace::trace_note!(
                "Выполняем встроенный расчёт один раз и сохраняем результат в `(weight_gradient, bias_gradient)`."
            );
            lesson_trace::trace_note!(
                "Производную функции по параметру или вектор таких производных называют gradient."
            );
            let (weight_loss_rate_of_change, bias_loss_rate_of_change): (f64, f64) =
                (|| -> (f64, f64) {
                    lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
                    lesson_trace::trace_note!(
                        "Частные производные MSE по весу и смещению: 2/n * sum(error*x) и 2/n * sum(error)."
                    );
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `data`.");
                    let data: &[(f64, f64)] = data;
                    lesson_trace::trace_step!(data);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `weight` для следующих операций."
                    );
                    let weight: f64 = weight;
                    lesson_trace::trace_step!(weight);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `bias` для следующих операций."
                    );
                    let bias: f64 = bias;
                    lesson_trace::trace_step!(bias);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `(mut weight_gradient_sum, mut bias_gradient_sum)` для следующих операций."
                    );
                    let (
                        mut accumulated_weight_loss_rate_of_change,
                        mut accumulated_bias_loss_rate_of_change,
                    ): (f64, f64) = (0.0, 0.0);
                    lesson_trace::trace_step!(accumulated_weight_loss_rate_of_change);
                    lesson_trace::trace_step!(accumulated_bias_loss_rate_of_change);
                    lesson_trace::trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for &(feature_value, target_value) in data {
                        lesson_trace::trace_step!(feature_value);
                        lesson_trace::trace_step!(target_value);
                        lesson_trace::trace_note!(
                            "Умножаем значения и сохраняем результат в `prediction_error`."
                        );
                        let prediction_error: f64 = weight * feature_value + bias - target_value;
                        lesson_trace::trace_step!(prediction_error);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        accumulated_weight_loss_rate_of_change +=
                            2.0 * feature_value * prediction_error;
                        lesson_trace::trace_step!(accumulated_weight_loss_rate_of_change);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        accumulated_bias_loss_rate_of_change += 2.0 * prediction_error;
                        lesson_trace::trace_step!(accumulated_bias_loss_rate_of_change);
                    }
                    lesson_trace::trace_note!(
                        "Составляем результат из вычисленных значений в указанном порядке."
                    );
                    lesson_trace::trace_note!(
                        "Делим значения, получая нормированную величину или среднее."
                    );
                    lesson_trace::trace_note!(
                        "Делим значения, получая нормированную величину или среднее."
                    );
                    (
                        accumulated_weight_loss_rate_of_change / data.len() as f64,
                        accumulated_bias_loss_rate_of_change / data.len() as f64,
                    )
                })();
            lesson_trace::trace_step!(weight_loss_rate_of_change);
            lesson_trace::trace_step!(bias_loss_rate_of_change);
            lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
            lesson_trace::trace_note!(
                "0.02 — скорость обучения: вычитаем только 2% рассчитанного градиента веса."
            );
            weight -= 0.02 * weight_loss_rate_of_change;
            lesson_trace::trace_step!(weight);
            lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
            bias -= 0.02 * bias_loss_rate_of_change;
            lesson_trace::trace_step!(bias);
        }
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        (weight, bias)
    })();
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_step!(bias);
    lesson_trace::trace_note!("Шаг: Отдельно задаём примеры, которые не участвовали в обучении.");
    let test: [(f64, f64); 2] = [(5., 11.), (6., 13.)];
    lesson_trace::trace_step!(test);

    lesson_trace::trace_note!("Собираем значения для `targets` в коллекцию.");
    let targets: Vec<f64> = test.iter().map(|&(_, target)| target).collect();
    lesson_trace::trace_step!(targets);
    lesson_trace::trace_note!("Собираем значения для `model_predictions` в коллекцию.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let model_predictions: Vec<f64> = test
        .iter()
        .map(|&(feature, _)| weight * feature + bias)
        .collect();
    lesson_trace::trace_step!(model_predictions);
    lesson_trace::trace_note!("Собираем значения для `baseline_predictions` в коллекцию.");
    let baseline_predictions: Vec<f64> = test.iter().map(|_| 5.0).collect();
    lesson_trace::trace_step!(baseline_predictions);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `model_mean_squared_error`.");
    let model_mean_squared_error: f64 =
        l050_09_calculate_mean_squared_error_as_squared_error_sum_divided_by_count::calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(&targets, &model_predictions)
            .unwrap();
    lesson_trace::trace_step!(model_mean_squared_error);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `baseline_mean_squared_error`.");
    let baseline_mean_squared_error: f64 =
        l050_09_calculate_mean_squared_error_as_squared_error_sum_divided_by_count::calculate_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(&targets, &baseline_predictions)
            .unwrap();
    lesson_trace::trace_step!(baseline_mean_squared_error);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!(
        "w={weight:.3}, b={bias:.3}, test MSE={model_mean_squared_error:.6}, baseline MSE={baseline_mean_squared_error:.3}"
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_prediction_line_and_held_out_points(weight, bias, test);
}

// Строим график по результатам урока.
fn plot_prediction_line_and_held_out_points(weight: f64, bias: f64, test: [(f64, f64); 2]) {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let model_points: Vec<(f64, f64)> = (0..=60)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (horizontal_value, weight * horizontal_value + bias)
        })
        .collect();
    lesson_trace::trace_note!("Собираем значения для `test_points` в коллекцию.");
    let test_points: Vec<(f64, f64)> = test
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Линейная регрессия и тест",
        "признак x",
        "целевое значение",
        &[
            lesson_visualization::Series {
                name: "модель",

                points: &model_points,
            },
            lesson_visualization::Series {
                name: "тест",

                points: &test_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
