// Урок 02.3. Умножение матрицы на вектор: умножение соответствующих координат строки и вектора с последующим сложением.
// Связь с принятой терминологией: Умножение матрицы на вектор через скалярные произведения строк.
// Зачем здесь эта тема: Матрица превращает входной вектор в выходной; каждая строка задаёт одно
//   скалярное произведение.
// Почему код устроен так: Сначала сверяем длину строки с длиной вектора, затем считаем каждую
//   координату результата отдельно.
// Представь: Для строки [1, 2] и входа [5, 6] один выход равен 1·5+2·6=17.
//
// Для каждой строки умножаем её значения на координаты вектора и складываем.
// Нулевой вектор даёт нулевой ответ; число столбцов должно совпадать с длиной вектора.

use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;

fn main() {
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    for (_description, vector, expected) in [
        ("обычный вектор", [5.0, 6.0], [17.0, 39.0]),
        ("нулевой вектор", [0.0, 0.0], [0.0, 0.0]),
    ] {
        let mut matrix_vector_output: [f64; 2] = [0.0; 2];
        for row in 0..matrix.len() {
            matrix_vector_output[row] =
                multiply_matching_coordinates_then_add_results(&matrix[row], &vector)
                    .expect("число столбцов совпадает с длиной вектора");
        }
        assert_eq!(matrix_vector_output, expected);
    }
    let too_short: [f64; 1] = [5.0];
    let _error: &str = multiply_matching_coordinates_then_add_results(&matrix[0], &too_short)
        .expect_err("разные длины нужно отклонить");

    plot_matrix_coefficients_used_in_weighted_row_sums(matrix);
}

// Строим график по результатам урока.
fn plot_matrix_coefficients_used_in_weighted_row_sums(matrix: [[f64; 2]; 2]) {
    let _chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Коэффициенты матрицы",
        &matrix.iter().map(|row| row.to_vec()).collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
