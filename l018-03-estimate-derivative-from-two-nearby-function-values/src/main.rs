// Урок 018. Оцениваем производную без заранее выведенной формулы.
// Сравниваем значения слева и справа, затем делим изменение ответа на 2*step.
// Производная показывает местную скорость изменения, а не само значение функции.
use l018_03_estimate_derivative_from_two_nearby_function_values::estimate_derivative_from_two_nearby_function_values;
use lesson_float_comparison::check_f64_eq_1e_minus_8;

fn main() {
    let loss = |x: f64| (x - 3.0).powi(2);
    let step = 0.001;
    // Слева от минимума производная отрицательна, в минимуме ноль, справа положительна.
    for x in [0.0, 3.0, 6.0] {
        let left_output = loss(x - step);
        let right_output = loss(x + step);
        let derivative = estimate_derivative_from_two_nearby_function_values(loss, x, step);
        let expected = 2.0 * (x - 3.0);
        println!(
            "x={x}: ошибка={}, слева={left_output}, справа={right_output}",
            loss(x)
        );
        println!(
            "Производная: ({right_output} - {left_output}) / (2 * {step}) = {derivative}; по формуле {expected}"
        );
        assert!(check_f64_eq_1e_minus_8(derivative, expected));
    }
    // Та же функция принимает другие вычисления: постоянный ответ и прямую.
    let constant_derivative =
        estimate_derivative_from_two_nearby_function_values(|_| 7.0, 2.0, step);
    let line_derivative =
        estimate_derivative_from_two_nearby_function_values(|x| 4.0 * x + 1.0, 2.0, step);
    assert!(check_f64_eq_1e_minus_8(constant_derivative, 0.0));
    assert!(check_f64_eq_1e_minus_8(line_derivative, 4.0));
    println!("Постоянная 7: производная {constant_derivative}; прямая 4*x+1: {line_derivative}");

    // Для x² центральная разность случайно точна без учёта округления.
    // На x³ видно приближение: оценка равна 3*x² + step², а производная — 3*x².
    let cube = |x: f64| x.powi(3);
    let x: f64 = 2.0;
    let expected = 3.0 * x.powi(2);
    let mut previous_error = f64::INFINITY;
    for step in [1.0, 0.1, 0.001] {
        let estimate = estimate_derivative_from_two_nearby_function_values(cube, x, step);
        let error = (estimate - expected).abs();
        println!(
            "x³ при x=2, шаг={step}: оценка={estimate}, точная производная={expected}, погрешность={error}"
        );
        assert!(check_f64_eq_1e_minus_8(estimate, expected + step * step));
        assert!(error < previous_error);
        previous_error = error;
    }
    let tiny_step = 1e-20;
    let estimate = estimate_derivative_from_two_nearby_function_values(cube, x, tiny_step);
    assert_eq!(x + tiny_step, x);
    assert_eq!(x - tiny_step, x);
    assert_eq!(estimate, 0.0);
    println!("Шаг 1e-20: обе точки округлились до {x}; оценка {estimate} вместо {expected}.");

    // Применение: один шаг уменьшения ошибки с численно вычисленной производной.
    let mut parameter = 0.0;
    let before = loss(parameter);
    let derivative = estimate_derivative_from_two_nearby_function_values(loss, parameter, 0.001);
    parameter -= 0.1 * derivative;
    let after = loss(parameter);
    println!("Один шаг: параметр 0 -> {parameter}; ошибка {before} -> {after}");
    assert!(after < before);
    // Это ещё не минимум; повторение шагов разбирается в уроке 021.
    assert!(after > 0.0);
}

// Чему учит этот урок:
// Получаем численную производную разных функций одной общей функцией.
// Различаем значение функции и её производную, проверяем знаки слева и справа от минимума.
// На кубе видим погрешность большого шага, на слишком малом шаге — потерю точности f64.
// Используем оценку для одного обновления параметра; это не полный цикл обучения.
