// Урок 126. Учитывать прошлое направление изменения вместе с текущей производной.
// Накопленное значение участвует в следующем шаге: это обновление с инерцией, называемое momentum.

fn main() {
    let mut accumulated_gradient_for_momentum_update: f64 = 0.0;
    let mut weight: f64 = 1.0;
    let mut weight_history: [(f64, f64); 4] = [(0.0, weight); 4];
    let mut velocity_history: [(f64, f64); 4] =
        [(0.0, accumulated_gradient_for_momentum_update); 4];
    let rates_of_change: [f64; 3] = [2.0, 1.0, -0.5];
    for (step, rate_of_change) in rates_of_change.into_iter().enumerate() {
        accumulated_gradient_for_momentum_update =
            0.8 * accumulated_gradient_for_momentum_update + rate_of_change;
        weight -= 0.1 * accumulated_gradient_for_momentum_update;

        weight_history[step + 1] = ((step + 1) as f64, weight);
        velocity_history[step + 1] = ((step + 1) as f64, accumulated_gradient_for_momentum_update);
    }

    // Выполняем вычисления из примера.
    let _ = (&weight_history, &velocity_history);

    println!(
        "История веса с инерцией={weight_history:?}; накопленное направление={velocity_history:?}"
    );
    let mut plain = 1.0;
    for gradient in rates_of_change {
        plain -= 0.1 * gradient;
    }
    println!("Без инерции итоговый вес={plain}, с инерцией={weight}");
    assert!((plain - weight).abs() > 0.01);
}

// Чему учит этот урок:
// Учимся учитывать прошлое направление изменения вместе с текущей производной.
// Накопленное значение участвует в следующем шаге: это обновление с инерцией, называемое momentum.
