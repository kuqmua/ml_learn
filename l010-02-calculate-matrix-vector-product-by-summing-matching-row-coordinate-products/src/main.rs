// Урок 02.3. Произведение матрицы и вектора: сложение произведений координат каждой строки и вектора.
// Связь с принятой терминологией: Умножение матрицы на вектор через скалярные произведения строк.
// Зачем здесь эта тема: Матрица превращает входной вектор в выходной; каждая строка задаёт одно
//   скалярное произведение.
// Почему код устроен так: Сначала сверяем длину строки с длиной вектора, затем считаем каждую
//   координату результата отдельно.
// Представь: Для строки [1, 2] и входа [5, 6] один выход равен 1·5+2·6=17.
//
// Для каждой строки умножаем её значения на координаты вектора и складываем.
// Нулевой вектор даёт нулевой ответ; число столбцов должно совпадать с длиной вектора.

use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;

use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Задаём учебные значения для `matrix`.");
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    trace_step!(matrix);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, vector, expected) in [
        ("обычный вектор", [5.0, 6.0], [17.0, 39.0]),
        ("нулевой вектор", [0.0, 0.0], [0.0, 0.0]),
    ] {
        trace_step!(description);
        trace_step!(vector);
        trace_step!(expected);
        trace_note!("Задаём учебные значения для `result`.");
        let mut result: [f64; 2] = [0.0; 2];
        trace_step!(result);
        trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for row in 0..matrix.len() {
            trace_step!(row);
            trace_note!("Урок 01.1 теперь работает и для каждой строки матрицы.");
            trace_note!("Используем результат, ожидая успешного выполнения шага.");
            result[row] = calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                &matrix[row],
                &vector,
            )
            .expect("число столбцов совпадает с длиной вектора");
            trace_step!(result);
        }
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(result, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {vector:?} → {result:?}");
    }
    trace_note!("Задаём учебные значения для `too_short`.");
    let too_short: [f64; 1] = [5.0];
    trace_step!(too_short);
    trace_note!("Сохраняем результат этого шага в `error`.");
    trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    let error: &str = calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
        &matrix[0], &too_short,
    )
    .expect_err("разные длины нужно отклонить");
    trace_step!(error);
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("разная длина строки и вектора: {error}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_matrix_coefficients_used_in_weighted_row_sums(matrix);
}

// Строим график по результатам урока.
fn plot_matrix_coefficients_used_in_weighted_row_sums(matrix: [[f64; 2]; 2]) {
    trace_note!("Значения ячеек видны по цвету и подписи.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок тепловой карты.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Коэффициенты матрицы",
        &matrix.iter().map(|row| row.to_vec()).collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
