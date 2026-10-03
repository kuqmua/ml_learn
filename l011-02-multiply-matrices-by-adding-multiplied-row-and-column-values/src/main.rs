// Урок 011. Строим новую таблицу из двух исходных таблиц чисел.
// Для одной ячейки берём строку первой таблицы и столбец второй.
// Умножаем числа на одинаковых местах и складываем результаты.
// Поэтому число столбцов первой таблицы должно совпадать с числом строк второй.
// Например, таблицы 2×3 и 3×4 дают результат 2×4.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    let first_matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let identity: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
    let second_matrix: [[f64; 2]; 2] = [[5.0, 6.0], [7.0, 8.0]];
    for (_description, matrix_on_left, matrix_on_right, expected) in [
        (
            "обычный порядок",
            first_matrix,
            second_matrix,
            [[19.0, 22.0], [43.0, 50.0]],
        ),
        (
            "обратный порядок",
            second_matrix,
            first_matrix,
            [[23.0, 34.0], [31.0, 46.0]],
        ),
        ("единичная справа", first_matrix, identity, first_matrix),
        ("единичная слева", identity, first_matrix, first_matrix),
    ] {
        assert_eq!(
            matrix_on_left[0].len(),
            matrix_on_right.len(),
            "внутренние размеры матриц должны совпадать"
        );
        let mut multiplied_matrix: [[f64; 2]; 2] = [[0.0; 2]; 2];
        for row in 0..matrix_on_left.len() {
            for column in 0..matrix_on_right[0].len() {
                let column_values: [f64; 2] =
                    [matrix_on_right[0][column], matrix_on_right[1][column]];
                multiplied_matrix[row][column] =
                    multiply_matching_coords_then_add_results(&matrix_on_left[row], &column_values)
                        .expect("внутренние размеры матриц должны совпадать");
            }
        }
        assert_eq!(multiplied_matrix, expected);
    }
    let incompatible_left_shape: (i32, i32) = (2, 3);
    let incompatible_right_shape: (i32, i32) = (2, 2);
    if incompatible_left_shape.1 != incompatible_right_shape.0 {}

    plot_matrix_multiplication_as_sums_after_multiplying_matching_row_and_column_values();
}

// Строим график по результатам урока.
fn plot_matrix_multiplication_as_sums_after_multiplying_matching_row_and_column_values() {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Произведение матриц A × B",
        &[vec![19.0, 22.0], vec![43.0, 50.0]],
    )
    .expect("не удалось сохранить тепловую карту");
}
