// Урок 023. Считать производную ошибки по одному примеру и в среднем по всему набору.
// Это объясняет, почему обновления по отдельным примерам могут отличаться от обновления по всей
// выборке.

fn main() {
    let examples = [(1.0_f64, 2.0_f64), (2.0, 4.0)];
    let weight = 0.0;
    let slopes = examples.map(|(x, y)| 2.0 * (weight * x - y) * x);
    let mean_slope = slopes.iter().sum::<f64>() / slopes.len() as f64;
    let batch_weight = weight - 0.1 * mean_slope;
    let single_weight = weight - 0.1 * slopes[0];
    let loss = |w: f64| {
        examples
            .iter()
            .map(|(x, y)| (w * x - y).powi(2))
            .sum::<f64>()
            / examples.len() as f64
    };
    println!("Производные отдельных примеров={slopes:?}; средняя={mean_slope}");
    println!(
        "По первому примеру: вес={single_weight}, общая ошибка={}",
        loss(single_weight)
    );
    println!(
        "По всему набору: вес={batch_weight}, общая ошибка={}",
        loss(batch_weight)
    );
    assert_ne!(single_weight, batch_weight);
    assert!(loss(single_weight) < loss(weight));
    assert!(loss(batch_weight) < loss(weight));
}

// Чему учит этот урок:
// Учимся считать производную ошибки по одному примеру и в среднем по всему набору.
// Это объясняет, почему обновления по отдельным примерам могут отличаться от обновления по всей
// выборке.
