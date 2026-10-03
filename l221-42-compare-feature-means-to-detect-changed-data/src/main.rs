// Урок 42.1. Сравнение средних значений признаков для обнаружения изменений данных.
// Зачем здесь эта тема: После выпуска модели входные данные могут измениться без изменения кода.
// Почему код устроен так: Сравниваем статистики признаков старых и новых строк при одинаковой
//   схеме.
// Представь: Если после выпуска средний возраст входных клиентов изменился, это повод проверить
//   модель.
//
// Сравнение средних — первый сигнал: при похожих данных разница мала, при сдвиге растёт.
// Совпадение средних само по себе не доказывает совпадения распределений.

use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let reference: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(!reference.is_empty());
    let reference_mean: f64 =
        calc_mean_by_summing_values_and_dividing_by_count(&reference).unwrap();
    let cases: [(&str, [f64; 3], f64); 3] = [
        ("без сдвига среднего", [3.0, 2.0, 1.0], 0.0),
        ("сдвиг к большим значениям", [5.0, 6.0, 7.0], 4.0),
        ("то же среднее, другой разброс", [0.0, 2.0, 4.0], 0.0),
    ];
    for (_description, current, expected_diff) in cases {
        assert!(!current.is_empty());

        let diff: f64 =
            calc_mean_by_summing_values_and_dividing_by_count(&current).unwrap() - reference_mean;
        assert_eq!(diff, expected_diff);
    }

    // Выполняем вычисления из примера.
    let _ = (reference, cases);
}
