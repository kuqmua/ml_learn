// Урок 010. Для каждого индекса первого измерения получаем одно число.
// Умножаем значения вложенного массива на соответствующие значения входного вектора.
// Затем складываем результаты. Для массива [1, 2] и входа [5, 6]: 1×5 + 2×6 = 17.
// Повторяем для всех индексов первого измерения и получаем список ответов.
// Размер второго измерения должен совпадать с длиной входного вектора.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    for (_description, vec, expected) in [
        ("обычный вектор", [5.0, 6.0], [17.0, 39.0]),
        ("нулевой вектор", [0.0, 0.0], [0.0, 0.0]),
    ] {
        let mut matrix_vec_output: [f64; 2] = [0.0; 2];
        for first_dimension_index in 0..matrix.len() {
            matrix_vec_output[first_dimension_index] = multiply_matching_coords_then_add_results(
                &matrix[first_dimension_index],
                &vec,
            )
            .expect("размер второго измерения матрицы должен совпадать с числом координат вектора");
        }
        assert_eq!(matrix_vec_output, expected);
    }
    let too_short: [f64; 1] = [5.0];
    let _: &str = multiply_matching_coords_then_add_results(&matrix[0], &too_short)
        .expect_err("разные длины нужно отклонить");

    // Выполняем вычисления из примера.
    let _ = matrix;
}

// Чему учит этот урок:
// Учимся получать одно число для каждого индекса первого измерения матрицы и общего входного вектора.
// Такая сумма с весами позволяет из одного набора входов вычислить несколько ответов; размеры
// должны совпадать.
