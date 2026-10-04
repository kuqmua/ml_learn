// Урок 012. Ищем два числа, которые одновременно подходят под два уравнения.
// Для уравнений a*x + b*y = u и c*x + d*y = v сначала считаем a*d - b*c.
// Если это число не равно нулю, существует ровно одна пара ответов x и y.
// Если оно равно нулю, ответов может не быть или их может быть бесконечно много.
// Поэтому этот случай проверяем отдельно и не делим на ноль.

fn main() {
    let cases: [(&str, [f64; 4], [f64; 2]); 5] = [
        ("одно решение", [2.0, 1.0, 1.0, -1.0], [5.0, 1.0]),
        ("бесконечно много решений", [1.0, 1.0, 2.0, 2.0], [3.0, 6.0]),
        ("решений нет", [1.0, 1.0, 2.0, 2.0], [3.0, 7.0]),
        ("бесконечно много решений", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0]),
        ("решений нет", [0.0, 0.0, 0.0, 0.0], [1.0, 0.0]),
    ];
    for (
        description,
        [row1_coef1, row1_coef2, row2_coef1, row2_coef2],
        [right_hand_side1, right_hand_side2],
    ) in cases
    {
        let determinant: f64 = row1_coef1 * row2_coef2 - row1_coef2 * row2_coef1;
        if determinant != 0.0 {
            let _: f64 =
                (right_hand_side1 * row2_coef2 - row1_coef2 * right_hand_side2) / determinant;
            let _: f64 =
                (row1_coef1 * right_hand_side2 - right_hand_side1 * row2_coef1) / determinant;
        } else {
            let replaced1: f64 = right_hand_side1 * row2_coef2 - row1_coef2 * right_hand_side2;
            let replaced2: f64 = row1_coef1 * right_hand_side2 - right_hand_side1 * row2_coef1;
            let impossible_zero_row: bool =
                (row1_coef1 == 0.0 && row1_coef2 == 0.0 && right_hand_side1 != 0.0)
                    || (row2_coef1 == 0.0 && row2_coef2 == 0.0 && right_hand_side2 != 0.0);
            let actual: &str = if replaced1 == 0.0 && replaced2 == 0.0 && !impossible_zero_row {
                "бесконечно много решений"
            } else {
                "решений нет"
            };
            assert_eq!(actual, description);
        }
    }
}
