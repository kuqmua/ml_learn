// Урок 049. Повторяем обучение с одинаковым seed и сравниваем результаты.
// Seed задаёт начальный вес; данные и настройки вместе определяют эксперимент.
// На отдельных точках сравниваем обученную модель с постоянным прогнозом.

fn main() {
    const DATA: &str = "1,2\n2,4\n3,6\n";
    let training: Vec<(f64, f64)> = DATA
        .lines()
        .map(|row| {
            let (x, y) = row.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let seed: u64 = std::env::args()
        .nth(1)
        .map(|s| {
            s.parse()
                .expect("seed должен быть целым неотрицательным числом")
        })
        .unwrap_or(42);
    let train = |seed: u64| {
        let state = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let mut weight = (state >> 11) as f64 / ((1_u64 << 53) as f64);
        let loss = |weight: f64| {
            training
                .iter()
                .map(|&(x, y)| (weight * x - y).powi(2))
                .sum::<f64>()
                / training.len() as f64
        };
        let mut history = vec![loss(weight)];
        for _ in 0..50 {
            let derivative = training
                .iter()
                .map(|&(x, y)| 2.0 * (weight * x - y) * x)
                .sum::<f64>()
                / training.len() as f64;
            weight -= 0.05 * derivative;
            history.push(loss(weight));
        }
        (weight, history)
    };
    let run1 = train(seed);
    let run2 = train(seed);
    let other = train(seed.wrapping_add(1));
    assert_eq!(run1, run2); // Повторяемость и параметра, и каждого значения ошибки.
    assert_ne!(run1.1[0], other.1[0]);
    assert!(run1.1.last().unwrap() < &run1.1[0]);
    println!(
        "Seed={seed}: вес={}, ошибка {} -> {}; повторный запуск совпал",
        run1.0,
        run1.1[0],
        run1.1.last().unwrap()
    );
    println!("Другой seed меняет начальную ошибку: {}", other.1[0]);
    let baseline = training.iter().map(|&(_, y)| y).sum::<f64>() / training.len() as f64;
    let test = [(4.0, 8.0), (5.0, 10.0)];
    let model_error = test
        .iter()
        .map(|&(x, y)| (run1.0 * x - y).powi(2))
        .sum::<f64>()
        / test.len() as f64;
    let baseline_error = test
        .iter()
        .map(|&(_, y)| (baseline - y).powi(2))
        .sum::<f64>()
        / test.len() as f64;
    assert!(model_error < baseline_error);
    println!("Отдельный тест: ошибка модели={model_error}, постоянного ответа={baseline_error}");
    let fingerprint = |data: &str| {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(data, &mut hasher);
        std::hash::Hasher::finish(&hasher)
    };
    assert_ne!(fingerprint(DATA), fingerprint("1,3\n2,4\n3,6\n"));
    println!(
        "Отпечаток данных={}, скорость=0.05, шагов=50",
        fingerprint(DATA)
    );
    // DefaultHasher здесь лишь демонстрация отпечатка, не стабильный формат версии данных.
}

// Чему учит этот урок:
// Повторяем настоящее обучение с одинаковыми данными, настройками и seed.
// Проверяем совпадение истории и веса, а затем сравниваем модель с baseline вне обучения.
// Отпечаток помогает заметить изменение содержимого данных в этом запуске.
