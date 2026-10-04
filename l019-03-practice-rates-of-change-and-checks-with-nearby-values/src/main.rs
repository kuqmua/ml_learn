// Урок 019. Проверяем обе компоненты градиента общей функцией из урока 018.
// Меняем один вход, удерживая другой: так численно получаем частные производные.
use l018_03_estimate_derivative_from_two_nearby_function_values::estimate_derivative_from_two_nearby_function_values;
use lesson_float_comparison::{check_f64_eq_1e_minus_6, check_f64_eq_1e_minus_7};

fn main() {
    let loss = |x: f64, y: f64| (x - 2.0).powi(2) + 3.0 * (y + 1.0).powi(2);
    for (x, y) in [(0.0, 0.0), (2.0, -1.0), (4.0, -2.0)] {
        let analytical = [2.0 * (x - 2.0), 6.0 * (y + 1.0)];
        for step in [1e-2, 1e-4, 1e-8] {
            let numerical = [
                estimate_derivative_from_two_nearby_function_values(
                    |new_x| loss(new_x, y),
                    x,
                    step,
                ),
                estimate_derivative_from_two_nearby_function_values(
                    |new_y| loss(x, new_y),
                    y,
                    step,
                ),
            ];
            println!(
                "Точка ({x}, {y}), шаг={step:e}: по формулам={analytical:?}, численно={numerical:?}"
            );
            for coord in 0..2 {
                assert!(check_f64_eq_1e_minus_6(numerical[coord], analytical[coord]));
            }
        }
    }
    // Для квадратичной функции центральная разность точна до округления.
    // Погрешность приближения большого шага показана на кубе в уроке 018.
    // При слишком малом шаге изменение входа теряется в f64.
    let x = 4.0;
    let tiny_step = 1e-20;
    let estimate = estimate_derivative_from_two_nearby_function_values(
        |new_x| loss(new_x, -2.0),
        x,
        tiny_step,
    );
    assert_eq!(estimate, 0.0);
    println!(
        "Шаг {tiny_step:e}: получили {estimate} вместо производной 4 — округление скрыло изменение."
    );

    // Дополнение: производная первообразной по x возвращает исходную функцию.
    let antiderivative = |x: f64, y: f64| (x - 2.0).powi(3) / 3.0 + 3.0 * (y + 1.0).powi(2) * x;
    let (x, y) = (0.3, 2.0);
    let restored = estimate_derivative_from_two_nearby_function_values(
        |new_x| antiderivative(new_x, y),
        x,
        1e-4,
    );
    assert!(check_f64_eq_1e_minus_7(restored, loss(x, y)));
    println!(
        "Производная первообразной={restored}; исходная функция={}",
        loss(x, y)
    );
}

// Чему учит этот урок:
// Проверяем обе частные производные в трёх точках, включая минимум, и при разных шагах.
// Общая функция из урока 018 принимает замыкание с одним изменяемым входом.
// Сравниваем численный градиент с аналитическим и видим отказ слишком малого шага.
// Дополнительно проверяем связь первообразной и исходной функции.
