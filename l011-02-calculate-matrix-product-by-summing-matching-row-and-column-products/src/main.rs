// Урок 02.4. Произведение двух матриц: сложение произведений соответствующих элементов строк и столбцов.
// Связь с принятой терминологией: Умножение двух матриц через скалярные произведения строк и столбцов.
// Зачем здесь эта тема: После произведения матрицы на вектор произведение матриц повторяет ту же
//   идею для каждого столбца второго множителя.
// Почему код устроен так: Для элемента результата берём строку слева и столбец справа; их длины
//   должны совпадать.
// Представь: Одно число в произведении матриц получается из строки первой и столбца второй матрицы.
//
// Каждая ячейка ответа получается попарным умножением строки и столбца со сложением.
// Единичная матрица не меняет значения; порядок множителей обычно влияет на ответ.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `left`.");
    let left: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    lesson_trace::trace_step!(left);
    lesson_trace::trace_note!("Задаём учебные значения для `identity`.");
    let identity: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 1.0]];
    lesson_trace::trace_step!(identity);
    lesson_trace::trace_note!("Задаём учебные значения для `right`.");
    let right: [[f64; 2]; 2] = [[5.0, 6.0], [7.0, 8.0]];
    lesson_trace::trace_step!(right);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Задаём значения следующей строки или последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, first, second, expected) in [
        ("обычный порядок", left, right, [[19.0, 22.0], [43.0, 50.0]]),
        (
            "обратный порядок",
            right,
            left,
            [[23.0, 34.0], [31.0, 46.0]],
        ),
        ("единичная справа", left, identity, left),
        ("единичная слева", identity, left, left),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first);
        lesson_trace::trace_step!(second);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert_eq!(
            first[0].len(),
            second.len(),
            "внутренние размеры матриц должны совпадать"
        );
        lesson_trace::trace_note!("Задаём учебные значения для `result`.");
        let mut result: [[f64; 2]; 2] = [[0.0; 2]; 2];
        lesson_trace::trace_step!(result);
        lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for row in 0..first.len() {
            lesson_trace::trace_step!(row);
            lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
            for column in 0..second[0].len() {
                lesson_trace::trace_step!(column);
                lesson_trace::trace_note!("Задаём учебные значения для `column_values`.");
                let column_values: [f64; 2] = [second[0][column], second[1][column]];
                lesson_trace::trace_step!(column_values);
                lesson_trace::trace_note!(
                    "Строка × столбец — то же попарное умножение и сложение из урока 01.1."
                );
                lesson_trace::trace_note!(
                    "Используем подготовленное значение в следующем шаге примера."
                );
                lesson_trace::trace_note!(
                    "Используем результат, ожидая успешного выполнения шага."
                );
                result[row][column] =

                    l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&first[row], &column_values)

                        .expect("внутренние размеры матриц совпадают");
                lesson_trace::trace_step!(result);
            }
        }
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(result, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {result:?}");
    }
    lesson_trace::trace_note!("Сохраняем результат этого шага в `incompatible_left_shape`.");
    let incompatible_left_shape: (i32, i32) = (2, 3);
    lesson_trace::trace_step!(incompatible_left_shape);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `incompatible_right_shape`.");
    let incompatible_right_shape: (i32, i32) = (2, 2);
    lesson_trace::trace_step!(incompatible_right_shape);
    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
    if incompatible_left_shape.1 != incompatible_right_shape.0 {
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        println!(
            "размеры {incompatible_left_shape:?} и {incompatible_right_shape:?}: умножение невозможно"
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_matrix_product_as_sums_of_matching_row_and_column_products();
}

// Строим график по результатам урока.
fn plot_matrix_product_as_sums_of_matching_row_and_column_products() {
    lesson_trace::trace_note!("Значения ячеек видны по цвету и подписи.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок тепловой карты.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Произведение матриц A × B",
        &[vec![19.0, 22.0], vec![43.0, 50.0]],
    )
    .expect("не удалось сохранить тепловую карту");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
