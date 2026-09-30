// Урок 41.2. Перемещение агента влево или вправо в границах среды.
// Связь с принятой терминологией: Перемещение агента влево или вправо в ограниченной среде.
// Зачем здесь эта тема: Действие имеет смысл только вместе с правилом перехода между состояниями.
// Почему код устроен так: Ограничиваем движение границами среды и показываем результат каждого
//   шага.
// Представь: Из крайней левой клетки действие «влево» не должно выводить агента за границу мира.
//
// В линейном мире агент может шагнуть вправо или влево. На левой границе
// движение влево оставляет его в нулевой клетке.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Сохраняем результат этого шага в `last_state`.");
    let last_state: i32 = 4;
    lesson_trace::trace_step!(last_state);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, state, action, expected) in [
        ("шаг вправо", 2, 1, 3),
        ("шаг влево", 2, -1, 1),
        ("левая граница", 0, -1, 0),
        ("правая граница", 4, 1, 4),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(state);
        lesson_trace::trace_step!(action);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((0..=last_state).contains(&state));
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(action == -1 || action == 1);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `next_state`.");
        let next_state: i32 = (state + action).clamp(0, last_state);
        lesson_trace::trace_step!(next_state);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(next_state, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {state} + {action} → {next_state}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_next_position_after_moving_right();
}

// Строим график по результатам урока.
fn plot_next_position_after_moving_right() {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    let action_points: Vec<(f64, f64)> = (0..=5)
        .map(|plot_step_index| (plot_step_index as f64, (plot_step_index + 1) as f64))
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
        "Переход состояния",
        "текущее состояние",
        "следующее состояние",
        &[lesson_visualization::Series {
            name: "действие +1",

            points: &action_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
