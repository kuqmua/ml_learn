// Урок 41.4. Выбор действия с наибольшей оценкой ожидаемой награды.
// Связь с принятой терминологией: Выбор действия агента по оценкам политики.
// Зачем здесь эта тема: После появления наград политика должна выбирать действие по оценкам
//   состояния.
// Почему код устроен так: Сопоставляем доступные действия с оценками и выбираем предсказанный
//   лучший вариант.
// Представь: В клетке агент сравнивает оценки «влево» и «вправо» и выбирает действие.
//
// Что изучаем: Политика агента.
// Зачем это нужно: Политика выбирает действие по оценкам доступных вариантов в текущем состоянии.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Инициализируем значение `left_action_value` начальным состоянием.");
    let left_action_value: f64 = 0.2;
    lesson_trace::trace_step!(left_action_value);
    lesson_trace::trace_note!("Инициализируем значение `right_action_value` начальным состоянием.");
    let right_action_value: f64 = 0.8;
    lesson_trace::trace_step!(right_action_value);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `action` для следующих операций.");
    let action: &str = if right_action_value > left_action_value {
        lesson_trace::trace_note!(
            "Подставляем результаты в этот шаблон вывода или текстового значения."
        );
        "вправо"
    } else {
        lesson_trace::trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
        lesson_trace::trace_note!(
            "Подставляем результаты в этот шаблон вывода или текстового значения."
        );
        "влево"
    };
    lesson_trace::trace_step!(action);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("политика выбирает: {action}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_estimated_rewards_for_available_actions(left_action_value, right_action_value);
}

// Строим график по результатам урока.
fn plot_estimated_rewards_for_available_actions(left_action_value: f64, right_action_value: f64) {
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Оценки действий политики",
        "Q-значение",
        &[("влево", left_action_value), ("вправо", right_action_value)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
