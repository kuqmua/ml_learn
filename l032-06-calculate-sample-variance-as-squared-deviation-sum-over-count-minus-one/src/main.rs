// Урок 06.3. Разброс значений (выборочная дисперсия): сумма квадратов отклонений от среднего, делённая на число значений минус один.
// Связь с принятой терминологией: Выборочная дисперсия числовых значений.
// Зачем здесь эта тема: Центр не описывает разброс; дисперсия измеряет средний квадрат отклонений
//   от центра.
// Почему код устроен так: Вычитаем выборочное среднее и делим на n−1, поскольку оцениваем разброс
//   совокупности по выборке.
// Представь: Наборы [4, 4, 4] и [2, 4, 6] имеют одно среднее, но второй заметно сильнее разбросан.
//
// Общая функция использует среднее из урока 06.1. При одинаковых значениях разброс равен нулю;
// для выборочной оценки нужны хотя бы два значения.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64); 3] = [
        ("все значения одинаковы", &[4.0, 4.0, 4.0], 0.0),
        ("умеренный разброс", &[2.0, 4.0, 6.0], 4.0),
        ("значения раздвинули", &[0.0, 4.0, 8.0], 16.0),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, values, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(values);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `variance`.");
        lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let variance: f64 = l032_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(values)

            .expect("для этой выборки дисперсия определена");
        lesson_trace::trace_step!(variance);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(variance, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {values:?} → дисперсия {variance}");
    }
    lesson_trace::trace_note!("Сохраняем результат этого шага в `error`.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let error: &str =
        l032_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(&[
            4.0,
        ])

        .expect_err("одного значения недостаточно");
    lesson_trace::trace_step!(error);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("одно значение: {error}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_squared_deviations_from_mean();
}

// Строим график по результатам урока.
fn plot_squared_deviations_from_mean() {
    lesson_trace::trace_note!("Наглядное представление величин из этого урока.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let variance_points: Vec<(f64, f64)> = (0..=80)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                horizontal_value,
                (horizontal_value - 4.0) * (horizontal_value - 4.0),
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
        "Разброс относительно среднего",
        "значение",
        "квадрат отклонения",
        &[lesson_visualization::Series {
            name: "среднее=4",

            points: &variance_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
