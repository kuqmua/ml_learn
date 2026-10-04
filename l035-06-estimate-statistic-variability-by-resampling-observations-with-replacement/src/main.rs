// Урок 035. Оцениваем изменчивость среднего с помощью повторных случайных выборок.
// Каждый раз берём столько же элементов, сколько в исходных данных, с возвращением.
fn main() {
    let values = [2.0_f64, 4.0, 6.0];
    let bootstrap = |values: &[f64], seed: u64| {
        assert!(!values.is_empty());
        let mut state = seed;
        let mut means = Vec::new();
        for _ in 0..10_000 {
            let mut sum = 0.0;
            for _ in 0..values.len() {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let draw = (state >> 11) as f64 / ((1_u64 << 53) as f64);
                let index = (draw * values.len() as f64) as usize;
                sum += values[index];
            }
            means.push(sum / values.len() as f64);
        }
        means
    };
    let means = bootstrap(&values, 42);
    assert_eq!(means, bootstrap(&values, 42));
    assert!(means.iter().all(|&mean| (2.0..=6.0).contains(&mean)));
    let mean_of_means = means.iter().sum::<f64>() / means.len() as f64;
    let variance = means
        .iter()
        .map(|mean| (mean - mean_of_means).powi(2))
        .sum::<f64>()
        / (means.len() - 1) as f64;
    let standard_error = variance.sqrt();
    println!("Первые 10 средних: {:?}", &means[..10]);
    println!(
        "Исходное среднее=4; среднее повторных выборок={mean_of_means}; оценка стандартной ошибки={standard_error}"
    );
    assert!(standard_error > 0.8 && standard_error < 1.1);
    // Если все исходные значения одинаковы, повторный выбор не меняет среднее.
    let identical = bootstrap(&[4.0, 4.0, 4.0], 42);
    assert!(identical.iter().all(|&mean| mean == 4.0));
    println!("Для [4,4,4] все средние равны 4, их разброс равен 0.");
    // Bootstrap отражает только исходную выборку: три числа не представляют всю популяцию.
}

// Чему учит этот урок:
// Строим множество повторных выборок с возвращением и считаем среднее каждой.
// Разброс этих средних оценивает изменчивость статистики, а не разброс отдельных наблюдений.
// Проверяем воспроизводимость и случай нулевого разброса исходных данных.
