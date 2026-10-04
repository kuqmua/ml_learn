// Урок 131. Соединять инерцию, ограничение производной и остановку по отдельной проверочной выборке.
// Сохраняем лучший вес по проверочной ошибке, чтобы не возвращать автоматически вес последнего
// шага.

fn main() {
    // Два отдельных набора: подбираем вес по training, останавливаем по validation.
    let training = [(1.0_f64, 3.0_f64)];
    let validation = [(1.0_f64, 2.5_f64)];
    let error = |weight: f64, rows: &[(f64, f64)]| {
        rows.iter()
            .map(|(x, y)| (weight * x - y).powi(2))
            .sum::<f64>()
            / rows.len() as f64
    };
    for momentum in [0.0, 0.8] {
        let mut weight = 8.0;
        let mut velocity = 0.0;
        let mut best_weight = weight;
        let mut best_loss = error(weight, &validation);
        let mut stale = 0;
        for epoch in 0..200 {
            let slope = training
                .iter()
                .map(|(x, y)| 2.0 * (weight * x - y) * x)
                .sum::<f64>()
                / training.len() as f64;
            let clipped = slope.clamp(-1.0, 1.0);
            velocity = momentum * velocity + clipped;
            weight -= 0.1 * velocity;
            let validation_error = error(weight, &validation);
            if validation_error < best_loss {
                best_loss = validation_error;
                best_weight = weight;
                stale = 0;
            } else {
                stale += 1;
            }
            if epoch % 10 == 0 {
                println!(
                    "Инерция={momentum}, шаг={epoch}: вес={weight:.4}, ошибка обучения={:.4}, проверки={validation_error:.4}",
                    error(weight, &training)
                );
            }
            if stale >= 12 {
                println!("Остановка после 12 шагов без улучшения проверки");
                break;
            }
        }
        println!("Возвращаем лучший вес={best_weight}, ошибка проверки={best_loss}");
        assert!(best_loss < error(8.0, &validation));
        assert!(best_loss <= error(weight, &validation));
    }
}

// Чему учит этот урок:
// Учимся соединять инерцию, ограничение производной и остановку по отдельной проверочной выборке.
// Сохраняем лучший вес по проверочной ошибке, чтобы не возвращать автоматически вес последнего
// шага.
