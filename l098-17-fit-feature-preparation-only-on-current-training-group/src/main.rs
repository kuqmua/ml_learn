// Урок 098. Вычислять параметры подготовки признаков только по текущей обучающей группе.
// Проверочное значение преобразуем с обучающим средним, не позволяя ему влиять на это среднее.

fn main() {
    let training_data: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    let training_mean: f64 = training_data.iter().sum::<f64>() / training_data.len() as f64;
    let validation: [f64; 1] = [100.0];
    let validation_centered: f64 = validation[0] - training_mean;

    // Выполняем вычисления из примера.
    let _ = (&training_mean, &validation_centered);

    println!(
        "Среднее обучения={training_mean}; проверочное значение={} -> {validation_centered}",
        validation[0]
    );
    assert_eq!(training_mean, 2.0);
    assert_eq!(validation_centered, 98.0);
    let wrong_mean = (training_data.iter().sum::<f64>() + validation[0]) / 4.0;
    println!(
        "Если включить проверку в среднее: {wrong_mean}; это уже подготовка с подсматриванием."
    );
    assert_ne!(wrong_mean, training_mean);
}

// Чему учит этот урок:
// Учимся вычислять параметры подготовки признаков только по текущей обучающей группе.
// Проверочное значение преобразуем с обучающим средним, не позволяя ему влиять на это среднее.
