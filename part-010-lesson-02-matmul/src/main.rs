// Урок 02.4. Умножение матриц.
//
// Каждая ячейка ответа получается попарным умножением строки и столбца со сложением.
// Единичная матрица не меняет значения; порядок множителей обычно влияет на ответ.

fn main() {
    let left = [[1, 2], [3, 4]];
    let identity = [[1, 0], [0, 1]];
    let right = [[5, 6], [7, 8]];
    for (description, first, second, expected) in [
        ("обычный порядок", left, right, [[19, 22], [43, 50]]),
        ("обратный порядок", right, left, [[23, 34], [31, 46]]),
        ("единичная справа", left, identity, left),
        ("единичная слева", identity, left, left),
    ] {
        assert_eq!(
            first[0].len(),
            second.len(),
            "внутренние размеры матриц должны совпадать"
        );
        let mut result = [[0; 2]; 2];
        for row in 0..first.len() {
            for column in 0..second[0].len() {
                for shared in 0..second.len() {
                    result[row][column] += first[row][shared] * second[shared][column];
                }
            }
        }
        assert_eq!(result, expected);
        println!("{description}: {result:?}");
    }
    let incompatible_left_shape = (2, 3);
    let incompatible_right_shape = (2, 2);
    if incompatible_left_shape.1 != incompatible_right_shape.0 {
        println!(
            "размеры {incompatible_left_shape:?} и {incompatible_right_shape:?}: умножение невозможно"
        );
    }
}
