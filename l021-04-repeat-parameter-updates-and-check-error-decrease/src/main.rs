// Урок 04.2. Повторение обновлений параметра и проверка уменьшения ошибки.
// Связь с принятой терминологией: Сходимость последовательности шагов градиентного спуска.
// Зачем здесь эта тема: Один удачный шаг ещё не означает обучение; важна последовательность ошибок
//   и параметров.
// Почему код устроен так: Повторяем одинаковое правило обновления, отслеживая приближение к
//   минимуму.
// Представь: Если ошибка после каждого шага снижается, видно движение к минимуму; один шаг этого
//   ещё не показывает.
//
// Что изучаем: Сходимость градиентного спуска.
// Зачем это нужно: Последовательность шагов должна приближаться к минимуму. Останавливаемся по малому
// градиенту, а не только по числу эпох.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `parameter` начальным состоянием."
    );
    let mut parameter: f64 = 0.0;
    lesson_trace::trace_step!(parameter);
    lesson_trace::trace_note!(
        "Скорость 0.2 уменьшает расстояние до минимума x=3 на каждом шаге этой параболы."
    );
    let rate: f64 = 0.2;
    lesson_trace::trace_step!(rate);
    lesson_trace::trace_note!(
        "100 — страховочный предел шагов; обычно остановимся раньше, когда |градиент| < 10⁻⁶."
    );
    for epoch in 0..100 {
        lesson_trace::trace_step!(epoch);
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `rate_of_change`.");
        lesson_trace::trace_note!(
            "Производную функции по параметру или вектор таких производных называют gradient."
        );
        let rate_of_change: f64 = 2.0 * (parameter - 3.0);
        lesson_trace::trace_step!(rate_of_change);
        lesson_trace::trace_note!(
            "Порог 10⁻⁶ задаёт, насколько близко к нулю должен стать градиент перед остановкой."
        );
        if rate_of_change > -0.000001 && rate_of_change < 0.000001 {
            lesson_trace::trace_note!(
                "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
            );
            println!("сошлись за {epoch} шагов: параметр={parameter}");
            lesson_trace::trace_note!("Останавливаем цикл после достижения условия завершения.");
            break;
        }
        lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
        parameter -= rate * rate_of_change;
        lesson_trace::trace_step!(parameter);
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_squared_error_after_each_parameter_update();
}

// Строим график по результатам урока.
fn plot_squared_error_after_each_parameter_update() {
    lesson_trace::trace_note!("Наглядное представление величин из этого урока.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let convergence_points: Vec<(f64, f64)> = (0..=30)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = 3.0 * (1.0 - 0.6_f64.powi(plot_step_index));
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                plot_step_index as f64,
                (horizontal_value - 3.0) * (horizontal_value - 3.0),
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
        "Сходимость градиентного спуска",
        "шаг",
        "ошибка",
        &[lesson_visualization::Series {
            name: "η=0.2",

            points: &convergence_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
