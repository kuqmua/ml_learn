// Урок 02.4. Произведение двух матриц: сложение произведений соответствующих элементов строк и столбцов.
// Связь с принятой терминологией: Умножение двух матриц через скалярные произведения строк и столбцов.
// Зачем здесь эта тема: После произведения матрицы на вектор произведение матриц повторяет ту же
//   идею для каждого столбца второго множителя.
// Почему код устроен так: Для элемента результата берём строку слева и столбец справа; их длины
//   должны совпадать.
// Представь: Одно число в произведении матриц получается из строки первой и столбца второй матрицы.
//
// Каждая ячейка ответа получается попарным умножением строки и столбца со сложением.
// Единичная матрица не меняет значения; порядок множителей обычно влияет на ответ.

use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;

fn main() {
    let left: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let identity: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
    let right: [[f64; 2]; 2] = [[5.0, 6.0], [7.0, 8.0]];
    for (_description, first, second, expected) in [
        ("обычный порядок", left, right, [[19.0, 22.0], [43.0, 50.0]]),
        (
            "обратный порядок",
            right,
            left,
            [[23.0, 34.0], [31.0, 46.0]],
        ),
        ("единичная справа", left, identity, left),
        ("единичная слева", identity, left, left),
    ] {
        assert_eq!(
            first[0].len(),
            second.len(),
            "внутренние размеры матриц должны совпадать"
        );
        let mut result: [[f64; 2]; 2] = [[0.0; 2]; 2];
        for row in 0..first.len() {
            for column in 0..second[0].len() {
                let column_values: [f64; 2] = [second[0][column], second[1][column]];
                result[row][column] =
                    multiply_matching_coordinates_then_add_results(&first[row], &column_values)
                        .expect("внутренние размеры матриц совпадают");
            }
        }
        assert_eq!(result, expected);
    }
    let incompatible_left_shape: (i32, i32) = (2, 3);
    let incompatible_right_shape: (i32, i32) = (2, 2);
    if incompatible_left_shape.1 != incompatible_right_shape.0 {}

    plot_matrix_product_as_sums_of_matching_row_and_column_products();
}

// Строим график по результатам урока.
fn plot_matrix_product_as_sums_of_matching_row_and_column_products() {
    let _chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Произведение матриц A × B",
        &[vec![19.0, 22.0], vec![43.0, 50.0]],
    )
    .expect("не удалось сохранить тепловую карту");
}
