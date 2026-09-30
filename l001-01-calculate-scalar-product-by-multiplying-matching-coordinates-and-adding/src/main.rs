// Урок 01.1. Скалярное произведение: умножение соответствующих координат двух векторов и сложение произведений.
// Зачем здесь эта тема: Скалярное произведение связывает координаты с направлением; оно понадобится
//   для длины, матриц и сходства.
// Почему код устроен так: Берём две координаты и разные углы, чтобы знак суммы можно было проверить
//   вручную.
// Представь: Два стрелочных направления могут смотреть в одну сторону, под прямым углом или в
//   противоположные стороны; знак суммы помогает это различить.
//
// Что изучаем: для двух векторов умножаем координаты с одинаковыми индексами и складываем числа.
// Положительный ответ означает угол меньше 90° (включая 0°), отрицательный — больше 90°
// (включая 180°). Ноль означает прямой угол, если оба вектора ненулевые.
// Сам по себе знак не доказывает точного совпадения направлений.
// Нулевой вектор тоже даёт ноль, хотя направления у него нет.
// Что делает пример: показывает все варианты знака, включая граничные случаи и разную длину.

use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;

fn main() {
    let left: [f64; 2] = [1.0, 2.0];
    let cases: [(&str, &[f64], f64); 6] = [
        ("то же направление", &[2.0, 4.0], 10.0),
        ("острый угол", &[2.0, 1.0], 4.0),
        ("перпендикулярные векторы", &[-2.0, 1.0], 0.0),
        ("тупой угол", &[-3.0, 1.0], -1.0),
        ("противоположные направления", &[-1.0, -2.0], -5.0),
        ("нулевой вектор без направления", &[0.0, 0.0], 0.0),
    ];

    for (_description, right, expected) in cases {
        let sum_after_multiplying_coordinates: f64 =
            calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&left, right)
                .expect("у этой пары одинаковое число координат");
        assert_eq!(sum_after_multiplying_coordinates, expected);
    }

    let too_short: [f64; 1] = [3.0];
    let _error: &str =
        calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&left, &too_short)
            .expect_err("разная длина должна быть отклонена");
    let _ = &(left);
    plot_scalar_product_as_coordinate_product_sum_for_changing_second_coordinate(&left);
}

// Визуализация вынесена из основного сценария урока.
fn plot_scalar_product_as_coordinate_product_sum_for_changing_second_coordinate(left: &[f64; 2]) {
    let chart_points: Vec<(f64, f64)> = (-40..=40)
        .map(|plot_step_index| {
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            let product: f64 =
                calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                    left,
                    &[1.0, horizontal_value],
                )
                .expect("оба вектора имеют две координаты");
            (horizontal_value, product)
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Скалярное произведение [1, 2] · [1, x]",
        "вторая координата правого вектора, x",
        "скалярное произведение",
        &[lesson_visualization::Series {
            name: "вектор [1, 2]",

            points: &chart_points,
        }],
    )
    .expect("не удалось сохранить график");
}
