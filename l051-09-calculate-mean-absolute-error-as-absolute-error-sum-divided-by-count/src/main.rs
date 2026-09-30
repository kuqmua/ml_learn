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

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `targets`.");
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    lesson_trace::trace_step!(targets);
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64); 4] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка выше ответа", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка ниже ответа", &[2.0, 3.0, 6.0], 1.0 / 3.0),
        ("ошибка вдвое больше", &[2.0, 6.0, 6.0], 2.0 / 3.0),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, predictions, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(predictions);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `mean_absolute_error_value`.");
        lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let mean_absolute_error_value: f64 =
            l051_09_calculate_mean_absolute_error_as_absolute_error_sum_divided_by_count::calculate_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(&targets, predictions)

                .expect("у каждого прогноза есть правильный ответ");
        lesson_trace::trace_step!(mean_absolute_error_value);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((mean_absolute_error_value - expected).abs() < 1e-10);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {predictions:?} → MAE {mean_absolute_error_value:.3}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_average_absolute_prediction_error_for_changing_offset();
}

// Строим график по результатам урока.
fn plot_average_absolute_prediction_error_for_changing_offset() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let mean_absolute_error_points: Vec<(f64, f64)> = (-30..=30)

        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `prediction_difference`.");
            let prediction_difference: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
            lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
            lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
            lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
            lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
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
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
