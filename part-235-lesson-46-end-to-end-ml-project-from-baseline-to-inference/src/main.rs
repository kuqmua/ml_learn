// Итоговый проект 35. Сквозной ML проект от базовой модели до инференса.
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
fn main() {
    lesson_trace::enable();
    // Фиксируем демонстрационные данные на время выполнения программы.
    const EXAMPLE_DATA: [(f64, f64); 10] = [
        // Учебная пара: вход 0., ожидаемое значение 1..
        (0., 1.),
        // Учебная пара: вход 1., ожидаемое значение 3..
        (1., 3.),
        // Учебная пара: вход 2., ожидаемое значение 5..
        (2., 5.),
        // Учебная пара: вход 3., ожидаемое значение 7..
        (3., 7.),
        // Учебная пара: вход 4., ожидаемое значение 9..
        (4., 9.),
        // Учебная пара: вход 5., ожидаемое значение 11..
        (5., 11.),
        // Учебная пара: вход 6., ожидаемое значение 13..
        (6., 13.),
        // Учебная пара: вход 7., ожидаемое значение 15..
        (7., 15.),
        // Учебная пара: вход 8., ожидаемое значение 17..
        (8., 17.),
        // Учебная пара: вход 9., ожидаемое значение 19..
        (9., 19.),
    ];

    // Шаг: Делим данные на train, validation и test.
    assert!(
        // Обновляем значение результатом текущего вычисления.
        EXAMPLE_DATA.len() >= 9,
        // Передаём подпись или текстовое значение для следующего шага.
        "для разделения нужны train, validation и test"
    );
    // Сохраняем результат этого шага в `training_examples`.
    let training_examples: &[(f64, f64)] = &EXAMPLE_DATA[..6];
    lesson_trace::trace_step!(training_examples);
    // Сохраняем рассчитанное значение `validation` для следующих операций.
    let validation: &[(f64, f64)] = &EXAMPLE_DATA[6..8];
    lesson_trace::trace_step!(validation);
    // Сохраняем рассчитанное значение `test` для следующих операций.
    let test: &[(f64, f64)] = &EXAMPLE_DATA[8..];
    lesson_trace::trace_step!(test);

    // Шаг: Считаем константный прогноз только по train.
    let training_targets: Vec<f64> = training_examples
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Преобразуем каждый элемент в новое значение.
        .map(|&(_, target)| target)
        // Собираем результаты в коллекцию.
        .collect();
    lesson_trace::trace_step!(training_targets);
    // Сохраняем результат этого шага в `baseline`.
    let baseline: f64 =
        part_029_lesson_06_arithmetic_mean_of_numeric_values::arithmetic_mean_of_numeric_values(
            &training_targets,
        )
        .unwrap();
    lesson_trace::trace_step!(baseline);

    // Учебные реализации математических операций для этого урока.

    // Минимальный pipeline: данные -> split -> baseline -> обучение -> test -> инференс.

    // Объявляем повторно используемое вычисление `calculate_mean_absolute_prediction_error`; параметры ниже задают его входы.
    fn calculate_mean_absolute_error_of_linear_model_on_data(
        // Получаем набор наблюдений, по которому считаем ошибку или градиент.
        data: &[(f64, f64)],
        // Параметр `weight` передаёт коэффициент при признаке.
        weight: f64,
        // Параметр `bias` передаёт свободный член модели.
        bias: f64,
        // Указываем тип возвращаемого значения.
    ) -> f64 {
        // Собираем значения для `targets` в коллекцию.
        let targets: Vec<f64> = data.iter().map(|&(_, target)| target).collect();
        lesson_trace::trace_step!(targets);
        // Собираем значения для `predictions` в коллекцию.
        let predictions: Vec<f64> = data
            // Просматриваем элементы коллекции по ссылке.
            .iter()
            // Преобразуем каждый элемент в новое значение.
            .map(|&(feature, _)| weight * feature + bias)
            // Собираем результаты в коллекцию.
            .collect();
        lesson_trace::trace_step!(predictions);
        // Используем подготовленное значение в следующем шаге примера.
        part_050_lesson_09_mean_absolute_error_between_targets_and_predictions::mean_absolute_error_between_targets_and_predictions(&targets, &predictions).unwrap()
    }

    // Шаг: Обучаем линейную модель и сравниваем её с baseline на validation и test.
    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        // Используем подготовленное значение в следующем шаге примера.
        /* Оцениваем коэффициенты прямой по ковариации и дисперсии обучающего признака. */
        // Сохраняем результат этого шага в `data`.
        let data: &[(f64, f64)] = training_examples;
        lesson_trace::trace_step!(data);
        // Считаем количество элементов и сохраняем его в `sample_count`.
        let sample_count: f64 = data.len() as f64;
        lesson_trace::trace_step!(sample_count);
        // Сохраняем рассчитанное значение `(mut feature_sum, mut target_sum)` для следующих операций.
        let (mut feature_sum, mut target_sum): (f64, f64) = (0.0, 0.0);
        lesson_trace::trace_step!(feature_sum);
        lesson_trace::trace_step!(target_sum);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for &(feature_value, target_value) in data {
            lesson_trace::trace_step!(feature_value);
            lesson_trace::trace_step!(target_value);
            // Прибавляем очередной вклад к ранее накопленному результату.
            feature_sum += feature_value;
            lesson_trace::trace_step!(feature_sum);
            // Прибавляем очередной вклад к ранее накопленному результату.
            target_sum += target_value;
            lesson_trace::trace_step!(target_sum);
        }
        // Нормируем или усредняем величину делением и сохраняем её в `mean_feature`.
        let mean_feature: f64 = feature_sum / sample_count;
        lesson_trace::trace_step!(mean_feature);
        // Нормируем или усредняем величину делением и сохраняем её в `mean_target`.
        let mean_target: f64 = target_sum / sample_count;
        lesson_trace::trace_step!(mean_target);
        // Сохраняем рассчитанное значение `(mut covariance_sum, mut variance_sum)` для следующих операций.
        // Совместное изменение двух величин описывают через covariance.
        let (mut joint_deviation_product_sum, mut variance_sum): (f64, f64) = (0.0, 0.0);
        lesson_trace::trace_step!(joint_deviation_product_sum);
        lesson_trace::trace_step!(variance_sum);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for &(feature_value, target_value) in data {
            lesson_trace::trace_step!(feature_value);
            lesson_trace::trace_step!(target_value);
            // Прибавляем очередной вклад к ранее накопленному результату.
            joint_deviation_product_sum +=
                (feature_value - mean_feature) * (target_value - mean_target);
            lesson_trace::trace_step!(joint_deviation_product_sum);
            // Прибавляем очередной вклад к ранее накопленному результату.
            variance_sum += (|| -> f64 {
                // Используем подготовленное значение в следующем шаге примера.
                /* Возводим число в квадрат обычным умножением. */
                // Сохраняем результат этого шага в `value`.
                let value: f64 = feature_value - mean_feature;
                lesson_trace::trace_step!(value);
                // Умножаем величины согласно используемой формуле.
                value * value
            })();
            lesson_trace::trace_step!(variance_sum);
        }
        // Нормируем или усредняем величину делением и сохраняем её в `weight`.
        let weight: f64 = joint_deviation_product_sum / variance_sum;
        lesson_trace::trace_step!(weight);
        // Составляем результат из вычисленных значений в указанном порядке.
        (weight, mean_target - weight * mean_feature)
    })();
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_step!(bias);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "baseline validation MAE={:.3}, model validation MAE={:.3}, test MAE={:.3}, prediction x=10: {:.3}",
        // Вызываем нужное вычисление с подготовленными аргументами.
        calculate_mean_absolute_error_of_linear_model_on_data(validation, 0., baseline),
        // Вызываем нужное вычисление с подготовленными аргументами.
        calculate_mean_absolute_error_of_linear_model_on_data(validation, weight, bias),
        // Вызываем нужное вычисление с подготовленными аргументами.
        calculate_mean_absolute_error_of_linear_model_on_data(test, weight, bias),
        // Умножаем величины согласно используемой формуле.
        weight * 10. + bias
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_end_to_end_ml_project_from_baseline_to_inference(training_examples, weight, bias);
}

// Строим график по результатам урока.
fn visualize_end_to_end_ml_project_from_baseline_to_inference(
    training_examples: &[(f64, f64)],
    weight: f64,
    bias: f64,
) {
    // Значения из этого урока на графике.
    let training_points: Vec<(f64, f64)> = training_examples
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    // Собираем значения для `model_points` в коллекцию.
    let model_points: Vec<(f64, f64)> = (0..=80)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, weight * horizontal_value + bias)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Регрессия: данные и модель",
        // Указываем подпись горизонтальной оси.
        "признак",
        // Указываем подпись вертикальной оси.
        "целевое значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "train",
                // Передаём рассчитанные координаты точек.
                points: &training_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "модель",
                // Передаём рассчитанные координаты точек.
                points: &model_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
