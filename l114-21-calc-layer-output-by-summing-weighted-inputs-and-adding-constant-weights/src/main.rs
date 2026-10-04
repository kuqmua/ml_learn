// Урок 114. Вычислять выходы нескольких нейронов как суммы входов с весами и постоянными прибавками.
// Одна таблица весов преобразует общий входной вектор в несколько новых признаков.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

fn main() {
    let input: [f64; 2] = [1.0, 2.0];
    let weights: [[f64; 2]; 2] = [[0.5, 0.2], [-0.3, 0.8]];
    let constant_input_weights: [f64; 2] = [0.1, -0.2];
    let mut output: [f64; 2] = [0.0; 2];
    for neuron in 0..2 {
        output[neuron] = constant_input_weights[neuron];
        for feature in 0..2 {
            output[neuron] += weights[neuron][feature] * input[feature];
        }
    }

    // Выполняем вычисления из примера.
    let _ = weights;

    println!(
        "Вход={input:?}; веса={weights:?}; прибавки={constant_input_weights:?}; выход={output:?}"
    );
    assert!(check_f64_eq_1e_minus_12(output[0], 1.0));
    assert!(check_f64_eq_1e_minus_12(output[1], 1.1));
}

// Чему учит этот урок:
// Учимся вычислять выходы нескольких нейронов как суммы входов с весами и постоянными прибавками.
// Одна таблица весов преобразует общий входной вектор в несколько новых признаков.
