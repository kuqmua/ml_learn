//! Вычисления и примеры урока part-007-lesson-02-matrix-shape.

// Урок 02.1. Форма матрицы.
//
// Что изучаем: форма rows×columns требует ровно rows*columns элементов.
// Также индекс строки и столбца должен оставаться внутри этих границ.

pub fn run() {
    let elements = [1, 2, 3, 4, 5, 6];
    for (description, rows, columns, row, column) in [
        ("допустимая ячейка", 2, 3, 1, 2),
        ("лишний столбец", 2, 4, 1, 2),
        ("строка вне матрицы", 2, 3, 2, 0),
        ("столбец вне матрицы", 2, 3, 1, 3),
    ] {
        if rows * columns != elements.len() {
            println!(
                "{description}: форма {rows}×{columns} не подходит для {} элементов",
                elements.len()
            );
            continue;
        }
        if row >= rows || column >= columns {
            println!("{description}: ячейка [{row}, {column}] вне формы {rows}×{columns}");
            continue;
        }
        let value = elements[row * columns + column];
        println!("{description}: ячейка [{row}, {column}] = {value}");
    }
}
