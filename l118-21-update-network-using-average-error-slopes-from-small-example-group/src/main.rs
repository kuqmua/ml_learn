// Урок 118. Усреднять производные ошибки по небольшой группе примеров перед обновлением веса.
// Так получаем один шаг обучения, учитывающий сразу несколько наблюдений.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

fn main() {
    let rates_of_change: [f64; 2] = [2.0, 4.0];
    assert!(
        !rates_of_change.is_empty(),
        "мини-пакет градиентов не должен быть пустым"
    );
    let small_batch_loss_rate_of_change: f64 =
        rates_of_change.iter().sum::<f64>() / rates_of_change.len() as f64;
    let old_weight: f64 = 1.0;
    let learning_rate: f64 = 0.1;
    let new_weight: f64 = old_weight - learning_rate * small_batch_loss_rate_of_change;

    // Выполняем вычисления из примера.
    let _ = (&old_weight, &new_weight);

    println!(
        "Производные примеров={rates_of_change:?}; средняя={small_batch_loss_rate_of_change}; вес {old_weight} -> {new_weight}"
    );
    assert!(check_f64_eq_1e_minus_12(new_weight, 0.7));
    let summed_update = old_weight - learning_rate * rates_of_change.iter().sum::<f64>();
    println!(
        "Если забыть деление на размер группы, получится {summed_update}: шаг зависит от числа примеров."
    );
    assert_ne!(new_weight, summed_update);
}

// Чему учит этот урок:
// Учимся усреднять производные ошибки по небольшой группе примеров перед обновлением веса.
// Так получаем один шаг обучения, учитывающий сразу несколько наблюдений.
