//! Вычисления и примеры урока part-011-lesson-02-linear-system.

// Урок 02.5. Система двух линейных уравнений.
//
// При ненулевом определителе решение одно. При нулевом определителе уравнения
// могут совпадать (решений бесконечно много) или противоречить друг другу (решений нет).

pub fn run() {
    let cases = [
        ("одно решение", [2.0, 1.0, 1.0, -1.0], [5.0, 1.0]),
        ("бесконечно много решений", [1.0, 1.0, 2.0, 2.0], [3.0, 6.0]),
        ("решений нет", [1.0, 1.0, 2.0, 2.0], [3.0, 7.0]),
        ("бесконечно много решений", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0]),
        ("решений нет", [0.0, 0.0, 0.0, 0.0], [1.0, 0.0]),
    ];
    for (description, [a, b, c, d], [first_rhs, second_rhs]) in cases {
        let determinant = a * d - b * c;
        if determinant != 0.0 {
            let x = (first_rhs * d - b * second_rhs) / determinant;
            let y = (a * second_rhs - first_rhs * c) / determinant;
            println!("{description}: x={x}, y={y}");
        } else {
            // Если замена столбца правой частью тоже даёт ноль, обе строки описывают одну прямую.
            let first_replaced = first_rhs * d - b * second_rhs;
            let second_replaced = a * second_rhs - first_rhs * c;
            let impossible_zero_row = (a == 0.0 && b == 0.0 && first_rhs != 0.0)
                || (c == 0.0 && d == 0.0 && second_rhs != 0.0);
            let actual = if first_replaced == 0.0 && second_replaced == 0.0 && !impossible_zero_row
            {
                "бесконечно много решений"
            } else {
                "решений нет"
            };
            assert_eq!(actual, description);
            println!("{description}: определитель равен нулю");
        }
    }
}
