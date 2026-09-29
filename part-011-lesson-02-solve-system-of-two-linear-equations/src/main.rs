// Урок 02.5. Решение системы двух линейных уравнений.
//
// При ненулевом определителе решение одно. При нулевом определителе уравнения
// могут совпадать (решений бесконечно много) или противоречить друг другу (решений нет).

fn main() {
    // Задаём учебные значения для `cases`.
    let cases = [
        // Добавляем пару значений для сравнения или построения графика.
        ("одно решение", [2.0, 1.0, 1.0, -1.0], [5.0, 1.0]),
        // Добавляем пару значений для сравнения или построения графика.
        ("бесконечно много решений", [1.0, 1.0, 2.0, 2.0], [3.0, 6.0]),
        // Добавляем пару значений для сравнения или построения графика.
        ("решений нет", [1.0, 1.0, 2.0, 2.0], [3.0, 7.0]),
        // Добавляем пару значений для сравнения или построения графика.
        ("бесконечно много решений", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0]),
        // Добавляем пару значений для сравнения или построения графика.
        ("решений нет", [0.0, 0.0, 0.0, 0.0], [1.0, 0.0]),
    ];
    // Повторяем расчёт для каждого элемента последовательности.
    for (
        description,
        [
            first_row_first_coefficient,
            first_row_second_coefficient,
            second_row_first_coefficient,
            second_row_second_coefficient,
        ],
        [first_right_hand_side, second_right_hand_side],
    ) in cases
    {
        // Сохраняем результат этого шага в `determinant`.
        let determinant = first_row_first_coefficient * second_row_second_coefficient
            - first_row_second_coefficient * second_row_first_coefficient;
        // Выбираем дальнейший шаг по выполнению условия.
        if determinant != 0.0 {
            // Сохраняем результат этого шага в `first_unknown`.
            let first_unknown = (first_right_hand_side * second_row_second_coefficient
                - first_row_second_coefficient * second_right_hand_side)
                / determinant;
            // Сохраняем результат этого шага в `second_unknown`.
            let second_unknown = (first_row_first_coefficient * second_right_hand_side
                - first_right_hand_side * second_row_first_coefficient)
                / determinant;
            // Печатаем рассчитанные значения для проверки примера.
            println!("{description}: x={first_unknown}, y={second_unknown}");
        // Обрабатываем случай, когда предыдущее условие не выполнено.
        } else {
            // Если замена столбца правой частью тоже даёт ноль, обе строки описывают одну прямую.
            let first_replaced = first_right_hand_side * second_row_second_coefficient
                - first_row_second_coefficient * second_right_hand_side;
            // Сохраняем результат этого шага в `second_replaced`.
            let second_replaced = first_row_first_coefficient * second_right_hand_side
                - first_right_hand_side * second_row_first_coefficient;
            // Сохраняем результат этого шага в `impossible_zero_row`.
            let impossible_zero_row = (first_row_first_coefficient == 0.0 && first_row_second_coefficient == 0.0 && first_right_hand_side != 0.0)
                // Задаём преобразование для элементов коллекции.
                || (second_row_first_coefficient == 0.0 && second_row_second_coefficient == 0.0 && second_right_hand_side != 0.0);
            // Сохраняем результат этого шага в `actual`.
            let actual = if first_replaced == 0.0 && second_replaced == 0.0 && !impossible_zero_row
            {
                // Передаём подпись или текстовое значение для следующего шага.
                "бесконечно много решений"
            // Обрабатываем случай, когда предыдущее условие не выполнено.
            } else {
                // Передаём подпись или текстовое значение для следующего шага.
                "решений нет"
            };
            // Проверяем ожидаемое свойство учебного примера.
            assert_eq!(actual, description);
            // Печатаем рассчитанные значения для проверки примера.
            println!("{description}: определитель равен нулю");
        }
    }

    // Построение графика вынесено из основного кода урока.
    visualize_solve_system_of_two_linear_equations();
}

// Строим график по результатам урока.
fn visualize_solve_system_of_two_linear_equations() {
    // Значения из этого урока на графике.
    let first_equation_points: Vec<(f64, f64)> = (0..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, 5.0 - 2.0 * horizontal_value)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `second_equation_points` в коллекцию.
    let second_equation_points: Vec<(f64, f64)> = (0..=50)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `horizontal_value`.
            let horizontal_value = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (horizontal_value, horizontal_value - 1.0)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Две прямые с единственным пересечением",
        // Указываем подпись горизонтальной оси.
        "x",
        // Указываем подпись вертикальной оси.
        "y",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "2x+y=5",
                // Передаём рассчитанные координаты точек.
                points: &first_equation_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "x−y=1",
                // Передаём рассчитанные координаты точек.
                points: &second_equation_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
