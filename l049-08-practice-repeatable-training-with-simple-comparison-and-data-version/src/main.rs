// Урок 049. Готовим опыт так, чтобы его можно было повторить и понять, что именно изменилось.
// Фиксируем начальное число для случайных выборов, настройки и версию данных.
// Сравниваем качество с простым исходным способом прогнозирования.
// Записываем настройки вместе с результатом, чтобы не перепутать разные запуски.

fn main() {
    const SAMPLE_DATA: &str = "1,0\n2,0\n3,1\n4,1\n";

    let (_random_state, _baseline_accuracy): (u64, f64) = (|| -> (u64, f64) {
        let seed: u64 = std::env::args()
            .nth(1)
            .map(|seed_text| {
                seed_text
                    .parse::<u64>()
                    .expect("seed должен быть целым неотрицательным числом")
            })
            .unwrap_or(42);
        let baseline_correct_prediction_share: f64 = SAMPLE_DATA
            .lines()
            .filter(|line| line.ends_with(",1"))
            .count() as f64
            / SAMPLE_DATA.lines().count() as f64;
        (
            seed.wrapping_mul(6364136223846793005).wrapping_add(1),
            baseline_correct_prediction_share,
        )
    })();

    let _ = &((|| -> u64 {
        let data: &str = SAMPLE_DATA;

        let mut hasher: std::collections::hash_map::DefaultHasher =
            std::collections::hash_map::DefaultHasher::new();

        std::hash::Hash::hash(data, &mut hasher);

        std::hash::Hasher::finish(&hasher)
    })());
}
