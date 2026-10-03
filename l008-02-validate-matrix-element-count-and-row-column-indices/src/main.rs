// Урок 008. Проверяем размеры таблицы чисел и положение нужной ячейки.
// Для 2 строк и 3 столбцов нужно ровно 6 чисел: 2×3 = 6.
// Номера начинаются с нуля: строки 0 и 1, столбцы 0, 1 и 2.
// Если размеры не подходят или ячейки нет, пропускаем этот случай.
// В плоском массиве ячейка находится по индексу row * columns + column:
// сначала пропускаем предыдущие строки, затем доходим до нужного столбца.

fn main() {
    let elements: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for (_description, rows, columns, row, column) in [
        ("допустимая ячейка", 2, 3, 1, 2),
        ("лишний столбец", 2, 4, 1, 2),
        ("строка вне матрицы", 2, 3, 2, 0),
        ("столбец вне матрицы", 2, 3, 1, 3),
    ] {
        if rows * columns != elements.len() {
            let _ = &(elements.len());
            continue;
        }
        if row >= rows || column >= columns {
            continue;
        }
        let _: i32 = elements[row * columns + column];
    }

    plot_matrix_with_two_rows_and_three_columns();
}

// Строим график по результатам урока.
fn plot_matrix_with_two_rows_and_three_columns() {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Форма 2 × 3",
        &[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
    )
    .expect("не удалось сохранить тепловую карту");
}
