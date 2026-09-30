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
use l051_09_calculate_mean_absolute_error_as_absolute_error_sum_divided_by_count::calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count;

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    trace_note!("Учебная пара: вход 0., ожидаемое значение 1..");
    trace_note!("Учебная пара: вход 1., ожидаемое значение 3..");
    trace_note!("Учебная пара: вход 2., ожидаемое значение 5..");
    trace_note!("Учебная пара: вход 3., ожидаемое значение 7..");
    trace_note!("Учебная пара: вход 4., ожидаемое значение 9..");
    trace_note!("Учебная пара: вход 5., ожидаемое значение 11..");
    trace_note!("Учебная пара: вход 6., ожидаемое значение 13..");
    trace_note!("Учебная пара: вход 7., ожидаемое значение 15..");
    trace_note!("Учебная пара: вход 8., ожидаемое значение 17..");
    trace_note!("Учебная пара: вход 9., ожидаемое значение 19..");
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

    trace_note!("Шаг: Делим данные на train, validation и test.");
    trace_note!("Обновляем значение результатом текущего вычисления.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        EXAMPLE_DATA.len() >= 9,
        "для разделения нужны train, validation и test"
    );
    trace_note!("Сохраняем результат этого шага в `training_examples`.");
    let training_examples: &[(f64, f64)] = &EXAMPLE_DATA[..6];
    trace_step!(training_examples);
    trace_note!("Сохраняем рассчитанное значение `validation` для следующих операций.");
    let validation: &[(f64, f64)] = &EXAMPLE_DATA[6..8];
    trace_step!(validation);
    trace_note!("Сохраняем рассчитанное значение `test` для следующих операций.");
    let test: &[(f64, f64)] = &EXAMPLE_DATA[8..];
    trace_step!(test);

    trace_note!("Шаг: Считаем константный прогноз только по train.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let training_targets: Vec<f64> = training_examples
        .iter()
        .map(|&(_, target)| target)
        .collect();
    trace_step!(training_targets);
    trace_note!("Сохраняем результат этого шага в `baseline`.");
    let baseline: f64 =
        calculate_mean_by_summing_values_and_dividing_by_count(&training_targets).unwrap();
    trace_step!(baseline);

    trace_note!("Учебные реализации математических операций для этого урока.");

    trace_note!(
        "Минимальный pipeline: данные -> split -> baseline -> обучение -> test -> инференс."
    );

    trace_note!(
        "Объявляем повторно используемое вычисление `calculate_mean_absolute_prediction_error`; параметры ниже задают его входы."
    );
    /// Средняя абсолютная ошибка линейной модели: для каждого x считаем weight·x+bias, сравниваем с ответом и усредняем модули ошибок.
    fn calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets(
        data: &[(f64, f64)],

        weight: f64,

        bias: f64,
    ) -> f64 {
        trace_note!("Получаем набор наблюдений, по которому считаем ошибку или градиент.");
        trace_note!("Параметр `weight` передаёт коэффициент при признаке.");
        trace_note!("Параметр `bias` передаёт свободный член модели.");
        trace_note!("Указываем тип возвращаемого значения.");
        trace_note!("Собираем значения для `targets` в коллекцию.");
        let targets: Vec<f64> = data.iter().map(|&(_, target)| target).collect();
        trace_step!(targets);
        trace_note!("Собираем значения для `predictions` в коллекцию.");
        trace_note!("Просматриваем элементы коллекции по ссылке.");
        trace_note!("Преобразуем каждый элемент в новое значение.");
        trace_note!("Собираем результаты в коллекцию.");
        let predictions: Vec<f64> = data
            .iter()
            .map(|&(feature, _)| weight * feature + bias)
            .collect();
        trace_step!(predictions);
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(
            &targets,
            &predictions,
        )
        .unwrap()
    }

    trace_note!("Шаг: Обучаем линейную модель и сравниваем её с baseline на validation и test.");
    let (weight, bias): (f64, f64) = (|| -> (f64, f64) {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Оцениваем коэффициенты прямой по ковариации и дисперсии обучающего признака.");
        trace_note!("Сохраняем результат этого шага в `data`.");
        let data: &[(f64, f64)] = training_examples;
        trace_step!(data);
        trace_note!("Считаем количество элементов и сохраняем его в `sample_count`.");
        let sample_count: f64 = data.len() as f64;
        trace_step!(sample_count);
        trace_note!(
            "Сохраняем рассчитанное значение `(mut feature_sum, mut target_sum)` для следующих операций."
        );
        let (mut feature_sum, mut target_sum): (f64, f64) = (0.0, 0.0);
        trace_step!(feature_sum);
        trace_step!(target_sum);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for &(feature_value, target_value) in data {
            trace_step!(feature_value);
            trace_step!(target_value);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            feature_sum += feature_value;
            trace_step!(feature_sum);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            target_sum += target_value;
            trace_step!(target_sum);
        }
        trace_note!("Нормируем или усредняем величину делением и сохраняем её в `mean_feature`.");
        let mean_feature: f64 = feature_sum / sample_count;
        trace_step!(mean_feature);
        trace_note!("Нормируем или усредняем величину делением и сохраняем её в `mean_target`.");
        let mean_target: f64 = target_sum / sample_count;
        trace_step!(mean_target);
        trace_note!(
            "Сохраняем рассчитанное значение `(mut covariance_sum, mut variance_sum)` для следующих операций."
        );
        trace_note!("Совместное изменение двух величин описывают через covariance.");
        let (mut joint_deviation_product_sum, mut variance_sum): (f64, f64) = (0.0, 0.0);
        trace_step!(joint_deviation_product_sum);
        trace_step!(variance_sum);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for &(feature_value, target_value) in data {
            trace_step!(feature_value);
            trace_step!(target_value);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            joint_deviation_product_sum +=
                (feature_value - mean_feature) * (target_value - mean_target);
            trace_step!(joint_deviation_product_sum);
            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
            variance_sum += (|| -> f64 {
                trace_note!("Используем подготовленное значение в следующем шаге примера.");
                trace_note!("Возводим число в квадрат обычным умножением.");
                trace_note!("Сохраняем результат этого шага в `value`.");
                let value: f64 = feature_value - mean_feature;
                trace_step!(value);
                trace_note!("Умножаем величины согласно используемой формуле.");
                value * value
            })();
            trace_step!(variance_sum);
        }
        trace_note!("Нормируем или усредняем величину делением и сохраняем её в `weight`.");
        let weight: f64 = joint_deviation_product_sum / variance_sum;
        trace_step!(weight);
        trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
        (weight, mean_target - weight * mean_feature)
    })();
    trace_step!(weight);
    trace_step!(bias);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    trace_note!("Умножаем величины согласно используемой формуле.");
    println!(
        "baseline validation MAE={:.3}, model validation MAE={:.3}, test MAE={:.3}, prediction x=10: {:.3}",
        calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets(
            validation, 0., baseline
        ),
        calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets(
            validation, weight, bias
        ),
        calculate_linear_model_error_as_average_absolute_difference_between_predictions_and_targets(
            test, weight, bias
        ),
        weight * 10. + bias
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_training_points_and_fitted_prediction_line(training_examples, weight, bias);
}

// Строим график по результатам урока.
fn plot_training_points_and_fitted_prediction_line(
    training_examples: &[(f64, f64)],
    weight: f64,
    bias: f64,
) {
    trace_note!("Значения из этого урока на графике.");
    let training_points: Vec<(f64, f64)> = training_examples
        .iter()
        .map(|&(horizontal_value, vertical_value)| (horizontal_value, vertical_value))
        .collect();
    trace_note!("Собираем значения для `model_points` в коллекцию.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let model_points: Vec<(f64, f64)> = (0..=80)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, weight * horizontal_value + bias)
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Регрессия: данные и модель",
        "признак",
        "целевое значение",
        &[
            lesson_visualization::Series {
                name: "train",

                points: &training_points,
            },
            lesson_visualization::Series {
                name: "модель",

                points: &model_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
