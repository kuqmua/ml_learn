// Урок 125. Обновлять вес сразу после каждого обучающего примера.
// Сохраняем историю, чтобы проследить, как последовательные ошибки меняют параметр.

fn main() {
    let mut weight: f64 = 0.0;
    let mut weight_history: [(f64, f64); 3] = [(0.0, weight); 3];
    let examples: [(f64, f64); 2] = [(1.0, 2.0), (2.0, 4.0)];
    for (step, (feature, target)) in examples.into_iter().enumerate() {
        let loss_slope: f64 = 2.0 * (weight * feature - target) * feature;
        weight -= 0.1 * loss_slope;

        weight_history[step + 1] = ((step + 1) as f64, weight);
    }

    // Выполняем вычисления из примера.
    let _ = weight_history;

    println!("Последовательные обновления (шаг, вес): {weight_history:?}");
    assert_ne!(weight_history[0].1, weight_history[1].1);
    assert_ne!(weight_history[1].1, weight_history[2].1);
    assert!((weight_history[2].1 - 2.0).abs() < (weight_history[0].1 - 2.0).abs());
}

// Чему учит этот урок:
// Учимся обновлять вес сразу после каждого обучающего примера.
// Сохраняем историю, чтобы проследить, как последовательные ошибки меняют параметр.
