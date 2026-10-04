// Урок 013. Хранить матрицу в структуре: размеры отдельно, числа строка за строкой в Vec.
// Проверяем размеры, извлекаем строки, умножаем матрицу на вектор и переставляем строки в столбцы.
// Соединяем операции, вычисляя произведение транспонированной матрицы на исходную.
// Решаем систему Ax=b для этой матрицы и проверяем ответ обратным умножением.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    #[derive(Debug, PartialEq)]
    struct Matrix {
        rows: usize,

        column_count: usize,

        data: Vec<f64>,
    }

    impl Matrix {
        /// Создаём матрицу из элементов, перечисленных строка за строкой (row-major order).
        fn create_matrix_from_elements_listed_row_by_row(
            rows: usize,

            column_count: usize,

            data: Vec<f64>,
        ) -> Result<Self, &'static str> {
            if rows * column_count != data.len() {
                return Err(
                    "число элементов должно равняться произведению числа строк на число столбцов",
                );
            }
            Ok(Self {
                rows,
                column_count,
                data,
            })
        }

        fn value_at_row_and_column(&self, row_index: usize, column_index: usize) -> f64 {
            self.data[row_index * self.column_count + column_index]
        }
    }

    let left_matrix: Matrix =
        Matrix::create_matrix_from_elements_listed_row_by_row(2, 2, vec![1.0, 2.0, 3.0, 4.0])
            .unwrap();
    let input_vec: [f64; 2] = [1.0, 1.0];
    assert_eq!(
        left_matrix.column_count,
        input_vec.len(),
        "несовместимые формы"
    );
    let mut output_vec: Vec<f64> = Vec::with_capacity(left_matrix.rows);
    for row_index in 0..left_matrix.rows {
        let row_start: usize = row_index * left_matrix.column_count;
        let row_end: usize = row_start + left_matrix.column_count;

        output_vec.push(
            multiply_matching_coords_then_add_results(
                &left_matrix.data[row_start..row_end],
                &input_vec,
            )
            .expect("число элементов строки должно совпадать с числом координат вектора"),
        );
    }
    let mut transposed_elements: Vec<f64> = Vec::with_capacity(left_matrix.data.len());
    for column_index in 0..left_matrix.column_count {
        for row_index in 0..left_matrix.rows {
            transposed_elements.push(left_matrix.value_at_row_and_column(row_index, column_index));
        }
    }
    let transposed_matrix: Matrix = Matrix {
        rows: left_matrix.column_count,

        column_count: left_matrix.rows,

        data: transposed_elements,
    };
    assert_eq!(
        transposed_matrix.column_count, left_matrix.rows,
        "несовместимые формы"
    );
    let mut result_elements: Vec<f64> =
        Vec::with_capacity(transposed_matrix.rows * left_matrix.column_count);
    for row_index in 0..transposed_matrix.rows {
        for column_index in 0..left_matrix.column_count {
            let row_start: usize = row_index * transposed_matrix.column_count;
            let row_end: usize = row_start + transposed_matrix.column_count;

            result_elements.push(
                multiply_matching_coords_then_add_results(
                    &transposed_matrix.data[row_start..row_end],
                    &(0..left_matrix.rows)
                        .map(|shared_index| {
                            left_matrix.value_at_row_and_column(shared_index, column_index)
                        })
                        .collect::<Vec<_>>(),
                )
                .expect("внутренние размеры матриц должны совпадать"),
            );
        }
    }
    let result_matrix: Matrix = Matrix::create_matrix_from_elements_listed_row_by_row(
        transposed_matrix.rows,
        left_matrix.column_count,
        result_elements,
    )
    .unwrap();

    assert_eq!(output_vec, vec![3.0, 7.0]);
    assert_eq!(transposed_matrix.data, vec![1.0, 3.0, 2.0, 4.0]);
    // Один и тот же столбец сравниваем с собой или с другим столбцом.
    // В результате A^T*A получаются суммы произведений каждой пары столбцов.
    println!(
        "Исходная матрица: {:?}; умножение на [1,1]: {output_vec:?}",
        left_matrix.data
    );
    println!("Транспонированная: {:?}", transposed_matrix.data);

    assert_eq!(result_matrix.data, vec![10.0, 14.0, 14.0, 20.0]);
    println!(
        "Суммы произведений пар столбцов, A^T*A: {:?}",
        result_matrix.data
    );
    // Обратная задача: известны результаты [5, 11], найдём вход [x, y].
    // Строки матрицы задают x + 2*y = 5 и 3*x + 4*y = 11.
    let right_hand_side = [5.0, 11.0];
    let row1_coef1 = left_matrix.value_at_row_and_column(0, 0);
    let row1_coef2 = left_matrix.value_at_row_and_column(0, 1);
    let row2_coef1 = left_matrix.value_at_row_and_column(1, 0);
    let row2_coef2 = left_matrix.value_at_row_and_column(1, 1);
    // Как в уроке 012: умножаем строки и вычитаем, чтобы исключить y.
    let x_coef = row1_coef1 * row2_coef2 - row2_coef1 * row1_coef2;
    let right_after_eliminating_y =
        right_hand_side[0] * row2_coef2 - right_hand_side[1] * row1_coef2;
    assert_ne!(x_coef, 0.0, "этот пример требует единственного решения");
    let x = right_after_eliminating_y / x_coef;
    let y = (right_hand_side[0] - row1_coef1 * x) / row1_coef2;
    let solution = [x, y];
    assert_eq!(solution, [1.0, 2.0]);
    // Проверяем обе строки, а не только уравнение, из которого получили y.
    for row in 0..left_matrix.rows {
        let start = row * left_matrix.column_count;
        let restored = multiply_matching_coords_then_add_results(
            &left_matrix.data[start..start + left_matrix.column_count],
            &solution,
        )
        .unwrap();
        assert_eq!(restored, right_hand_side[row]);
        println!("Строка {row}: найденный вход {solution:?} даёт {restored}");
    }
    assert!(Matrix::create_matrix_from_elements_listed_row_by_row(2, 3, vec![1.0; 4]).is_err());
}

// Чему учит этот урок:
// Учимся хранить матрицу в структуре: размеры отдельно, числа строка за строкой в Vec.
// Проверяем размеры, извлекаем строки, умножаем матрицу на вектор и переставляем строки в столбцы.
// Соединяем операции, вычисляя произведение транспонированной матрицы на исходную.
// Решаем систему Ax=b для этой матрицы и проверяем ответ обратным умножением.
