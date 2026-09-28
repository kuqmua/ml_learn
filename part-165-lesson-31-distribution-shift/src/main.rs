// Урок 31.1. Сдвиг распределения.
//
// Сравнение средних — первый сигнал: при похожих данных разница мала, при сдвиге растёт.
// Совпадение средних само по себе не доказывает совпадения распределений.

fn main() {
    let reference = [1.0, 2.0, 3.0];
    let cases = [
        ("без сдвига среднего", [3.0, 2.0, 1.0], 0.0),
        ("сдвиг к большим значениям", [5.0, 6.0, 7.0], 4.0),
        ("то же среднее, другой разброс", [0.0, 2.0, 4.0], 0.0),
    ];
    assert!(!reference.is_empty());
    let reference_mean = lesson_029::mean(&reference).unwrap();
    for (description, current, expected_difference) in cases {
        assert!(!current.is_empty());
        let current_mean = lesson_029::mean(&current).unwrap();
        let difference = current_mean - reference_mean;
        assert_eq!(difference, expected_difference);
        println!(
            "{description}: эталон={reference_mean}, новые данные={current_mean}, разница={difference}"
        );
    }
    // Показываем значения, рассчитанные по данным примера.
    let chart_points_0: Vec<(f64, f64)> = reference
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v))
        .collect();
    let chart_points_1: Vec<(f64, f64)> = cases[1]
        .1
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v))
        .collect();
    let chart_points_2: Vec<(f64, f64)> = cases[2]
        .1
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v))
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сдвиг среднего",
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "эталон",
                points: &chart_points_0,
            },
            lesson_visualization::Series {
                name: "сдвиг",
                points: &chart_points_1,
            },
            lesson_visualization::Series {
                name: "изменение разброса",
                points: &chart_points_2,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
