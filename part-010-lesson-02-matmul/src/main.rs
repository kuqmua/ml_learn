// Урок 02.4. Умножение матриц.
//
// Каждая ячейка ответа получается попарным умножением строки и столбца со сложением.
// Единичная матрица не меняет значения; порядок множителей обычно влияет на ответ.

fn main() {
    let left = [[1.0, 2.0], [3.0, 4.0]];
    let identity = [[1.0, 0.0], [0.0, 1.0]];
    let right = [[5.0, 6.0], [7.0, 8.0]];
    for (description, first, second, expected) in [
        ("обычный порядок", left, right, [[19.0, 22.0], [43.0, 50.0]]),
        (
            "обратный порядок",
            right,
            left,
            [[23.0, 34.0], [31.0, 46.0]],
        ),
        ("единичная справа", left, identity, left),
        ("единичная слева", identity, left, left),
    ] {
        assert_eq!(
            first[0].len(),
            second.len(),
            "внутренние размеры матриц должны совпадать"
        );
        let mut result = [[0.0; 2]; 2];
        for row in 0..first.len() {
            for column in 0..second[0].len() {
                let column_values = [second[0][column], second[1][column]];
                // Строка × столбец — то же попарное умножение и сложение из урока 01.1.
                result[row][column] =
                    lesson_001::multiply_matching_coordinates_then_add(&first[row], &column_values)
                        .expect("внутренние размеры матриц совпадают");
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
