// Урок 122. Прибавлять один вектор постоянных значений по второму измерению для каждого первого индекса.
// Так одинаковые поправки применяются сразу ко всем примерам в группе.

fn main() {
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let mut matrix_after_adding_constant_weight: [[f64; 2]; 2] = matrix;
    let constant_input_weight: [f64; 2] = [10.0, 20.0];
    for first_dimension_index in 0..2 {
        for second_dimension_index in 0..2 {
            matrix_after_adding_constant_weight[first_dimension_index][second_dimension_index] +=
                constant_input_weight[second_dimension_index];
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
// Учимся прибавлять один вектор постоянных значений по второму измерению для каждого первого индекса.
// Так одинаковые поправки применяются сразу ко всем примерам в группе.
