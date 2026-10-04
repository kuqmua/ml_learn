// Урок 128. Центрируем признак и сравниваем один шаг обучения до и после подготовки.
// Вычитаем среднее, рассчитанное только по обучающим данным.
use l030_06_calc_mean_by_summing_values_and_dividing_by_count::calc_mean_by_summing_values_and_dividing_by_count;

fn main() {
    let inputs = [10.0_f64, 20.0, 30.0];
    let targets = [-10.0, 0.0, 10.0];
    let mean = calc_mean_by_summing_values_and_dividing_by_count(&inputs).unwrap();
    let centered = inputs.map(|x| x - mean);
    assert_eq!(centered, [-10.0, 0.0, 10.0]);
    println!("Входы={inputs:?}; среднее={mean}; после центрирования={centered:?}");
    // В обеих системах координат начальный прогноз одинаков: weight=0, bias=0.
    let mut losses = Vec::new();
    for (label, features) in [("исходные", inputs), ("центрированные", centered)]
    {
        let mut weight = 0.0;
        let mut bias = 0.0;
        let loss = |weight: f64, bias: f64| {
            features
                .iter()
                .zip(targets)
                .map(|(&x, y)| (weight * x + bias - y).powi(2))
                .sum::<f64>()
                / 3.0
        };
        let before = loss(weight, bias);
        let gradient_weight = features
            .iter()
            .zip(targets)
            .map(|(&x, y)| 2.0 * (weight * x + bias - y) * x)
            .sum::<f64>()
            / 3.0;
        let gradient_bias = features
            .iter()
            .zip(targets)
            .map(|(&x, y)| 2.0 * (weight * x + bias - y))
            .sum::<f64>()
            / 3.0;
        weight -= 0.005 * gradient_weight;
        bias -= 0.005 * gradient_bias;
        let after = loss(weight, bias);
        println!("{label}: вес={weight}, прибавка={bias}, ошибка {before} -> {after}");
        losses.push((before, after));
    }
    assert_eq!(losses[0].0, losses[1].0);
    assert!(losses[0].1 > losses[0].0);
    assert!(losses[1].1 < losses[1].0);
    // Это результат выбранного примера и скорости, не гарантия для любого шага.
    let new_input = 40.0;
    assert_eq!(new_input - mean, 20.0);
    println!(
        "Новый вход {new_input} преобразуем прежним обучающим средним: {}",
        new_input - mean
    );
}

// Чему учит этот урок:
// Сравниваем одинаковый начальный прогноз и размер шага при разных представлениях признака.
// Видим, как центрирование меняет поведение обновления, и применяем обучающее среднее к новым входам.
// Центрирование не заменяет выбор скорости обучения и не гарантирует сходимость само по себе.
