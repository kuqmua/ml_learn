// Урок 02.2. Транспонирование матрицы: перестановка строк в столбцы.
// Зачем здесь эта тема: Транспонирование меняет роль строк и столбцов; это подготовка к матричному
//   произведению.
// Почему код устроен так: На малой матрице явно переставляем индексы, чтобы проверить новую форму и
//   адрес каждого элемента.
// Представь: Число в строке 1 и столбце 2 после поворота таблицы окажется в строке 2 и столбце 1.
//
// Что изучаем: Транспонирование матрицы.
// Зачем это нужно: Строки исходной матрицы становятся столбцами результата. Значение не меняется, меняются
// только его координаты.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `matrix` для следующего шага примера.");
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    lesson_trace::trace_step!(matrix);
    lesson_trace::trace_note!("Создаём набор значений `transposed` для следующего шага примера.");
    let mut transposed: [[i32; 2]; 3] = [[0; 2]; 3];
    lesson_trace::trace_step!(transposed);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for row in 0..2 {
        lesson_trace::trace_step!(row);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for column in 0..3 {
            lesson_trace::trace_step!(column);
            lesson_trace::trace_note!("Элемент [row, column] переносим в [column, row].");
            transposed[column][row] = matrix[row][column];
            lesson_trace::trace_step!(transposed);
        }
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("исходная: {matrix:?}; транспонированная: {transposed:?}");

    lesson_trace::trace_note!(
        "Повторное транспонирование возвращает каждое число на исходное место."
    );
    let mut restored: [[i32; 3]; 2] = [[0; 3]; 2];
    lesson_trace::trace_step!(restored);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for row in 0..transposed.len() {
        lesson_trace::trace_step!(row);
        lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for column in 0..transposed[row].len() {
            lesson_trace::trace_step!(column);
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            restored[column][row] = transposed[row][column];
            lesson_trace::trace_step!(restored);
        }
    }
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert_eq!(restored, matrix);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("после второго транспонирования: {restored:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_matrix_after_turning_rows_into_columns(transposed);
}

// Строим график по результатам урока.
fn plot_matrix_after_turning_rows_into_columns(transposed: [[i32; 2]; 3]) {
    lesson_trace::trace_note!("Значения ячеек видны по цвету и подписи.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок тепловой карты.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Транспонированная матрица",
        &transposed
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&element_value| element_value as f64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
