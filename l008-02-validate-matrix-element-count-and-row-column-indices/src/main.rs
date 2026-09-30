// Урок 02.1. Проверка числа элементов матрицы и границ строки и столбца.
// Зачем здесь эта тема: Матрица хранит значения по строкам и столбцам; неверная форма испортит все
//   последующие операции.
// Почему код устроен так: Сначала проверяем число элементов и индексы, затем допускаем чтение
//   конкретной ячейки.
// Представь: Таблица из двух строк по три числа имеет форму 2×3; строка с двумя числами нарушает
//   форму.
//
// Что изучаем: форма rows×columns требует ровно rows*columns элементов.
// Также индекс строки и столбца должен оставаться внутри этих границ.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `elements`.");
    let elements: [i32; 6] = [1, 2, 3, 4, 5, 6];
    lesson_trace::trace_step!(elements);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, rows, columns, row, column) in [
        ("допустимая ячейка", 2, 3, 1, 2),
        ("лишний столбец", 2, 4, 1, 2),
        ("строка вне матрицы", 2, 3, 2, 0),
        ("столбец вне матрицы", 2, 3, 1, 3),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(rows);
        lesson_trace::trace_step!(columns);
        lesson_trace::trace_step!(row);
        lesson_trace::trace_step!(column);
        lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if rows * columns != elements.len() {
            lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
            lesson_trace::trace_note!(
                "Передаём подпись или текстовое значение для следующего шага."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            println!(
                "{description}: форма {rows}×{columns} не подходит для {} элементов",
                elements.len()
            );
            lesson_trace::trace_note!("Переходим к следующему шагу цикла или завершаем его.");
            continue;
        }
        lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if row >= rows || column >= columns {
            lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
            println!("{description}: ячейка [{row}, {column}] вне формы {rows}×{columns}");
            lesson_trace::trace_note!("Переходим к следующему шагу цикла или завершаем его.");
            continue;
        }
        lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
        let value: i32 = elements[row * columns + column];
        lesson_trace::trace_step!(value);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: ячейка [{row}, {column}] = {value}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_matrix_with_two_rows_and_three_columns();
}

// Строим график по результатам урока.
fn plot_matrix_with_two_rows_and_three_columns() {
    lesson_trace::trace_note!("Значения ячеек видны по цвету и подписи.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок тепловой карты.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Форма 2 × 3",
        &[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    )
    .expect("не удалось сохранить тепловую карту");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
