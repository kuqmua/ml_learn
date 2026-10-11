// Урок 013. Хранить матрицу в структуре: размеры измерений отдельно, элементы последовательно в Vec.
// Проверяем размеры, извлекаем вложенные массивы, умножаем матрицу на вектор и переставляем измерения.
// Соединяем операции, вычисляя произведение транспонированной матрицы на исходную.
// Решаем систему Ax=b для этой матрицы и проверяем ответ обратным умножением.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    #[derive(Debug, PartialEq)]
    struct Matrix {
        first_dimension_length: usize,

        second_dimension_length: usize,

        data: Vec<f64>,
    }

    impl Matrix {
        /// Создаём матрицу из плоского массива: сначала перебирается второе измерение, затем первое.
        fn create_matrix_from_elements_listed_row_by_row(
            first_dimension_length: usize,

            second_dimension_length: usize,

            data: Vec<f64>,
        ) -> Result<Self, &'static str> {
            if first_dimension_length * second_dimension_length != data.len() {
                return Err(
                    "число элементов должно равняться произведению размеров двух измерений",
                );
            }
            Ok(Self {
                first_dimension_length,
                second_dimension_length,
                data,
            })
        }

        fn value_at_row_and_column(
            &self,
            first_dimension_index: usize,
            second_dimension_index: usize,
        ) -> f64 {
            self.data[first_dimension_index * self.second_dimension_length + second_dimension_index]
        }
    }

    let left_matrix: Matrix =
        Matrix::create_matrix_from_elements_listed_row_by_row(2, 2, vec![1.0, 2.0, 3.0, 4.0])
            .unwrap();
    let input_vec: [f64; 2] = [1.0, 1.0];
    assert_eq!(
        left_matrix.second_dimension_length,
        input_vec.len(),
        "несовместимые формы"
    );
    let mut output_vec: Vec<f64> = Vec::with_capacity(left_matrix.first_dimension_length);
    for first_dimension_index in 0..left_matrix.first_dimension_length {
        let row_start: usize = first_dimension_index * left_matrix.second_dimension_length;
        let row_end: usize = row_start + left_matrix.second_dimension_length;

        output_vec.push(
            multiply_matching_coords_then_add_results(
                &left_matrix.data[row_start..row_end],
                &input_vec,
            )
            .expect("размер второго измерения должен совпадать с числом координат вектора"),
        );
    }
    let mut transposed_elements: Vec<f64> = Vec::with_capacity(left_matrix.data.len());
    for second_dimension_index in 0..left_matrix.second_dimension_length {
        for first_dimension_index in 0..left_matrix.first_dimension_length {
            transposed_elements.push(
                left_matrix.value_at_row_and_column(first_dimension_index, second_dimension_index),
            );
        }
    }
    let transposed_matrix: Matrix = Matrix {
        first_dimension_length: left_matrix.second_dimension_length,

        second_dimension_length: left_matrix.first_dimension_length,

        data: transposed_elements,
    };
    assert_eq!(
        transposed_matrix.second_dimension_length, left_matrix.first_dimension_length,
        "несовместимые формы"
    );
    let mut result_elements: Vec<f64> = Vec::with_capacity(
        transposed_matrix.first_dimension_length * left_matrix.second_dimension_length,
    );
    for first_dimension_index in 0..transposed_matrix.first_dimension_length {
        for second_dimension_index in 0..left_matrix.second_dimension_length {
            let row_start: usize =
                first_dimension_index * transposed_matrix.second_dimension_length;
            let row_end: usize = row_start + transposed_matrix.second_dimension_length;

            result_elements.push(
                multiply_matching_coords_then_add_results(
                    &transposed_matrix.data[row_start..row_end],
                    &(0..left_matrix.first_dimension_length)
                        .map(|shared_index| {
                            left_matrix
                                .value_at_row_and_column(shared_index, second_dimension_index)
                        })
                        .collect::<Vec<_>>(),
                )
                .expect("внутренние размеры матриц должны совпадать"),
            );
        }
    }
    let result_matrix: Matrix = Matrix::create_matrix_from_elements_listed_row_by_row(
        transposed_matrix.first_dimension_length,
        left_matrix.second_dimension_length,
        result_elements,
    )
    .unwrap();

    assert_eq!(output_vec, vec![3.0, 7.0]);
    assert_eq!(transposed_matrix.data, vec![1.0, 3.0, 2.0, 4.0]);
    // Значения с фиксированным вторым индексом сравниваем с собой или с другим таким набором.
    // В результате A^T*A для каждой пары вторых индексов суммируем произведения по первому измерению.
    println!(
        "Исходная матрица: {:?}; умножение на [1,1]: {output_vec:?}",
        left_matrix.data
    );
    println!("Транспонированная: {:?}", transposed_matrix.data);

    assert_eq!(result_matrix.data, vec![10.0, 14.0, 14.0, 20.0]);
    println!(
        "Суммы произведений по первому измерению, A^T*A: {:?}",
        result_matrix.data
    );
    // Обратная задача: известны результаты [5, 11], найдём вход [x, y].
    // Строки матрицы задают x + 2*y = 5 и 3*x + 4*y = 11.
    let right_hand_side = [5.0, 11.0];
    let row1_coef1 = left_matrix.value_at_row_and_column(0, 0);
    let row1_coef2 = left_matrix.value_at_row_and_column(0, 1);
    let row2_coef1 = left_matrix.value_at_row_and_column(1, 0);
    let row2_coef2 = left_matrix.value_at_row_and_column(1, 1);
    // Как в уроке 012: умножаем коэффициенты уравнений и вычитаем, чтобы исключить y.
    let x_coef = row1_coef1 * row2_coef2 - row2_coef1 * row1_coef2;
    let right_after_eliminating_y =
        right_hand_side[0] * row2_coef2 - right_hand_side[1] * row1_coef2;
    assert_ne!(x_coef, 0.0, "этот пример требует единственного решения");
    let x = right_after_eliminating_y / x_coef;
    let y = (right_hand_side[0] - row1_coef1 * x) / row1_coef2;
    let solution = [x, y];
    assert_eq!(solution, [1.0, 2.0]);
    // Проверяем оба уравнения, а не только то, из которого получили y.
    for first_dimension_index in 0..left_matrix.first_dimension_length {
        let start = first_dimension_index * left_matrix.second_dimension_length;
        let restored = multiply_matching_coords_then_add_results(
            &left_matrix.data[start..start + left_matrix.second_dimension_length],
            &solution,
        )
        .unwrap();
        assert_eq!(restored, right_hand_side[first_dimension_index]);
        println!("Строка {first_dimension_index}: найденный вход {solution:?} даёт {restored}");
    }
    assert!(Matrix::create_matrix_from_elements_listed_row_by_row(2, 3, vec![1.0; 4]).is_err());
}

// Чему учит этот урок:
// Учимся хранить матрицу в структуре: размеры измерений отдельно, элементы последовательно в Vec.
// Проверяем размеры, извлекаем вложенные массивы, умножаем матрицу на вектор и переставляем измерения.
// Соединяем операции, вычисляя произведение транспонированной матрицы на исходную.
// Решаем систему Ax=b для этой матрицы и проверяем ответ обратным умножением.
