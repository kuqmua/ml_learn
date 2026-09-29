// Самопроверка: рассчитай квантили самостоятельно до запуска урока.
#[test]
#[ignore = "заполни ответы и запусти тест с --ignored"]
fn calculate_quantiles_by_lesson_rule() {
    let sorted: [i32; 5] = [1, 3, 5, 7, 9];
    // Урок использует индекс floor((n−1)·доля), без интерполяции.
    let values_at_zero_half_three_quarters_and_one: Option<[i32; 4]> = None;
    let values: [i32; 4] = values_at_zero_half_three_quarters_and_one
        .expect("впиши четыре значения в Some([..., ..., ..., ...])");
    assert_eq!(values, [1, 5, 7, 9]);
    for (fraction, expected) in [
        (0.0_f64, values[0]),
        (0.5, values[1]),
        (0.75, values[2]),
        (1.0, values[3]),
    ] {
        let index: usize = ((sorted.len() - 1) as f64 * fraction) as usize;
        assert_eq!(sorted[index], expected);
    }
}
