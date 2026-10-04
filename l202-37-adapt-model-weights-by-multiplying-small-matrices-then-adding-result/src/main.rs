// Урок 202. Обучаем поправку малого ранга, сохраняя исходную матрицу.
// Прогноз = W*x + B*(A*x). Здесь A — строка из 4 весов, B — столбец из 4 весов.
// Вместо изменения 16 весов W обучаем 8 весов A и B.

use l018_03_estimate_derivative_from_two_nearby_function_values::estimate_derivative_from_two_nearby_function_values;
use lesson_float_comparison::check_f64_eq_1e_minus_7;

fn main() {
    let frozen = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let original = frozen;
    let predict = |input: [f64; 4], a: [f64; 4], b: [f64; 4]| {
        let projected = a.iter().zip(input).map(|(w, x)| w * x).sum::<f64>();
        std::array::from_fn::<f64, 4, _>(|row| {
            let base = frozen[row]
                .iter()
                .zip(input)
                .map(|(w, x)| w * x)
                .sum::<f64>();
            base + b[row] * projected
        })
    };
    // Новая задача требует добавлять разные доли первой координаты к каждому выходу.
    let target = |x: [f64; 4]| {
        [
            2.0 * x[0],
            x[1] + 0.5 * x[0],
            x[2] - 0.25 * x[0],
            x[3] + x[0],
        ]
    };
    let training = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let loss = |a, b| {
        training
            .iter()
            .map(|&input| {
                let prediction = predict(input, a, b);
                let expected = target(input);
                (0..4)
                    .map(|row| (prediction[row] - expected[row]).powi(2))
                    .sum::<f64>()
            })
            .sum::<f64>()
            / 16.0
    };
    // Обе части нельзя начинать с нулей: тогда обе производные останутся нулевыми.
    let initial_a = [0.1, 0.2, -0.1, 0.05];
    let mut a = initial_a;
    let mut b = [0.0; 4];
    let before = loss(a, b);
    let gradients = |a: [f64; 4], b: [f64; 4]| {
        let mut gradient_a = [0.0; 4];
        let mut gradient_b = [0.0; 4];
        for input in training {
            let projected = a.iter().zip(input).map(|(w, x)| w * x).sum::<f64>();
            let prediction = predict(input, a, b);
            let expected = target(input);
            for row in 0..4 {
                let output_derivative = 2.0 * (prediction[row] - expected[row]) / 16.0;
                gradient_b[row] += output_derivative * projected;
                for column in 0..4 {
                    gradient_a[column] += output_derivative * b[row] * input[column];
                }
            }
        }
        (gradient_a, gradient_b)
    };
    // Проверяем все 8 производных численно общей функцией из урока 018.
    let probe_b = [0.2, -0.1, 0.3, 0.4];
    let (derivatives_a, derivatives_b) = gradients(initial_a, probe_b);
    for index in 0..4 {
        let numerical_a = estimate_derivative_from_two_nearby_function_values(
            |value| {
                let mut changed = initial_a;
                changed[index] = value;
                loss(changed, probe_b)
            },
            initial_a[index],
            1e-4,
        );
        let numerical_b = estimate_derivative_from_two_nearby_function_values(
            |value| {
                let mut changed = probe_b;
                changed[index] = value;
                loss(initial_a, changed)
            },
            probe_b[index],
            1e-4,
        );
        assert!(check_f64_eq_1e_minus_7(derivatives_a[index], numerical_a));
        assert!(check_f64_eq_1e_minus_7(derivatives_b[index], numerical_b));
    }
    println!("Все 8 производных совпали с численной проверкой");
    for epoch in 0..2000 {
        let (gradient_a, gradient_b) = gradients(a, b);
        // Обе производные рассчитаны по одним старым весам; лишь теперь обновляем их.
        for index in 0..4 {
            a[index] -= 0.1 * gradient_a[index];
            b[index] -= 0.1 * gradient_b[index];
        }
        if matches!(epoch, 0 | 99 | 499 | 1999) {
            println!("Шаг {}: ошибка={}", epoch + 1, loss(a, b));
        }
    }
    assert_ne!(a, initial_a);
    assert_ne!(b, [0.0; 4]);
    assert_eq!(frozen, original);
    let after = loss(a, b);
    assert!(after < before);
    assert!(after < 1e-8);
    // Новые сочетания координат не участвовали в обучении.
    for input in [[1.0, 2.0, 3.0, 4.0], [-2.0, 1.0, 0.5, -1.0]] {
        let base = predict(input, initial_a, [0.0; 4]);
        let adapted = predict(input, a, b);
        let expected = target(input);
        let base_error = (0..4).map(|i| (base[i] - expected[i]).powi(2)).sum::<f64>();
        let adapted_error = (0..4)
            .map(|i| (adapted[i] - expected[i]).powi(2))
            .sum::<f64>();
        assert!(adapted_error < base_error);
        assert!(adapted_error < 1e-6);
        println!("Вход={input:?}: до={base:?}, после={adapted:?}, цель={expected:?}");
    }
    println!("Основная матрица не изменилась; обучены 8 весов вместо 16: A={a:?}, B={b:?}");
}

// Чему учит этот урок:
// Вычисляем W*x + B*(A*x), выводим производные по A и B и обновляем обе части поправки.
// Проверяем уменьшение ошибки на новых входах и неизменность исходных весов W.
// Это пример поправки ранга 1; такая поправка не может выразить любое изменение матрицы.
