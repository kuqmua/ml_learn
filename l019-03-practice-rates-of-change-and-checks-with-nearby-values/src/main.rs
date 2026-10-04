// Урок 03.6. Практика: скорости изменения функции и их проверка по соседним значениям.
// Зачем здесь эта тема: Перед оптимизацией нужно связать правило цепочки, градиент и численную
//   проверку.
// Почему код устроен так: Считаем один результат несколькими способами и сравниваем, где ошибка
//   формулы становится заметной.
// Представь: Один и тот же градиент можно получить из формулы и приближённо из двух вызовов
//   функции; ответы должны быть близки.
//
// Что повторяем вместе: производная, частная производная, правило цепочки, градиент, численная разность.
// Зачем это нужно: Градиент показывает направление изменения функции ошибки, а численная разность помогает
//   проверить ручную производную.
// Что показывает программа: Меняем шаг конечной разности: слишком большой и слишком малый шаг дают разные
//   погрешности. Сравниваем численный градиент с производными, найденными вручную.
// Что проверить при изменении примера: Сравни градиенты при разных eps; объясни ошибку округления и ошибку
//   аппроксимации.
// Дополнительная практика: Для f(x,y)=(x−2)²+3(y+1)² реализуй аналитический и численный градиенты.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

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
}

// Чему учит этот урок:
// Учимся получать градиент по формулам и по соседним значениям функции при разных размерах шага.
// Дополнительно связываем первообразную с исходной функцией через численное дифференцирование.
// Это подготовка к проверке вычислений, от которых зависит обновление параметров модели.
