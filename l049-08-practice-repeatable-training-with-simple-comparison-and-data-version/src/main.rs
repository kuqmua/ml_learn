// Урок 049. Соединять начальное случайное состояние, простую оценку качества и отпечаток данных.
// Эти сведения помогают описать условия повторяемого эксперимента; полноценного обучения в этом
// примере нет.

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

    let seed = std::env::args()
        .nth(1)
        .map(|s| s.parse::<u64>().unwrap())
        .unwrap_or(42);
    let class1_count = SAMPLE_DATA
        .lines()
        .filter(|line| line.ends_with(",1"))
        .count();
    let count = SAMPLE_DATA.lines().count();
    let baseline = class1_count.max(count - class1_count) as f64 / count as f64;
    println!(
        "Начальное состояние={seed}; примеров={count}; точность постоянного класса={baseline}"
    );
    let fingerprint = |text: &str| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(text, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    };
    assert_eq!(fingerprint(SAMPLE_DATA), fingerprint(SAMPLE_DATA));
    assert_ne!(
        fingerprint(SAMPLE_DATA),
        fingerprint("1,1\n2,0\n3,1\n4,1\n")
    );
    println!("Отпечаток данных={}", fingerprint(SAMPLE_DATA));
}

// Чему учит этот урок:
// Учимся соединять начальное случайное состояние, простую оценку качества и отпечаток данных.
// Эти сведения помогают описать условия повторяемого эксперимента; полноценного обучения в этом
// примере нет.
