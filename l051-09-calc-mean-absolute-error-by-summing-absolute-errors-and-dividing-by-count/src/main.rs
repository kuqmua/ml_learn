// Урок 051. Измеряем среднюю величину промаха прогноза.
// Из прогноза вычитаем правильный ответ и берём величину разницы без знака.
// Складываем результаты и делим на число примеров. Ноль означает точные прогнозы.
// Если прогнозы ошиблись на −2 и +4, средняя величина ошибки равна (2+4)/2 = 3.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l051_09_calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count::calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count;

fn main() {
    let targets: [f64; 3] = [2.0, 4.0, 6.0];
    let cases: [(&str, &[f64], f64); 4] = [
        ("точный прогноз", &[2.0, 4.0, 6.0], 0.0),
        ("ошибка выше ответа", &[2.0, 5.0, 6.0], 1.0 / 3.0),
        ("ошибка ниже ответа", &[2.0, 3.0, 6.0], 1.0 / 3.0),
        ("ошибка вдвое больше", &[2.0, 6.0, 6.0], 2.0 / 3.0),
    ];
    for (_description, predictions, expected) in cases {
        assert!(check_f64_eq_1e_minus_10(
            calc_mean_absolute_error_by_summing_absolute_errors_and_dividing_by_count(
                &targets,
                predictions,
            )
            .expect("нужен непустой набор прогнозов и правильных ответов одинаковой длины"),
            expected
        ));
    }
}
