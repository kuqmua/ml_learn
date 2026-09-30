// Урок 42.1. Сравнение средних значений признаков для обнаружения изменений данных.
// Связь с принятой терминологией: Сравнение средних признаков для обнаружения сдвига распределения.
// Зачем здесь эта тема: После выпуска модели входные данные могут измениться без изменения кода.
// Почему код устроен так: Сравниваем статистики признаков старых и новых строк при одинаковой
//   схеме.
// Представь: Если после выпуска средний возраст входных клиентов изменился, это повод проверить
//   модель.
//
// Сравнение средних — первый сигнал: при похожих данных разница мала, при сдвиге растёт.
// Совпадение средних само по себе не доказывает совпадения распределений.

use l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let reference: [f64; 3] = [1.0, 2.0, 3.0];
    let cases: [(&str, [f64; 3], f64); 3] = [
        ("без сдвига среднего", [3.0, 2.0, 1.0], 0.0),
        ("сдвиг к большим значениям", [5.0, 6.0, 7.0], 4.0),
        ("то же среднее, другой разброс", [0.0, 2.0, 4.0], 0.0),
    ];
    assert!(!reference.is_empty());
    let reference_mean: f64 =
        calculate_mean_by_summing_values_and_dividing_by_count(&reference).unwrap();
    for (_description, current, expected_difference) in cases {
        assert!(!current.is_empty());
        let current_mean: f64 =
            calculate_mean_by_summing_values_and_dividing_by_count(&current).unwrap();
        let difference: f64 = current_mean - reference_mean;
        assert_eq!(difference, expected_difference);
    }

    plot_reference_shifted_and_more_spread_out_feature_values(reference, cases);
}

// Строим график по результатам урока.
fn plot_reference_shifted_and_more_spread_out_feature_values(
    reference: [f64; 3],
    cases: [(&str, [f64; 3], f64); 3],
) {
    let reference_distribution_points: Vec<(f64, f64)> = reference
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let shifted_distribution_points: Vec<(f64, f64)> = cases[1]
        .1
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let spread_distribution_points: Vec<(f64, f64)> = cases[2]
        .1
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сдвиг среднего",
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "эталон",

                points: &reference_distribution_points,
            },
            lesson_visualization::Series {
                name: "сдвиг",

                points: &shifted_distribution_points,
            },
            lesson_visualization::Series {
                name: "изменение разброса",

                points: &spread_distribution_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
