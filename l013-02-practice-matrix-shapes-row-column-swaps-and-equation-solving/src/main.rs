// Урок 02.6. Практика: размеры матриц, перестановка строк в столбцы, умножение и решение уравнений.
// Связь с принятой терминологией: Форма матрицы, транспонирование, произведения и система уравнений.
// Зачем здесь эта тема: Формы, транспонирование и произведения должны работать вместе, иначе ошибка
//   размера проявится уже в системе.
// Почему код устроен так: Собираем малую матрицу и проверяем каждый переход по размерности и
//   значению.
// Представь: Прежде чем умножать матрицы, можно на бумаге определить форму результата и заметить
//   несовместимые размеры.
//
// Что повторяем вместе: формы матриц, транспонирование, матричное умножение, системы уравнений.
// Зачем это нужно: Матрицы описывают преобразования сразу нескольких признаков и служат основой линейных
//   слоёв моделей.
// Что показывает программа: Создаём матрицу 2×2 с известными элементами. Умножаем матрицу на вектор: каждая
//   координата ответа — сумма после попарного умножения элементов строки. Транспонируем матрицу, меняя строки и столбцы местами.
// Что проверить при изменении примера: Проверь единичную матрицу, несовместимые формы и несколько
//   результатов умножения, вычисленных вручную.
// Дополнительная практика: Сделай Matrix с проверкой размерностей, transpose, matmul и matvec без внешних
//   библиотек.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Автоматически получаем стандартные реализации перечисленных трейтов для этого типа."
    );
    lesson_trace::trace_note!(
        "Описываем тип `Matrix`, чтобы явно хранить состояние и допустимые варианты."
    );
    lesson_trace::trace_note!("`rows` задаёт соответствующее входное значение или поле структуры.");
    lesson_trace::trace_note!(
        "`column_count` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("`data` задаёт соответствующее входное значение или поле структуры.");
    #[derive(Debug, PartialEq)]
    struct Matrix {
        rows: usize,

        column_count: usize,

        data: Vec<f64>,
    }

    lesson_trace::trace_note!("Группируем методы рядом с типом, к которому они относятся.");
    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `create_matrix_from_elements_listed_row_by_row`; параметры ниже задают его входы."
    );
    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `value_at_row_and_column`; параметры ниже задают его входы."
    );
    impl Matrix {
        /// Создаём матрицу из элементов, перечисленных строка за строкой (row-major order).
        fn create_matrix_from_elements_listed_row_by_row(
            rows: usize,

            column_count: usize,

            data: Vec<f64>,
        ) -> Result<Self, &'static str> {
            lesson_trace::trace_note!(
                "`rows` задаёт соответствующее входное значение или поле структуры."
            );
            lesson_trace::trace_note!(
                "`column_count` задаёт соответствующее входное значение или поле структуры."
            );
            lesson_trace::trace_note!(
                "`data` задаёт соответствующее входное значение или поле структуры."
            );
            lesson_trace::trace_note!("Указываем тип возвращаемого значения.");
            lesson_trace::trace_note!(
                "Число элементов обязано совпадать с заявленной формой матрицы."
            );
            if rows * column_count != data.len() {
                lesson_trace::trace_note!(
                    "Прерываем расчёт и явно сообщаем причину некорректного входа."
                );
                return Err("неверная форма");
            }
            lesson_trace::trace_note!("Возвращаем успешное значение в типе `Result`.");
            Ok(Self {
                rows,
                column_count,
                data,
            })
        }

        fn value_at_row_and_column(&self, row_index: usize, column_index: usize) -> f64 {
            lesson_trace::trace_note!("Обновляем состояние объекта результатом текущей операции.");
            self.data[row_index * self.column_count + column_index]
        }
    }

    lesson_trace::trace_note!("Шаг: Создаём матрицу 2×2 с известными элементами.");
    let left_matrix: Matrix =
        Matrix::create_matrix_from_elements_listed_row_by_row(2, 2, vec![1., 2., 3., 4.]).unwrap();
    lesson_trace::trace_step!(left_matrix);
    lesson_trace::trace_note!(
        "Шаг: Умножаем матрицу на вектор: каждая координата ответа — сумма после попарного умножения элементов строки."
    );
    let input_vector: [f64; 2] = [1., 1.];
    lesson_trace::trace_step!(input_vector);
    lesson_trace::trace_note!(
        "Проверяем, что сравниваемые размерности или значения действительно совпадают."
    );
    assert_eq!(
        left_matrix.column_count,
        input_vector.len(),
        "несовместимые формы"
    );
    lesson_trace::trace_note!(
        "Создаём изменяемое значение `output_vector` для следующих операций."
    );
    let mut output_vector: Vec<f64> = Vec::with_capacity(left_matrix.rows);
    lesson_trace::trace_step!(output_vector);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for row_index in 0..left_matrix.rows {
        lesson_trace::trace_step!(row_index);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `row_start`.");
        let row_start: usize = row_index * left_matrix.column_count;
        lesson_trace::trace_step!(row_start);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `row_end`.");
        let row_end: usize = row_start + left_matrix.column_count;
        lesson_trace::trace_step!(row_end);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `row_result`.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
        let row_result: f64 = l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(

            &left_matrix.data[row_start..row_end],

            &input_vector,
        )

        .expect("длина строки совпадает с длиной вектора");
        lesson_trace::trace_step!(row_result);
        lesson_trace::trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
        output_vector.push(row_result);
    }
    lesson_trace::trace_note!("Шаг: Транспонируем матрицу, меняя строки и столбцы местами.");
    let mut transposed_elements: Vec<f64> = Vec::with_capacity(left_matrix.data.len());
    lesson_trace::trace_step!(transposed_elements);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for column_index in 0..left_matrix.column_count {
        lesson_trace::trace_step!(column_index);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for row_index in 0..left_matrix.rows {
            lesson_trace::trace_step!(row_index);
            lesson_trace::trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
            transposed_elements.push(left_matrix.value_at_row_and_column(row_index, column_index));
        }
    }
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `transposed_matrix` для следующих операций."
    );
    lesson_trace::trace_note!("`rows` задаёт соответствующее входное значение или поле структуры.");
    lesson_trace::trace_note!(
        "`column_count` задаёт соответствующее входное значение или поле структуры."
    );
    lesson_trace::trace_note!("`data` задаёт соответствующее входное значение или поле структуры.");
    let transposed_matrix: Matrix = Matrix {
        rows: left_matrix.column_count,

        column_count: left_matrix.rows,

        data: transposed_elements,
    };
    lesson_trace::trace_step!(transposed_matrix);
    lesson_trace::trace_note!(
        "Шаг: Считаем каждый элемент результата умножения как сумму после попарного умножения координат строки и столбца."
    );
    lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
    lesson_trace::trace_note!("Передаём число строк результата как первую размерность матрицы.");
    lesson_trace::trace_note!(
        "Подставляем результаты в этот шаблон вывода или текстового значения."
    );
    assert_eq!(
        transposed_matrix.column_count, left_matrix.rows,
        "несовместимые формы"
    );
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `result_elements`.");
    let mut result_elements: Vec<f64> =
        Vec::with_capacity(transposed_matrix.rows * left_matrix.column_count);
    lesson_trace::trace_step!(result_elements);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for row_index in 0..transposed_matrix.rows {
        lesson_trace::trace_step!(row_index);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for column_index in 0..left_matrix.column_count {
            lesson_trace::trace_step!(column_index);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `row_start`.");
            let row_start: usize = row_index * transposed_matrix.column_count;
            lesson_trace::trace_step!(row_start);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `row_end`.");
            let row_end: usize = row_start + transposed_matrix.column_count;
            lesson_trace::trace_step!(row_end);
            lesson_trace::trace_note!("Собираем значения для `column_values` в коллекцию.");
            lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
            lesson_trace::trace_note!("Собираем результаты в коллекцию.");
            let column_values: Vec<f64> = (0..left_matrix.rows)
                .map(|shared_index| left_matrix.value_at_row_and_column(shared_index, column_index))
                .collect();
            lesson_trace::trace_step!(column_values);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `cell_value`.");
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
            let cell_value: f64 = l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(

                &transposed_matrix.data[row_start..row_end],

                &column_values,
            )

            .expect("внутренние размеры матриц совпадают");
            lesson_trace::trace_step!(cell_value);
            lesson_trace::trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
            result_elements.push(cell_value);
        }
    }
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `result_matrix` для следующих операций."
    );
    lesson_trace::trace_note!(
        "Собираем матрицу после умножения с рассчитанными размерностями и элементами."
    );
    lesson_trace::trace_note!("Извлекаем значение: выше в примере обеспечено отсутствие ошибки.");
    let result_matrix: Matrix = Matrix::create_matrix_from_elements_listed_row_by_row(
        transposed_matrix.rows,
        left_matrix.column_count,
        result_elements,
    )
    .unwrap();
    lesson_trace::trace_step!(result_matrix);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("A*x={:?}, A^T*A={:?}", output_vector, result_matrix);

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_product_of_transposed_matrix_and_original_matrix(result_matrix);

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_product_of_transposed_matrix_and_original_matrix(result_matrix: Matrix) {
        lesson_trace::trace_note!("Значения ячеек видны по цвету и подписи.");
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок тепловой карты.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::heatmap(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Матрица AᵀA",
            &result_matrix
                .data
                .chunks(result_matrix.column_count)
                .map(|row| row.to_vec())
                .collect::<Vec<_>>(),
        )
        .expect("не удалось сохранить тепловую карту");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
