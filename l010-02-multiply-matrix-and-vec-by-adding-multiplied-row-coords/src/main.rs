// Урок 010. Для каждой строки таблицы получаем одно число.
// Умножаем первое число строки на первое входное число, второе — на второе и так далее.
// Затем складываем результаты. Для строки [1, 2] и входа [5, 6]: 1×5 + 2×6 = 17.
// Повторяем для всех строк и получаем список ответов.
// В строке и во входном списке должно быть одинаковое количество чисел.

use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

fn main() {
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    for (_description, vec, expected) in [
        ("обычный вектор", [5.0, 6.0], [17.0, 39.0]),
        ("нулевой вектор", [0.0, 0.0], [0.0, 0.0]),
    ] {
        let mut matrix_vec_output: [f64; 2] = [0.0; 2];
        for row in 0..matrix.len() {
            matrix_vec_output[row] =
                multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&matrix[row], &vec)
                    .expect("число столбцов матрицы должно совпадать с числом координат вектора");
        }
        assert_eq!(matrix_vec_output, expected);
    }
    let too_short: [f64; 1] = [5.0];
    let _: &str = multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&matrix[0], &too_short)
        .expect_err("разные длины нужно отклонить");

    plot_matrix_coefficients_used_in_weighted_row_sums(matrix);
}

// Строим график по результатам урока.
fn plot_matrix_coefficients_used_in_weighted_row_sums(matrix: [[f64; 2]; 2]) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Коэффициенты матрицы",
        &matrix.iter().map(|row| row.to_vec()).collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
