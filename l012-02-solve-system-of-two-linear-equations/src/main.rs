// Урок 02.5. Поиск двух неизвестных, удовлетворяющих двум линейным уравнениям.
// Связь с принятой терминологией: Решение системы двух линейных уравнений.
// Зачем здесь эта тема: Линейная система ищет вход, который после преобразования матрицей даёт
//   заданный выход.
// Почему код устроен так: Две переменные позволяют увидеть решение и случай нулевого определителя
//   без скрытого численного алгоритма.
// Представь: Если после умножения матрицы на неизвестный [x, y] получен известный ответ, система
//   ищет эти x и y.
//
// При ненулевом определителе решение одно. При нулевом определителе уравнения
// могут совпадать (решений бесконечно много) или противоречить друг другу (решений нет).

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
            first_row_first_coefficient,
            first_row_second_coefficient,
            second_row_first_coefficient,
            second_row_second_coefficient,
        ],
        [first_right_hand_side, second_right_hand_side],
    ) in cases
    {
        let determinant: f64 = first_row_first_coefficient * second_row_second_coefficient
            - first_row_second_coefficient * second_row_first_coefficient;
        if determinant != 0.0 {
            let _first_unknown: f64 = (first_right_hand_side * second_row_second_coefficient
                - first_row_second_coefficient * second_right_hand_side)
                / determinant;
            let _second_unknown: f64 = (first_row_first_coefficient * second_right_hand_side
                - first_right_hand_side * second_row_first_coefficient)
                / determinant;
        } else {
            let first_replaced: f64 = first_right_hand_side * second_row_second_coefficient
                - first_row_second_coefficient * second_right_hand_side;
            let second_replaced: f64 = first_row_first_coefficient * second_right_hand_side
                - first_right_hand_side * second_row_first_coefficient;
            let impossible_zero_row: bool = (first_row_first_coefficient == 0.0
                && first_row_second_coefficient == 0.0
                && first_right_hand_side != 0.0)
                || (second_row_first_coefficient == 0.0
                    && second_row_second_coefficient == 0.0
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

    plot_two_lines_and_their_intersection();
}

// Строим график по результатам урока.
fn plot_two_lines_and_their_intersection() {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Две прямые с единственным пересечением",
        "x",
        "y",
        &[
            lesson_visualization::Series {
                name: "2x+y=5",

                points: &(0..=50)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, 5.0 - 2.0 * horizontal_value)
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "x−y=1",

                points: &(0..=50)
                    .map(|plot_step_index| {
                        let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                        (horizontal_value, horizontal_value - 1.0)
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
