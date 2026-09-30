// Урок 22.3. Смещение строк матрицы: прибавление одного вектора ко всем строкам.
// Связь с принятой терминологией: Добавление вектора смещений ко всем строкам матрицы.
// Зачем здесь эта тема: Bias добавляется к каждому примеру пакета, хотя хранится одним вектором.
// Почему код устроен так: Явно повторяем прибавление по строкам, чтобы увидеть правило
//   broadcasting.
// Представь: Если bias=[1, 2], он прибавляется к каждой строке пакета, а не только к первой.
//
// Что изучаем: Broadcasting.
// Зачем это нужно: Меньший массив повторяется вдоль совместимой оси; здесь один bias добавляется к каждой
// строке.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let matrix: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    let bias: [f64; 2] = [10.0, 20.0];
    let mut result: [[f64; 2]; 2] = matrix;
    for row in 0..2 {
        for column in 0..2 {
            result[row][column] += bias[column];
        }
    }

    plot_matrix_after_adding_same_bias_vector_to_each_row(result);
}

// Строим график по результатам урока.
fn plot_matrix_after_adding_same_bias_vector_to_each_row(result: [[f64; 2]; 2]) {
    let _chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Broadcasting: результат",
        &result.iter().map(|row| row.to_vec()).collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
