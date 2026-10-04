// Урок 115. Различать входные данные и изменяемые параметры модели.
// Меняем вес и постоянную прибавку при том же входе и получаем другой прогноз.

fn main() {
    let input = 2.0;
    let weight = 0.5;
    let bias = 0.1;
    let before = weight * input + bias;
    let changed_weight = (weight + 0.2) * input + bias;
    let changed_bias = weight * input + bias + 0.2;
    println!(
        "Тот же вход={input}: исходный ответ={before}, вес +0.2 -> {changed_weight}, прибавка +0.2 -> {changed_bias}"
    );
    assert!((changed_weight - before - 0.4_f64).abs() < 1e-12);
    assert!((changed_bias - before - 0.2_f64).abs() < 1e-12);
}

// Чему учит этот урок:
// Учимся различать входные данные и изменяемые параметры модели.
// Меняем вес и постоянную прибавку при том же входе и получаем другой прогноз.
