// Урок 23.5. Защита от слишком больших обновлений: ограничение скоростей изменения ошибки заданным интервалом.
// Связь с принятой терминологией: Ограничение координат градиента симметричным интервалом.
// Зачем здесь эта тема: Редкий очень большой градиент способен испортить устойчивость шага.
// Почему код устроен так: Ограничиваем координаты заданным интервалом и сравниваем обновление до и
//   после.
// Представь: Если градиент равен 100, а предел 5, в обновление пойдёт 5.
//
// Значения внутри интервала [-limit, limit] не меняются. Выходящие за границу
// заменяются ближайшей границей с сохранением знака.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Сохраняем результат этого шага в `limit`.");
    let limit: f64 = 1.0;
    lesson_trace::trace_step!(limit);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(limit > 0.0);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, rate_of_change, expected) in [
        ("слишком большой положительный", 12.0, 1.0),
        ("положительный внутри интервала", 0.5, 0.5),
        ("нулевой", 0.0, 0.0),
        ("отрицательный внутри интервала", -0.5, -0.5),
        ("слишком большой отрицательный", -12.0, -1.0),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(rate_of_change);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `clipped`.");
        let clipped: f64 = if rate_of_change > limit {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            limit
        } else if rate_of_change < -limit {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            -limit
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            rate_of_change
        };
        lesson_trace::trace_step!(clipped);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(clipped, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {rate_of_change} → {clipped}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_rate_of_change_clamped_to_symmetric_interval();
}

// Строим график по результатам урока.
fn plot_rate_of_change_clamped_to_symmetric_interval() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Ограничение величины заданным порогом называют clipping.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let bounded_value_points: Vec<(f64, f64)> = (-30..=30)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `rate_of_change_value`.");
            let rate_of_change_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (rate_of_change_value, rate_of_change_value.clamp(-1.0, 1.0))
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
        "Ограничение градиента",
        "градиент до",
        "градиент после",
        &[lesson_visualization::Series {
            name: "порог ±1",

            points: &bounded_value_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
