// Урок 013. Соединяем работу с таблицами: проверку размеров, перестановку строк в столбцы
// и получение новых чисел умножением соответствующих элементов и сложением.
// Перед каждым действием проверяем, подходят ли размеры входных таблиц.
// Так ошибка в размере обнаружится до попытки прочитать несуществующую ячейку.

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
    let _ = (&(output_vec), &(result_matrix));

    // Выполняем вычисления из примера.
    let _ = result_matrix;
}

// Чему учит этот урок:
// Учимся хранить матрицу в структуре: размеры отдельно, числа строка за строкой в Vec.
// Проверяем размеры, извлекаем строки, умножаем матрицу на вектор и переставляем строки в столбцы.
// Соединяем операции, вычисляя произведение транспонированной матрицы на исходную.
// Это практика работы с матрицами; прикладной задачи и решения уравнений в текущем примере нет.
