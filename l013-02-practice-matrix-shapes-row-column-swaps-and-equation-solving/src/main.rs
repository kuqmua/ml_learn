// Урок 02.6. Практика: размеры матриц, перестановка строк в столбцы, умножение и решение уравнений.
// Связь с принятой терминологией: Форма матрицы, транспонирование, произведения и система уравнений.
// Зачем здесь эта тема: Формы, транспонирование и произведения должны работать вместе, иначе ошибка
//   размера проявится уже в системе.
// Почему код устроен так: Собираем малую матрицу и проверяем каждый переход по размерности и
//   значению.
// Представь: Прежде чем умножать матрицы, можно на бумаге определить форму результата и заметить
//   несовместимые размеры.
//
// Что повторяем вместе: формы матриц, транспонирование, матричное умножение, системы уравнений.
// Зачем это нужно: Матрицы описывают преобразования сразу нескольких признаков и служат основой линейных
//   слоёв моделей.
// Что показывает программа: Создаём матрицу 2×2 с известными элементами. Умножаем матрицу на вектор: каждая
//   координата ответа — сумма после попарного умножения элементов строки. Транспонируем матрицу, меняя строки и столбцы местами.
// Что проверить при изменении примера: Проверь единичную матрицу, несовместимые формы и несколько
//   результатов умножения, вычисленных вручную.
// Дополнительная практика: Сделай Matrix с проверкой размерностей, transpose, matmul и matvec без внешних
//   библиотек.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coords_then_add_results_as_unnormalized_alignment_where_pos_means_angle_below_90_degrees_neg_means_angle_above_90_degrees_and_0_means_perpendicular_or_zero_vec;

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
        Matrix::create_matrix_from_elements_listed_row_by_row(2, 2, vec![1., 2., 3., 4.]).unwrap();
    let input_vec: [f64; 2] = [1., 1.];
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
            multiply_matching_coords_then_add_results_as_unnormalized_alignment_where_pos_means_angle_below_90_degrees_neg_means_angle_above_90_degrees_and_0_means_perpendicular_or_zero_vec(
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
                multiply_matching_coords_then_add_results_as_unnormalized_alignment_where_pos_means_angle_below_90_degrees_neg_means_angle_above_90_degrees_and_0_means_perpendicular_or_zero_vec(
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

    plot_result_after_multiplying_transposed_and_original_matrices(result_matrix);

    fn plot_result_after_multiplying_transposed_and_original_matrices(result_matrix: Matrix) {
        lesson_visualization::heatmap(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Матрица AᵀA",
            &result_matrix
                .data
                .chunks(result_matrix.column_count)
                .map(|row| row.to_vec())
                .collect::<Vec<_>>(),
        )
        .expect("не удалось сохранить тепловую карту");
    }
}
