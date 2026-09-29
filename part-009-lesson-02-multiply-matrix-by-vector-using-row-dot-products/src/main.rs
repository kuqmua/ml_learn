// Урок 02.3. Умножение матрицы на вектор через скалярные произведения строк.
// Почему этот урок сейчас: Матрица превращает входной вектор в выходной; каждая строка задаёт одно скалярное произведение.
// Почему пример устроен так: Сначала сверяем длину строки с длиной вектора, затем считаем каждую координату результата отдельно.
//
// Для каждой строки умножаем её значения на координаты вектора и складываем.
// Нулевой вектор даёт нулевой ответ; число столбцов должно совпадать с длиной вектора.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `matrix`.
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    lesson_trace::trace_step!(matrix);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, vector, expected) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("обычный вектор", [5.0, 6.0], [17.0, 39.0]),
        // Добавляем пару значений для сравнения или построения графика.
        ("нулевой вектор", [0.0, 0.0], [0.0, 0.0]),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(vector);
        lesson_trace::trace_step!(expected);
        // Задаём учебные значения для `result`.
        let mut result: [f64; 2] = [0.0; 2];
        lesson_trace::trace_step!(result);
        // Повторяем расчёт для каждого элемента последовательности.
        for row in 0..matrix.len() {
            lesson_trace::trace_step!(row);
            // Урок 01.1 теперь работает и для каждой строки матрицы.
            result[row] = part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::multiply_matching_coordinates_of_two_vectors_then_add(&matrix[row], &vector)
                // Используем результат, ожидая успешного выполнения шага.
                .expect("число столбцов совпадает с длиной вектора");
            lesson_trace::trace_step!(result);
        }
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(result, expected);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: {vector:?} → {result:?}");
    }
    // Задаём учебные значения для `too_short`.
    let too_short: [f64; 1] = [5.0];
    lesson_trace::trace_step!(too_short);
    // Сохраняем результат этого шага в `error`.
    let error: &str =
        part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::multiply_matching_coordinates_of_two_vectors_then_add(
            &matrix[0], &too_short,
        )
        // Настраиваем или преобразуем результат предыдущего шага.
        .expect_err("разные длины нужно отклонить");
    lesson_trace::trace_step!(error);
    // Печатаем рассчитанные значения для проверки примера.
    println!("разная длина строки и вектора: {error}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_multiply_matrix_by_vector_using_row_dot_products(matrix);
}

// Строим график по результатам урока.
fn visualize_multiply_matrix_by_vector_using_row_dot_products(matrix: [[f64; 2]; 2]) {
    // Значения ячеек видны по цвету и подписи.
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок тепловой карты.
        "Коэффициенты матрицы",
        // Используем подготовленное значение в следующем шаге примера.
        &matrix.iter().map(|row| row.to_vec()).collect::<Vec<_>>(),
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить тепловую карту");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
