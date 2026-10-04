// Урок 016. Если вычисление состоит из нескольких шагов, изменение проходит через каждый шаг.
// Здесь сначала считаем 2*x + 1, затем умножаем полученное число само на себя.
// Первый шаг меняется со скоростью 2, второй — со скоростью 2*(2*x + 1).
// Перемножаем эти скорости, чтобы узнать влияние маленького изменения исходного x.
// При x=3 получаем 2*14 = 28.

use lesson_float_comparison::compare_2_floats_for_approximate_equality;

fn main() {
    let x: f64 = 3.0;
    let step: f64 = 0.001;
    let inner = 2.0 * x + 1.0;
    let output = inner * inner;
    let next_inner = 2.0 * (x + step) + 1.0;
    let next_output = next_inner * next_inner;
    let inner_derivative = 2.0;
    let outer_derivative = 2.0 * inner;
    let derivative = outer_derivative * inner_derivative;
    println!("x={x} -> 2*x+1={inner} -> квадрат={output}");
    println!(
        "x={} -> 2*x+1={next_inner} -> квадрат={next_output}",
        x + step
    );
    println!("Скорости шагов: {inner_derivative} и {outer_derivative}; общая={derivative}");
    let actual_change = next_output - output;
    println!(
        "Изменение={actual_change:.6}; предсказание={:.6}",
        derivative * step
    );
    assert!(compare_2_floats_for_approximate_equality(
        actual_change,
        derivative * step,
        5e-6
    ));
    // Если забыть внутренний множитель 2, оценка будет примерно вдвое меньше.
    assert!((actual_change - outer_derivative * step).abs() > 0.01);
}

// Чему учит этот урок:
// Учимся прослеживать изменение через два последовательных вычисления.
// Проверяем произведение производных шагов и видим ошибку, если забыть влияние внутреннего шага.
