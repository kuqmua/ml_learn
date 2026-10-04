// Урок 019. Получать градиент по формулам и по соседним значениям функции при разных размерах шага.
// Дополнительно связываем первообразную с исходной функцией через численное дифференцирование.
// Это подготовка к проверке вычислений, от которых зависит обновление параметров модели.

use lesson_float_comparison::check_f64_eq_1e_minus_7;

fn main() {
    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calc_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        value * value
    }

    /// Квадратичная функция: (x − 2)² + 3(y + 1)².
    fn calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(
        parameter1: f64,
        parameter2: f64,
    ) -> f64 {
        calc_square_by_multiplying_number_by_itself(parameter1 - 2.0)
            + 3.0 * calc_square_by_multiplying_number_by_itself(parameter2 + 1.0)
    }

    /// Первообразная по x: (x − 2)³ / 3 + 3(y + 1)²x; её производная по x равна исходной функции.
    fn calc_antiderivative_by_cubing_first_shift_dividing_by_three_and_adding_second_shift_term(
        parameter1: f64,

        parameter2: f64,
    ) -> f64 {
        let shifted1: f64 = parameter1 - 2.0;
        shifted1 * shifted1 * shifted1 / 3.0
            + 3.0 * calc_square_by_multiplying_number_by_itself(parameter2 + 1.0) * parameter1
    }

    for step_size in [1e-2, 1e-4, 1e-8] {
        let _ = (
            &((|| -> [f64; 2] {
                let parameter1: f64 = 0.3;

                let parameter2: f64 = 2.0;

                [2.0 * (parameter1 - 2.0), 6.0 * (parameter2 + 1.0)]
            })()),
            &((|| -> [f64; 2] {
                let step_size: f64 = step_size;

                let parameter1: f64 = 0.3;
                let parameter2: f64 = 2.0;
                [

                    (calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        parameter1 + step_size,

                        parameter2,

                    ) - calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        parameter1 - step_size,

                        parameter2,

                    )) / (2.0 * step_size),

                    (calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        parameter1,

                        parameter2 + step_size,

                    ) - calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        parameter1,

                        parameter2 - step_size,

                    )) / (2.0 * step_size),
                ]
            })()),
        );
    }
    let step_size: f64 = 1e-5;
    let _: f64 =

        (calc_antiderivative_by_cubing_first_shift_dividing_by_three_and_adding_second_shift_term(0.3 + step_size, 2.0)

            - calc_antiderivative_by_cubing_first_shift_dividing_by_three_and_adding_second_shift_term(0.3 - step_size, 2.0))

            / (2.0 * step_size);
    let _ = &(calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(
        0.3, 2.0,
    ));

    let (x, y) = (0.3, 2.0);
    let h = 1e-4;
    let f = calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three;
    let analytical = [2.0 * (x - 2.0), 6.0 * (y + 1.0)];
    let numerical = [
        (f(x + h, y) - f(x - h, y)) / (2.0 * h),
        (f(x, y + h) - f(x, y - h)) / (2.0 * h),
    ];
    println!("Градиент по формулам={analytical:?}, по соседним значениям={numerical:?}");
    for i in 0..2 {
        assert!(check_f64_eq_1e_minus_7(analytical[i], numerical[i]));
    }
}

// Чему учит этот урок:
// Учимся получать градиент по формулам и по соседним значениям функции при разных размерах шага.
// Дополнительно связываем первообразную с исходной функцией через численное дифференцирование.
// Это подготовка к проверке вычислений, от которых зависит обновление параметров модели.
