// Урок 03.6. Практика: скорости изменения функции и их проверка по соседним значениям.
// Связь с принятой терминологией: Производные, правило цепочки, градиент и конечные разности.
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
        first_parameter: f64,
        second_parameter: f64,
    ) -> f64 {
        calc_square_by_multiplying_number_by_itself(first_parameter - 2.0)
            + 3.0 * calc_square_by_multiplying_number_by_itself(second_parameter + 1.0)
    }

    /// Первообразная по x: (x − 2)³ / 3 + 3(y + 1)²x; её производная по x равна исходной функции.
    fn calc_antiderivative_by_cubing_first_shift_dividing_by_three_and_adding_second_shift_term(
        first_parameter: f64,

        second_parameter: f64,
    ) -> f64 {
        let shifted_first: f64 = first_parameter - 2.0;
        shifted_first * shifted_first * shifted_first / 3.0
            + 3.0
                * calc_square_by_multiplying_number_by_itself(second_parameter + 1.0)
                * first_parameter
    }

    for step_size in [1e-2, 1e-4, 1e-8] {
        let _ = (
            &((|| -> [f64; 2] {
                let first_parameter: f64 = 0.3;

                let second_parameter: f64 = 2.0;

                [
                    2.0 * (first_parameter - 2.0),
                    6.0 * (second_parameter + 1.0),
                ]
            })()),
            &((|| -> [f64; 2] {
                let step_size: f64 = step_size;

                let first_parameter: f64 = 0.3;
                let second_parameter: f64 = 2.0;
                [

                    (calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter + step_size,

                        second_parameter,

                    ) - calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter - step_size,

                        second_parameter,

                    )) / (2.0 * step_size),

                    (calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter,

                        second_parameter + step_size,

                    ) - calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter,

                        second_parameter - step_size,

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

    plot_coord_slope_estimation_error_for_shrinking_step();

    fn plot_coord_slope_estimation_error_for_shrinking_step() {
        lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Ошибка численного градиента",
            "k для h=10⁻ᵏ",
            "абсолютная ошибка",
            &[lesson_visualization::Series {
                name: "∂f/∂x",

                points: &(1..=12)

            .map(|step_exponent| {
                let step_size: f64 = 10f64.powi(-step_exponent);
                let numeric: f64 = (calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(0.3 + step_size, 2.0)

                    - calc_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(0.3 - step_size, 2.0))

                    / (2.0 * step_size);
                (step_exponent as f64, (numeric - 2.0 * (0.3 - 2.0)).abs())
            })

            .collect::<Vec<_>>(),
            }],
        )
        .expect("не удалось сохранить график");
    }
}
