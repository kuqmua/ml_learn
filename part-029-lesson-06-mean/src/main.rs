// Урок 06.1. Среднее арифметическое.
//
// Складываем значения и делим на их количество. Для одного значения среднее равно ему,
// отрицательные числа могут уменьшить среднее, а для пустого набора делить не на что.

fn main() {
    let cases: [(&str, &[f64], f64); 3] = [
        ("несколько положительных", &[2.0, 4.0, 6.0], 4.0),
        ("одно значение", &[7.0], 7.0),
        ("значения разных знаков", &[-2.0, 2.0], 0.0),
    ];
    for (description, values, expected) in cases {
        // Общая функция среднего повторно понадобится в дисперсии и нормализации.
        let mean = part_029_lesson_06_mean::mean(values).expect("в этой строке есть значения");
        assert_eq!(mean, expected);
        println!("{description}: {values:?} → среднее {mean}");
    }
    let empty: [f64; 0] = [];
    let error = part_029_lesson_06_mean::mean(&empty)
        .expect_err("среднее пустого набора должно быть отклонено");
    println!("пустой набор: {error}");
    // Наглядное представление величин из этого урока.
    let chart_points_0: Vec<(f64, f64)> = [(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)].to_vec();
    let chart_points_1: Vec<(f64, f64)> = [(1.0, 4.0), (3.0, 4.0)].to_vec();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Среднее и отдельные значения",
        "индекс",
        "значение",
        &[
            lesson_visualization::Series {
                name: "наблюдения",
                points: &chart_points_0,
            },
            lesson_visualization::Series {
                name: "среднее",
                points: &chart_points_1,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
