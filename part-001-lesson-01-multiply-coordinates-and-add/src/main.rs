// Урок 01.1. Умножение координат попарно и сложение результатов.
//
// Что изучаем: для двух векторов умножаем координаты с одинаковыми индексами и складываем числа.
// Положительный ответ означает угол меньше 90° (включая 0°), отрицательный — больше 90°
// (включая 180°). Ноль означает прямой угол, если оба вектора ненулевые.
// Сам по себе знак не доказывает точного совпадения направлений.
// Нулевой вектор тоже даёт ноль, хотя направления у него нет.
// Что делает пример: показывает все варианты знака, включая граничные случаи и разную длину.

fn main() {
    // Для всех примеров слева используем один вектор, чтобы было проще сравнивать ответы.
    let left = [1.0, 2.0];
    let cases: [(&str, &[f64], f64); 6] = [
        ("то же направление", &[2.0, 4.0], 10.0),
        ("острый угол", &[2.0, 1.0], 4.0),
        ("перпендикулярные векторы", &[-2.0, 1.0], 0.0),
        ("тупой угол", &[-3.0, 1.0], -1.0),
        ("противоположные направления", &[-1.0, -2.0], -5.0),
        ("нулевой вектор без направления", &[0.0, 0.0], 0.0),
    ];

    for (description, right, expected) in cases {
        // Реализация находится в библиотеке этого урока; её используют и следующие уроки.
        let sum_after_multiplying_coordinates =
            part_001_lesson_01_multiply_coordinates_and_add::multiply_matching_coordinates_then_add(&left, right)
                .expect("у этой пары одинаковое число координат");
        // Например, для [1, 2] и [-2, 1] получаем 1·(-2) + 2·1 = 0.
        assert_eq!(sum_after_multiplying_coordinates, expected);
        println!("{description}: {left:?} и {right:?} → {sum_after_multiplying_coordinates}");
    }

    // Неполную пару отклоняем до вычисления: иначе лишнее значение потеряется.
    let too_short = [3.0];
    let error =
        part_001_lesson_01_multiply_coordinates_and_add::multiply_matching_coordinates_then_add(
            &left, &too_short,
        )
        .expect_err("разная длина должна быть отклонена");
    println!("разная длина: {:?} и {too_short:?} → {error}", left);
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = (-40..=40).map(|i| { let x=i as f64/10.0; (x, part_001_lesson_01_multiply_coordinates_and_add::multiply_matching_coordinates_then_add(&[1.0,2.0], &[1.0,x]).unwrap()) }).collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Скалярное произведение",
        "x второй координаты",
        "скалярное произведение",
        &[lesson_visualization::Series {
            name: "вектор [1, 2]",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
