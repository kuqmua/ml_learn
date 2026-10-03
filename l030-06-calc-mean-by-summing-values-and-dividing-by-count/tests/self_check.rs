// Сначала предскажи результат, затем сравни с функцией урока.
use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;

#[test]
#[ignore = "заполни ответы и запусти тест с --ignored"]
fn calc_mean_before_and_after_outlier() {
    let before: Option<f64> = None; // [1, 2, 3]
    let after: Option<f64> = None; // [1, 2, 3, 10]
    assert_eq!(before, Some(2.0));
    assert_eq!(after, Some(4.0));
    assert_eq!(
        calc_mean_by_summing_values_and_dividing_by_count(&[1.0, 2.0, 3.0]).unwrap(),
        before.unwrap()
    );
    assert_eq!(
        calc_mean_by_summing_values_and_dividing_by_count(&[1.0, 2.0, 3.0, 10.0]).unwrap(),
        after.unwrap()
    );
}
