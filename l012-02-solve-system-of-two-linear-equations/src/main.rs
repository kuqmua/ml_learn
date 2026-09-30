// Урок 02.5. Поиск двух неизвестных, удовлетворяющих двум линейным уравнениям.
// Связь с принятой терминологией: Решение системы двух линейных уравнений.
// Зачем здесь эта тема: Линейная система ищет вход, который после преобразования матрицей даёт
//   заданный выход.
// Почему код устроен так: Две переменные позволяют увидеть решение и случай нулевого определителя
//   без скрытого численного алгоритма.
// Представь: Если после умножения матрицы на неизвестный [x, y] получен известный ответ, система
//   ищет эти x и y.
//
// При ненулевом определителе решение одно. При нулевом определителе уравнения
// могут совпадать (решений бесконечно много) или противоречить друг другу (решений нет).

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 4], [f64; 2]); 5] = [
        ("одно решение", [2.0, 1.0, 1.0, -1.0], [5.0, 1.0]),
        ("бесконечно много решений", [1.0, 1.0, 2.0, 2.0], [3.0, 6.0]),
        ("решений нет", [1.0, 1.0, 2.0, 2.0], [3.0, 7.0]),
        ("бесконечно много решений", [0.0, 0.0, 0.0, 0.0], [0.0, 0.0]),
        ("решений нет", [0.0, 0.0, 0.0, 0.0], [1.0, 0.0]),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
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
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first_row_first_coefficient);
        lesson_trace::trace_step!(first_row_second_coefficient);
        lesson_trace::trace_step!(second_row_first_coefficient);
        lesson_trace::trace_step!(second_row_second_coefficient);
        lesson_trace::trace_step!(first_right_hand_side);
        lesson_trace::trace_step!(second_right_hand_side);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `determinant`.");
        let determinant: f64 = first_row_first_coefficient * second_row_second_coefficient
            - first_row_second_coefficient * second_row_first_coefficient;
        lesson_trace::trace_step!(determinant);
        lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
        if determinant != 0.0 {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `first_unknown`.");
            let first_unknown: f64 = (first_right_hand_side * second_row_second_coefficient
                - first_row_second_coefficient * second_right_hand_side)
                / determinant;
            lesson_trace::trace_step!(first_unknown);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `second_unknown`.");
            let second_unknown: f64 = (first_row_first_coefficient * second_right_hand_side
                - first_right_hand_side * second_row_first_coefficient)
                / determinant;
            lesson_trace::trace_step!(second_unknown);
            lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
            println!("{description}: x={first_unknown}, y={second_unknown}");
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Если замена столбца правой частью тоже даёт ноль, обе строки описывают одну прямую."
            );
            let first_replaced: f64 = first_right_hand_side * second_row_second_coefficient
                - first_row_second_coefficient * second_right_hand_side;
            lesson_trace::trace_step!(first_replaced);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `second_replaced`.");
            let second_replaced: f64 = first_row_first_coefficient * second_right_hand_side
                - first_right_hand_side * second_row_first_coefficient;
            lesson_trace::trace_step!(second_replaced);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `impossible_zero_row`.");
            lesson_trace::trace_note!("Задаём преобразование для элементов коллекции.");
            let impossible_zero_row: bool = (first_row_first_coefficient == 0.0
                && first_row_second_coefficient == 0.0
                && first_right_hand_side != 0.0)
                || (second_row_first_coefficient == 0.0
                    && second_row_second_coefficient == 0.0
                    && second_right_hand_side != 0.0);
            lesson_trace::trace_step!(impossible_zero_row);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `actual`.");
            let actual: &str =
                if first_replaced == 0.0 && second_replaced == 0.0 && !impossible_zero_row {
                    lesson_trace::trace_note!(
                        "Передаём подпись или текстовое значение для следующего шага."
                    );
                    "бесконечно много решений"
                } else {
                    lesson_trace::trace_note!(
                        "Обрабатываем случай, когда предыдущее условие не выполнено."
                    );
                    lesson_trace::trace_note!(
                        "Передаём подпись или текстовое значение для следующего шага."
                    );
                    "решений нет"
                };
            lesson_trace::trace_step!(actual);
            lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
            assert_eq!(actual, description);
            lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
            println!("{description}: определитель равен нулю");
        }
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_two_lines_and_their_intersection();
}

// Строим график по результатам урока.
fn plot_two_lines_and_their_intersection() {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let first_equation_points: Vec<(f64, f64)> = (0..=50)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (horizontal_value, 5.0 - 2.0 * horizontal_value)
        })
        .collect();
    lesson_trace::trace_note!("Собираем значения для `second_equation_points` в коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let second_equation_points: Vec<(f64, f64)> = (0..=50)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (horizontal_value, horizontal_value - 1.0)
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Две прямые с единственным пересечением",
        "x",
        "y",
        &[
            lesson_visualization::Series {
                name: "2x+y=5",

                points: &first_equation_points,
            },
            lesson_visualization::Series {
                name: "x−y=1",

                points: &second_equation_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
