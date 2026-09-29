// Сводная практика 22. Обратный режим дифференцирования и проверка градиента.
//
// Что повторяем вместе: reverse mode, тензорные формы, broadcasting, проверка градиента.
// Зачем это нужно: Автоматическое дифференцирование вычисляет градиенты составных операций, повторно
//   применяя локальные правила производных.
// Что показывает программа: Задаём две малые матрицы, результат умножения можно проверить вручную.
//   Вычисляем производные суммы элементов результата по обеим матрицам. Выводим loss и обе матрицы
//   градиентов.
// Что проверить при изменении примера: Проверь градиенты численно на случайных малых входах и ошибку
//   несовместимых форм.
// Дополнительная практика: Расширь скалярный граф или создай минимальный Tensor для elementwise и matmul.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Задаём две малые матрицы, результат умножения можно проверить вручную.
    let left_matrix: Vec<Vec<f64>> = vec![vec![1., 2.]];
    lesson_trace::trace_step!(left_matrix);
    // Создаём набор значений `right_matrix` для следующего шага примера.
    let right_matrix: Vec<Vec<f64>> = vec![vec![3.], vec![4.]];
    lesson_trace::trace_step!(right_matrix);

    // Шаг: Вычисляем производные суммы элементов результата по обеим матрицам.
    // Производную функции по параметру или вектор таких производных называют gradient.
    let (left_input_rates_of_change, right_input_rates_of_change): (Vec<Vec<f64>>, Vec<Vec<f64>>) =
        (|| -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
            // Используем подготовленное значение в следующем шаге примера.
            /* Для суммы элементов результата умножения матриц вычисляем производные по обоим входам. */
            // Собираем значения для `left_matrix` в коллекцию.
            let left_matrix: &[Vec<f64>] = &left_matrix;
            lesson_trace::trace_step!(left_matrix);
            // Сохраняем рассчитанное значение `right_matrix` для следующих операций.
            let right_matrix: &[Vec<f64>] = &right_matrix;
            lesson_trace::trace_step!(right_matrix);
            // Создаём набор значений `left_input_rates_of_change` для следующего шага примера.
            let mut left_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix.len()]; left_matrix.len()];
            lesson_trace::trace_step!(left_input_rates_of_change);
            // Создаём набор значений `right_input_rates_of_change` для следующего шага примера.
            let mut right_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix[0].len()]; right_matrix.len()];
            lesson_trace::trace_step!(right_input_rates_of_change);
            // Производная суммы элементов A·B по A[i,k] — сумма строки B[k,*].
            for row_index in 0..left_matrix.len() {
                lesson_trace::trace_step!(row_index);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for shared_index in 0..right_matrix.len() {
                    lesson_trace::trace_step!(shared_index);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for column_index in 0..right_matrix[0].len() {
                        lesson_trace::trace_step!(column_index);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        left_input_rates_of_change[row_index][shared_index] +=
                        // Выполняем очередное действие, после которого продолжаем следующий шаг.
                        right_matrix[shared_index][column_index];
                        lesson_trace::trace_step!(left_input_rates_of_change);
                    }
                }
            }
            // Производная по B[k,j] — сумма столбца A[*,k].
            for shared_index in 0..right_matrix.len() {
                lesson_trace::trace_step!(shared_index);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for column_index in 0..right_matrix[0].len() {
                    lesson_trace::trace_step!(column_index);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for row_index in 0..left_matrix.len() {
                        lesson_trace::trace_step!(row_index);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        right_input_rates_of_change[shared_index][column_index] +=
                        // Выполняем очередное действие, после которого продолжаем следующий шаг.
                        left_matrix[row_index][shared_index];
                        lesson_trace::trace_step!(right_input_rates_of_change);
                    }
                }
            }
            // Составляем результат из вычисленных значений в указанном порядке.
            (left_input_rates_of_change, right_input_rates_of_change)
        })();
    lesson_trace::trace_step!(left_input_rates_of_change);
    lesson_trace::trace_step!(right_input_rates_of_change);

    // Шаг: Выводим loss и обе матрицы градиентов.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "loss={}, dA={left_input_rates_of_change:?}, dB={right_input_rates_of_change:?}",
        // Составляем результат из вычисленных значений в указанном порядке.
        (|| -> f64 {
            // Используем подготовленное значение в следующем шаге примера.
            /* Складываем элементы матрицы, полученной после умножения двух матриц. */
            // Собираем значения для `left_matrix` в коллекцию.
            let left_matrix: &[Vec<f64>] = &left_matrix;
            lesson_trace::trace_step!(left_matrix);
            lesson_trace::trace_step!(left_matrix);
            // Сохраняем рассчитанное значение `right_matrix` для следующих операций.
            let right_matrix: &[Vec<f64>] = &right_matrix;
            lesson_trace::trace_step!(right_matrix);
            lesson_trace::trace_step!(right_matrix);
            // Выполняем встроенный расчёт один раз и сохраняем результат в `result`.
            let result: Vec<Vec<f64>> = (|| -> Result<Vec<Vec<f64>>, &'static str> {
                // Обновляем значение результатом текущего вычисления.
                /* Для L=sum(A*B) обратный проход: dA=1*B^T, dB=A^T*1. */
                // Собираем значения для `left_matrix` в коллекцию.
                let left_matrix: &[Vec<f64>] = left_matrix;
                lesson_trace::trace_step!(left_matrix);
                lesson_trace::trace_step!(left_matrix);
                // Сохраняем рассчитанное значение `right_matrix` для следующих операций.
                let right_matrix: &[Vec<f64>] = right_matrix;
                lesson_trace::trace_step!(right_matrix);
                lesson_trace::trace_step!(right_matrix);
                // Отдельно обрабатываем пустой набор, чтобы избежать неверного расчёта.
                if left_matrix.is_empty()
                    // Задаём параметры короткого локального вычисления.
                    || right_matrix.is_empty()
                    // Задаём параметры короткого локального вычисления.
                    || left_matrix
                        // Перебираем элементы по ссылке, не копируя исходную коллекцию.
                        .iter()
                        // Проверяем логическое условие для текущих элементов.
                        .any(|row| row.len() != right_matrix.len())
                    // Задаём параметры короткого локального вычисления.
                    || right_matrix
                        // Перебираем элементы по ссылке, не копируя исходную коллекцию.
                        .iter()
                        // Проверяем логическое условие для текущих элементов.
                        .any(|row| row.len() != right_matrix[0].len())
                {
                    // Прерываем расчёт и явно сообщаем причину некорректного входа.
                    return Err("несовместимые формы");
                }
                // Создаём набор значений `result` для следующего шага примера.
                let mut result: Vec<Vec<f64>> =
                    vec![vec![0.0; right_matrix[0].len()]; left_matrix.len()];
                lesson_trace::trace_step!(result);
                lesson_trace::trace_step!(result);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for row_index in 0..left_matrix.len() {
                    lesson_trace::trace_step!(row_index);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for column_index in 0..right_matrix[0].len() {
                        lesson_trace::trace_step!(column_index);
                        // Повторяем следующий блок для каждого элемента указанной последовательности.
                        for shared_index in 0..right_matrix.len() {
                            lesson_trace::trace_step!(shared_index);
                            // Прибавляем очередной вклад к ранее накопленному результату.
                            result[row_index][column_index] += left_matrix[row_index]
                                // Составляем результат из вычисленных значений в указанном порядке.
                                [shared_index]
                                // Добавляем этот член в составное арифметическое выражение.
                                * right_matrix[shared_index][column_index];
                            lesson_trace::trace_step!(result);
                        }
                    }
                }
                // Возвращаем успешное значение в типе `Result`.
                Ok(result)
            })()
            // Извлекаем значение: выше в примере обеспечено отсутствие ошибки.
            .unwrap();
            lesson_trace::trace_step!(result);
            lesson_trace::trace_step!(result);
            // Инициализируем изменяемый накопитель `output_sum` начальным состоянием.
            let mut output_sum: f64 = 0.0;
            lesson_trace::trace_step!(output_sum);
            lesson_trace::trace_step!(output_sum);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for row in &result {
                lesson_trace::trace_step!(row);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for &value in row {
                    lesson_trace::trace_step!(value);
                    // Прибавляем очередной вклад к ранее накопленному результату.
                    output_sum += value;
                    lesson_trace::trace_step!(output_sum);
                }
            }
            // Используем ранее рассчитанное значение `output_sum` в текущем выражении.
            output_sum
        })()
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_reverse_mode_autodiff_with_tensor_shapes_and_gradient_check(
        left_input_rates_of_change,
        right_input_rates_of_change,
    );
}

