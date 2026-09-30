// Урок 22.2. Проверка размеров матриц до и после умножения.
// Связь с принятой терминологией: Проверка форм матриц до умножения и формы результата.
// Зачем здесь эта тема: Автодифференцирование матричных операций требует согласованных форм, иначе
//   ошибка распространится на градиенты.
// Почему код устроен так: Проверяем внутренние размеры до умножения и ожидаемую форму результата.
// Представь: Матрицу 2×3 можно умножить на 3×4 и получить 2×4, но с 2×4 внутренние размеры не
//   совпадут.
//
// Матрицы (a,b) и (c,d) можно умножить, только если b=c; ответ имеет форму (a,d).

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, left_shape, right_shape, expected) in [
        ("совместимые формы", (2, 3), (3, 4), Some((2, 4))),
        ("квадратные матрицы", (2, 2), (2, 2), Some((2, 2))),
        ("несовместимые формы", (2, 3), (2, 4), None),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(left_shape);
        lesson_trace::trace_step!(right_shape);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `result_shape`.");
        let result_shape: Option<(i32, i32)> = if left_shape.1 == right_shape.0 {
            lesson_trace::trace_note!("Возвращаем присутствующее значение.");
            Some((left_shape.0, right_shape.1))
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!("Отмечаем отсутствие подходящего значения.");
            None
        };
        lesson_trace::trace_step!(result_shape);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(result_shape, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {left_shape:?} × {right_shape:?} → {result_shape:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_row_and_column_counts_of_input_and_output_matrices();
}

// Строим график по результатам урока.
fn plot_row_and_column_counts_of_input_and_output_matrices() {
    lesson_trace::trace_note!("Сравнение величин из этого урока.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Размеры тензоров",
        "измерение",
        &[
            ("строки A", 2.0),
            ("столбцы A", 3.0),
            ("строки B", 3.0),
            ("столбцы B", 2.0),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
