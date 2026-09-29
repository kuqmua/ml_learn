// Сводная практика 02. Форма матрицы, транспонирование, произведения и система уравнений.
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
    // Автоматически получаем стандартные реализации перечисленных трейтов для этого типа.
    #[derive(Debug, PartialEq)]
    // Описываем тип `Matrix`, чтобы явно хранить состояние и допустимые варианты.
    struct Matrix {
        // `rows` задаёт соответствующее входное значение или поле структуры.
        rows: usize,
        // `column_count` задаёт соответствующее входное значение или поле структуры.
        column_count: usize,
        // `data` задаёт соответствующее входное значение или поле структуры.
        data: Vec<f64>,
    }

    // Группируем методы рядом с типом, к которому они относятся.
    impl Matrix {
        // Объявляем повторно используемое вычисление `from_row_major_elements`; параметры ниже задают его входы.
        fn from_row_major_elements(
            // `rows` задаёт соответствующее входное значение или поле структуры.
            rows: usize,
            // `column_count` задаёт соответствующее входное значение или поле структуры.
            column_count: usize,
            // `data` задаёт соответствующее входное значение или поле структуры.
            data: Vec<f64>,
            // Указываем тип возвращаемого значения.
        ) -> Result<Self, &'static str> {
            // Число элементов обязано совпадать с заявленной формой матрицы.
            if rows * column_count != data.len() {
                // Прерываем расчёт и явно сообщаем причину некорректного входа.
                return Err("неверная форма");
            }
            // Возвращаем успешное значение в типе `Result`.
            Ok(Self {
                rows,
                column_count,
                data,
            })
        }
        // Объявляем повторно используемое вычисление `value_at_row_and_column`; параметры ниже задают его входы.
        fn value_at_row_and_column(&self, row_index: usize, column_index: usize) -> f64 {
            // Обновляем состояние объекта результатом текущей операции.
            self.data[row_index * self.column_count + column_index]
        }
    }

    // Шаг: Создаём матрицу 2×2 с известными элементами.
    let left_matrix = Matrix::from_row_major_elements(2, 2, vec![1., 2., 3., 4.]).unwrap();
    // Шаг: Умножаем матрицу на вектор: каждая координата ответа — сумма после попарного умножения элементов строки.
    let input_vector = [1., 1.];
    // Проверяем, что сравниваемые размерности или значения действительно совпадают.
    assert_eq!(
        left_matrix.column_count,
        input_vector.len(),
        "несовместимые формы"
    );
    // Создаём изменяемое значение `output_vector` для следующих операций.
    let mut output_vector = Vec::with_capacity(left_matrix.rows);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for row_index in 0..left_matrix.rows {
        // Сохраняем результат этого шага в `row_start`.
        let row_start = row_index * left_matrix.column_count;
        // Сохраняем результат этого шага в `row_end`.
        let row_end = row_start + left_matrix.column_count;
        // Сохраняем результат этого шага в `row_result`.
        let row_result = part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::multiply_matching_coordinates_of_two_vectors_then_add(
            // Используем подготовленное значение в следующем шаге примера.
            &left_matrix.data[row_start..row_end],
            // Используем подготовленное значение в следующем шаге примера.
            &input_vector,
        )
        // Используем результат, ожидая успешного выполнения шага.
        .expect("длина строки совпадает с длиной вектора");
        // Сохраняем очередной рассчитанный элемент в коллекции.
        output_vector.push(row_result);
    }
    // Шаг: Транспонируем матрицу, меняя строки и столбцы местами.
    let mut transposed_elements = Vec::with_capacity(left_matrix.data.len());
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for column_index in 0..left_matrix.column_count {
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for row_index in 0..left_matrix.rows {
            // Сохраняем очередной рассчитанный элемент в коллекции.
            transposed_elements.push(left_matrix.value_at_row_and_column(row_index, column_index));
        }
    }
    // Сохраняем рассчитанное значение `transposed_matrix` для следующих операций.
    let transposed_matrix = Matrix {
        // `rows` задаёт соответствующее входное значение или поле структуры.
        rows: left_matrix.column_count,
        // `column_count` задаёт соответствующее входное значение или поле структуры.
        column_count: left_matrix.rows,
        // `data` задаёт соответствующее входное значение или поле структуры.
        data: transposed_elements,
    };
    // Шаг: Считаем каждый элемент результата умножения как сумму после попарного умножения координат строки и столбца.
    assert_eq!(
        // Передаём очередное значение в составе результата или вызова.
        transposed_matrix.column_count,
        // Передаём число строк результата как первую размерность матрицы.
        left_matrix.rows,
        // Подставляем результаты в этот шаблон вывода или текстового значения.
        "несовместимые формы"
    );
    // Умножаем значения и сохраняем результат в `result_elements`.
    let mut result_elements = Vec::with_capacity(transposed_matrix.rows * left_matrix.column_count);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for row_index in 0..transposed_matrix.rows {
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for column_index in 0..left_matrix.column_count {
            // Сохраняем результат этого шага в `row_start`.
            let row_start = row_index * transposed_matrix.column_count;
            // Сохраняем результат этого шага в `row_end`.
            let row_end = row_start + transposed_matrix.column_count;
            // Собираем значения для `column_values` в коллекцию.
            let column_values: Vec<_> = (0..left_matrix.rows)
                // Преобразуем каждый элемент в новое значение.
                .map(|shared_index| left_matrix.value_at_row_and_column(shared_index, column_index))
                // Собираем результаты в коллекцию.
                .collect();
            // Сохраняем результат этого шага в `cell_value`.
            let cell_value = part_001_lesson_01_multiply_matching_coordinates_of_two_vectors_then_add::multiply_matching_coordinates_of_two_vectors_then_add(
                // Используем подготовленное значение в следующем шаге примера.
                &transposed_matrix.data[row_start..row_end],
                // Используем подготовленное значение в следующем шаге примера.
                &column_values,
            )
            // Используем результат, ожидая успешного выполнения шага.
            .expect("внутренние размеры матриц совпадают");
            // Сохраняем очередной рассчитанный элемент в коллекции.
            result_elements.push(cell_value);
        }
    }
    // Сохраняем рассчитанное значение `result_matrix` для следующих операций.
    let result_matrix =
        // Собираем матрицу после умножения с рассчитанными размерностями и элементами.
        Matrix::from_row_major_elements(transposed_matrix.rows, left_matrix.column_count, result_elements)
            // Извлекаем значение: выше в примере обеспечено отсутствие ошибки.
            .unwrap();
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("A*x={:?}, A^T*A={:?}", output_vector, result_matrix);

    // Построение графика вынесено из основного кода урока.
    visualize_practice_matrix_shapes_transpose_products_and_linear_system(result_matrix);

    // Строим график по результатам урока.
    fn visualize_practice_matrix_shapes_transpose_products_and_linear_system(
        result_matrix: Matrix,
    ) {
        // Значения ячеек видны по цвету и подписи.
        let chart = lesson_visualization::heatmap(
            // Передаём путь к каталогу текущего урока.
            env!("CARGO_MANIFEST_DIR"),
            // Указываем имя SVG-файла.
            "lesson-chart",
            // Указываем заголовок тепловой карты.
            "Матрица AᵀA",
            // Используем подготовленное значение в следующем шаге примера.
            &result_matrix
                // Настраиваем или преобразуем результат предыдущего шага.
                .data
                // Настраиваем или преобразуем результат предыдущего шага.
                .chunks(result_matrix.column_count)
                // Преобразуем каждый элемент в новое значение.
                .map(|row| row.to_vec())
                // Настраиваем или преобразуем результат предыдущего шага.
                .collect::<Vec<_>>(),
        )
        // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
        .expect("не удалось сохранить тепловую карту");
        // Печатаем путь к созданному SVG, чтобы его можно было открыть.
        println!("график: {}", chart.display());
    }
}