// Строим график по результатам урока.
fn visualize_practice_reverse_mode_autodiff_with_tensor_shapes_and_gradient_check(
    left_input_rates_of_change: std::vec::Vec<std::vec::Vec<f64>>,
    right_input_rates_of_change: std::vec::Vec<std::vec::Vec<f64>>,
) {
    // Цветом показываем вклад каждого элемента входных матриц в градиент.
    for (name, title, values) in [
        (
            // Передаём подпись или текстовое значение для следующего шага.
            "left-gradient",
            // Передаём подпись или текстовое значение для следующего шага.
            "Производная по левой матрице",
            // Используем подготовленное значение в следующем шаге примера.
            &left_input_rates_of_change,
        ),
        (
            // Передаём подпись или текстовое значение для следующего шага.
            "right-gradient",
            // Передаём подпись или текстовое значение для следующего шага.
            "Производная по правой матрице",
            // Используем подготовленное значение в следующем шаге примера.
            &right_input_rates_of_change,
        ),
    ] {
        // Строим график по рассчитанным значениям и сохраняем его как SVG.
        let chart: std::path::PathBuf =
            lesson_visualization::heatmap(env!("CARGO_MANIFEST_DIR"), name, title, values)
                // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
                .expect("не удалось сохранить график градиента");
        // Печатаем путь к созданному SVG, чтобы его можно было открыть.
        println!("график: {}", chart.display());
    }
}
