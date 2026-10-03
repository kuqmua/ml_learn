// Урок 22.3. Смещение строк матрицы: прибавление одного вектора ко всем строкам.
// Связь с принятой терминологией: Добавление вектора смещений ко всем строкам матрицы.
// Зачем здесь эта тема: Вектор постоянных весов добавляется к каждому примеру пакета, хотя хранится одним вектором.
// Почему код устроен так: Явно повторяем прибавление по строкам, чтобы увидеть правило
//   broadcasting.
// Представь: Если constant_input_weight=[1, 2], он прибавляется к каждой строке пакета, а не только к первой.
//
// Что изучаем: Broadcasting.
// Зачем это нужно: Меньший массив повторяется вдоль совместимой оси; здесь один constant_input_weight добавляется к каждой
// строке.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let mut matrix_after_adding_constant_weight: [[f64; 2]; 2] = matrix;
    let constant_input_weight: [f64; 2] = [10.0, 20.0];
    for row in 0..2 {
        for column in 0..2 {
            matrix_after_adding_constant_weight[row][column] += constant_input_weight[column];
        }
    }

    plot_matrix_after_adding_same_constant_input_weight_vec_to_each_row(
        matrix_after_adding_constant_weight,
    );
}

// Строим график по результатам урока.
fn plot_matrix_after_adding_same_constant_input_weight_vec_to_each_row(
    matrix_after_adding_constant_weight: [[f64; 2]; 2],
) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Broadcasting: результат",
        &matrix_after_adding_constant_weight
            .iter()
            .map(|row| row.to_vec())
            .collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
