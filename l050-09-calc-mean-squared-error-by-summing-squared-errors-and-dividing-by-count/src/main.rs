// Урок 050. Измеряем ошибку числовых прогнозов.
// Для каждой пары «прогноз — правильный ответ» считаем разницу и умножаем её саму на себя.
// Складываем эти ошибки и делим на число примеров.
// Ноль означает точные прогнозы. Большие промахи получают особенно большой вклад:
// ошибка на 2 даёт вклад 4, а ошибка на 10 — вклад 100.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l050_09_calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count::calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count;

fn main() {
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 3] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка на 1", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка на 2", &[2.0, 6.0, 6.0], 4.0 / 3.0),
    ];
    for (_description, predictions, expected) in cases {
        assert!(check_f64_eq_1e_minus_10(
            calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
                &targets,
                predictions,
            )
            .expect("нужен непустой набор прогнозов и правильных ответов одинаковой длины"),
            expected
        ));
    }
    let _: &str = calc_mean_squared_error_by_summing_squared_errors_and_dividing_by_count(
        &targets,
        &[2.0, 4.0],
    )
    .expect_err("ожидалась ошибка: число прогнозов и ответов различается");
}
