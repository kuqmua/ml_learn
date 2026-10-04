// Урок 008. Проверять размеры матрицы и границы индексов до чтения элемента.
// Переводим строку и столбец в индекс плоского списка, чтобы безопасно находить нужную ячейку.

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

    for (rows, columns, row, column) in [(2, 3, 1, 2), (2, 4, 1, 2), (2, 3, 2, 0), (2, 3, 1, 3)] {
        let result = if rows * columns != elements.len() {
            Err("размеры не соответствуют числу элементов")
        } else if row >= rows || column >= columns {
            Err("ячейка вне матрицы")
        } else {
            Ok(elements[row * columns + column])
        };
        println!("Матрица {rows}x{columns}, ячейка ({row},{column}): {result:?}");
    }
    assert_eq!(elements[1 * 3 + 2], 6);
}

// Чему учит этот урок:
// Учимся проверять размеры матрицы и границы индексов до чтения элемента.
// Переводим строку и столбец в индекс плоского списка, чтобы безопасно находить нужную ячейку.
