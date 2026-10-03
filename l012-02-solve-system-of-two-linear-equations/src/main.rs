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
        [
            first_row_first_coef,
            first_row_second_coef,
            second_row_first_coef,
            second_row_second_coef,
        ],
        [first_right_hand_side, second_right_hand_side],
    ) in cases
    {
        let determinant: f64 = first_row_first_coef * second_row_second_coef
            - first_row_second_coef * second_row_first_coef;
        if determinant != 0.0 {
            let _: f64 = (first_right_hand_side * second_row_second_coef
                - first_row_second_coef * second_right_hand_side)
                / determinant;
            let _: f64 = (first_row_first_coef * second_right_hand_side
                - first_right_hand_side * second_row_first_coef)
                / determinant;
        } else {
            let first_replaced: f64 = first_right_hand_side * second_row_second_coef
                - first_row_second_coef * second_right_hand_side;
            let second_replaced: f64 = first_row_first_coef * second_right_hand_side
                - first_right_hand_side * second_row_first_coef;
            let impossible_zero_row: bool = (first_row_first_coef == 0.0
                && first_row_second_coef == 0.0
                && first_right_hand_side != 0.0)
                || (second_row_first_coef == 0.0
                    && second_row_second_coef == 0.0
                    && second_right_hand_side != 0.0);
            let actual: &str =
                if first_replaced == 0.0 && second_replaced == 0.0 && !impossible_zero_row {
                    "бесконечно много решений"
                } else {
                    "решений нет"
                };
            assert_eq!(actual, description);
        }
    }
}
