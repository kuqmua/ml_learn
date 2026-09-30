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
    lesson_trace::enable();
    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        value * value
    }

    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three`; параметры ниже задают его входы."
    );
    /// Квадратичная функция: (x − 2)² + 3(y + 1)².
    fn calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(
        first_parameter: f64,
        second_parameter: f64,
    ) -> f64 {
        lesson_trace::trace_note!(
            "Складываем или вычитаем величины согласно используемой формуле."
        );
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        calculate_square_by_multiplying_number_by_itself(first_parameter - 2.0)
            + 3.0 * calculate_square_by_multiplying_number_by_itself(second_parameter + 1.0)
    }

    lesson_trace::trace_note!(
        "Интегрируем по первому параметру, считая второй постоянным; константа интегрирования равна нулю."
    );
    /// Первообразная по x: (x − 2)³ / 3 + 3(y + 1)²x; её производная по x равна исходной функции.
    fn calculate_antiderivative_by_cubing_first_shift_dividing_by_three_and_adding_second_shift_term(
        first_parameter: f64,

        second_parameter: f64,
    ) -> f64 {
        lesson_trace::trace_note!(
            "`first_parameter` задаёт соответствующее входное значение или поле структуры."
        );
        lesson_trace::trace_note!(
            "`second_parameter` задаёт соответствующее входное значение или поле структуры."
        );
        lesson_trace::trace_note!("Указываем тип возвращаемого значения.");
        lesson_trace::trace_note!(
            "Комбинируем исходные величины и сохраняем результат в `shifted_first`."
        );
        let shifted_first: f64 = first_parameter - 2.0;
        lesson_trace::trace_step!(shifted_first);
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        shifted_first * shifted_first * shifted_first / 3.0
            + 3.0
                * calculate_square_by_multiplying_number_by_itself(second_parameter + 1.0)
                * first_parameter
    }

    lesson_trace::trace_note!(
        "Сравниваем h=10⁻², 10⁻⁴ и 10⁻⁸: большой h даёт ошибку приближения, слишком малый усиливает округление f64."
    );
    for step_size in [1e-2, 1e-4, 1e-8] {
        lesson_trace::trace_step!(step_size);
        lesson_trace::trace_note!(
            "Шаг: Сравниваем численный градиент с производными, найденными вручную."
        );
        lesson_trace::trace_note!(
            "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
        );
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Вычисляем обе частные производные квадратичной функции.");
        lesson_trace::trace_note!("Сохраняем результат этого шага в `first_parameter`.");
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `second_parameter` для следующих операций."
        );
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!(
            "Центральная конечная разность приближает каждую частную производную."
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `first_parameter`.");
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `second_parameter` для следующих операций."
        );
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `step_size` для следующих операций."
        );
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!(
            "Складываем или вычитаем величины согласно используемой формуле."
        );
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `second_parameter` в текущем выражении."
        );
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        lesson_trace::trace_note!(
            "Складываем или вычитаем величины согласно используемой формуле."
        );
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `second_parameter` в текущем выражении."
        );
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `first_parameter` в текущем выражении."
        );
        lesson_trace::trace_note!(
            "Складываем или вычитаем величины согласно используемой формуле."
        );
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `first_parameter` в текущем выражении."
        );
        lesson_trace::trace_note!(
            "Складываем или вычитаем величины согласно используемой формуле."
        );
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        println!(
            "h={step_size:e}: analytic={:?}, numeric={:?}",
            (|| -> [f64; 2] {
                let first_parameter: f64 = 0.3;
                lesson_trace::trace_step!(first_parameter);
                lesson_trace::trace_step!(first_parameter);

                let second_parameter: f64 = 2.0;
                lesson_trace::trace_step!(second_parameter);
                lesson_trace::trace_step!(second_parameter);

                [
                    2.0 * (first_parameter - 2.0),
                    6.0 * (second_parameter + 1.0),
                ]
            })(),
            (|| -> [f64; 2] {
                let first_parameter: f64 = 0.3;
                lesson_trace::trace_step!(first_parameter);
                lesson_trace::trace_step!(first_parameter);

                let second_parameter: f64 = 2.0;
                lesson_trace::trace_step!(second_parameter);
                lesson_trace::trace_step!(second_parameter);

                let step_size: f64 = step_size;
                lesson_trace::trace_step!(step_size);
                lesson_trace::trace_step!(step_size);

                [

                    (calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter + step_size,

                        second_parameter,

                    ) - calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter - step_size,

                        second_parameter,

                    )) / (2.0 * step_size),

                    (calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter,

                        second_parameter + step_size,

                    ) - calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(

                        first_parameter,

                        second_parameter - step_size,

                    )) / (2.0 * step_size),
                ]
            })()
        );
    }
    lesson_trace::trace_note!(
        "Производная первообразной по первому параметру должна возвращать исходную функцию."
    );
    lesson_trace::trace_note!(
        "h=10⁻⁵ — отдельный небольшой шаг для проверки, что производная первообразной возвращает функцию."
    );
    let step_size: f64 = 1e-5;
    lesson_trace::trace_step!(step_size);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `recovered_value` для следующих операций."
    );
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    let recovered_value: f64 =

        (calculate_antiderivative_by_cubing_first_shift_dividing_by_three_and_adding_second_shift_term(0.3 + step_size, 2.0)

            - calculate_antiderivative_by_cubing_first_shift_dividing_by_three_and_adding_second_shift_term(0.3 - step_size, 2.0))

            / (2.0 * step_size);
    lesson_trace::trace_step!(recovered_value);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    println!(
        "f(0.3, 2)={}, производная первообразной={recovered_value}",
        calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(
            0.3, 2.0
        )
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_coordinate_slope_estimation_error_for_shrinking_step();

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_coordinate_slope_estimation_error_for_shrinking_step() {
        lesson_trace::trace_note!("Собираем значения для `points` в коллекцию.");
        lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
        lesson_trace::trace_note!("Собираем результаты в коллекцию.");
        let points: Vec<(f64, f64)> = (1..=12)

            .map(|step_exponent| {
                lesson_trace::trace_note!("Сохраняем результат этого шага в `step_size`.");
                let step_size: f64 = 10f64.powi(-step_exponent);
                lesson_trace::trace_note!("Сохраняем результат этого шага в `numeric`.");
                lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
                lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
                let numeric: f64 = (calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(0.3 + step_size, 2.0)

                    - calculate_quadratic_function_value_as_sum_of_squared_shifts_with_second_weighted_by_three(0.3 - step_size, 2.0))

                    / (2.0 * step_size);
                lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
                (step_exponent as f64, (numeric - 2.0 * (0.3 - 2.0)).abs())
            })

            .collect();
        lesson_trace::trace_note!(
            "Строим график по рассчитанным значениям и сохраняем его как SVG."
        );
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок графика.");
        lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
        lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
        lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
        lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
        lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Ошибка численного градиента",
            "k для h=10⁻ᵏ",
            "абсолютная ошибка",
            &[lesson_visualization::Series {
                name: "∂f/∂x",

                points: &points,
            }],
        )
        .expect("не удалось сохранить график");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
