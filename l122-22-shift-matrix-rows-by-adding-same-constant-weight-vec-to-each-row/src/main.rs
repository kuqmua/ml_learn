// Урок 122. Прибавлять один вектор постоянных значений к каждой строке матрицы.
// Так одинаковые поправки применяются сразу ко всем примерам в группе.

fn main() {
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let mut matrix_after_adding_constant_weight: [[f64; 2]; 2] = matrix;
    let constant_input_weight: [f64; 2] = [10.0, 20.0];
    for row in 0..2 {
        for column in 0..2 {
            matrix_after_adding_constant_weight[row][column] += constant_input_weight[column];
        }
    }

    // Выполняем вычисления из примера.
    let _ = &matrix_after_adding_constant_weight;

    println!(
        "До={matrix:?}; общая прибавка={constant_input_weight:?}; после={matrix_after_adding_constant_weight:?}"
    );
    assert_eq!(
        matrix_after_adding_constant_weight,
        [[11.0, 22.0], [13.0, 24.0]]
    );
}

// Чему учит этот урок:
// Учимся прибавлять один вектор постоянных значений к каждой строке матрицы.
// Так одинаковые поправки применяются сразу ко всем примерам в группе.
