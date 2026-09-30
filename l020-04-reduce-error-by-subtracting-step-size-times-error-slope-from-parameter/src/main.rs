// Урок 04.1. Шаг уменьшения ошибки: вычитание из параметра произведения размера шага и скорости изменения ошибки.
// Связь с принятой терминологией: Скорость обучения в шаге градиентного спуска.
// Зачем здесь эта тема: Градиент показывает направление, но не размер шага; его задаёт скорость
//   обучения.
// Почему код устроен так: Сравниваем обновления одного параметра при разных коэффициентах, чтобы
//   увидеть перескок и медленное движение.
// Представь: Большой шаг может перепрыгнуть минимум ошибки, а слишком маленький потребует много
//   повторений.
//
// Что изучаем: Скорость обучения.
// Зачем это нужно: Один и тот же градиент даёт разные шаги при разных learning rate. Слишком большой шаг
// может перескочить минимум.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Инициализируем значение `parameter` начальным состоянием.");
    let parameter: f64 = 0.0;
    lesson_trace::trace_step!(parameter);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `rate_of_change`.");
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    let rate_of_change: f64 = 2.0 * (parameter - 3.0);
    lesson_trace::trace_step!(rate_of_change);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for rate in [0.1, 1.0, 2.0] {
        lesson_trace::trace_step!(rate);
        lesson_trace::trace_note!(
            "Делаем ровно одно обновление, чтобы изолировать влияние скорости."
        );
        let updated: f64 = parameter - rate * rate_of_change;
        lesson_trace::trace_step!(updated);
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `error`.");
        let error: f64 = (updated - 3.0) * (updated - 3.0);
        lesson_trace::trace_step!(error);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("rate={rate}: параметр={updated}, ошибка={error}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_squared_error_after_one_update_for_different_step_sizes(parameter, rate_of_change);
}

// Строим график по результатам урока.
fn plot_squared_error_after_one_update_for_different_step_sizes(
    parameter: f64,
    rate_of_change: f64,
) {
    lesson_trace::trace_note!("Задаём учебные значения для `learning_rates`.");
    let learning_rates: [f64; 3] = [0.1, 1.0, 2.0];
    lesson_trace::trace_note!("Собираем значения для `errors` в коллекцию.");
    lesson_trace::trace_note!("Передаём элементы коллекции в итератор.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let errors: Vec<(f64, f64)> = learning_rates
        .into_iter()
        .map(|rate| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `updated`.");
            let updated: f64 = parameter - rate * rate_of_change;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (rate, (updated - 3.0) * (updated - 3.0))
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
        "learning-rate",
        "Ошибка после одного шага",
        "Скорость обучения",
        "Квадратичная ошибка",
        &[lesson_visualization::Series {
            name: "Ошибка",

            points: &errors,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
