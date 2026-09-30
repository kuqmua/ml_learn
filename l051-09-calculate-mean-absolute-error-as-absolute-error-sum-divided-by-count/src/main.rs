// Урок 09.2. Средняя абсолютная ошибка прогноза: сумма модулей ошибок, делённая на число примеров.
// Связь с принятой терминологией: Средний модуль разности правильных ответов и прогнозов.
// Зачем здесь эта тема: Квадратная ошибка подчёркивает выбросы; абсолютная показывает среднюю
//   величину промаха в исходных единицах.
// Почему код устроен так: Берём модули тех же разностей и сравниваем обе метрики на одинаковых
//   примерах.
// Представь: Для промахов на 3 и на 1 абсолютные вклады равны 3 и 1; крупный промах здесь не
//   возводится в квадрат.
//
// Ошибки разных знаков не сокращаются; удвоение промаха удваивает вклад в MAE.
// Общая библиотека проверяет, что у каждого прогноза есть правильный ответ.

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `targets`.");
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    trace_step!(targets);
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64); 4] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка выше ответа", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка ниже ответа", &[2.0, 3.0, 6.0], 1.0 / 3.0),
        ("ошибка вдвое больше", &[2.0, 6.0, 6.0], 2.0 / 3.0),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, predictions, expected) in cases {
        trace_step!(description);
        trace_step!(predictions);
        trace_step!(expected);
        trace_note!("Сохраняем результат этого шага в `mean_absolute_error_value`.");
        trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let mean_absolute_error_value: f64 =
            l051_09_calculate_mean_absolute_error_as_absolute_error_sum_divided_by_count::calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(&targets, predictions)

                .expect("у каждого прогноза есть правильный ответ");
        trace_step!(mean_absolute_error_value);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((mean_absolute_error_value - expected).abs() < 1e-10);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {predictions:?} → MAE {mean_absolute_error_value:.3}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_average_absolute_prediction_error_for_changing_offset();
}

// Строим график по результатам урока.
fn plot_average_absolute_prediction_error_for_changing_offset() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let mean_absolute_error_points: Vec<(f64, f64)> = (-30..=30)

        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `prediction_difference`.");
            let prediction_difference: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!("Передаём ряды или значения для отрисовки графика.");
            trace_note!("Передаём ряды или значения для отрисовки графика.");
            trace_note!("Используем результат, ожидая успешного выполнения шага.");
            (

                prediction_difference,

                l051_09_calculate_mean_absolute_error_as_absolute_error_sum_divided_by_count::calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(

                    &[2.0, 4.0, 6.0],

                    &[
                        2.0 + prediction_difference,
                        4.0 + prediction_difference,
                        6.0 + prediction_difference,
                    ],
                )

                .unwrap(),
            )
        })

        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Средняя абсолютная ошибка",
        "смещение прогноза",
        "MAE",
        &[lesson_visualization::Series {
            name: "цели [2,4,6]",

            points: &mean_absolute_error_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
