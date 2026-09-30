// Урок 22.5. Практика: обратный проход по операциям и проверка влияния входов на результат.
// Связь с принятой терминологией: Обратный режим дифференцирования и проверка градиента.
// Зачем здесь эта тема: Матричный обратный режим имеет смысл только вместе с проверкой формы и
//   численной проверкой градиента.
// Почему код устроен так: Проводим один малый вычислительный граф вперёд и назад и сравниваем
//   производные.
// Представь: Одно и то же выражение считаем вперёд как число и назад как градиенты входов.
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
    lesson_trace::trace_note!(
        "Шаг: Задаём две малые матрицы, результат умножения можно проверить вручную."
    );
    let left_matrix: Vec<Vec<f64>> = vec![vec![1., 2.]];
    lesson_trace::trace_step!(left_matrix);
    lesson_trace::trace_note!("Создаём набор значений `right_matrix` для следующего шага примера.");
    let right_matrix: Vec<Vec<f64>> = vec![vec![3.], vec![4.]];
    lesson_trace::trace_step!(right_matrix);

    lesson_trace::trace_note!(
        "Шаг: Вычисляем производные суммы элементов результата по обеим матрицам."
    );
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    let (left_input_rates_of_change, right_input_rates_of_change): (Vec<Vec<f64>>, Vec<Vec<f64>>) =
        (|| -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!(
                "Для суммы элементов результата умножения матриц вычисляем производные по обоим входам."
            );
            lesson_trace::trace_note!("Собираем значения для `left_matrix` в коллекцию.");
            let left_matrix: &[Vec<f64>] = &left_matrix;
            lesson_trace::trace_step!(left_matrix);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `right_matrix` для следующих операций."
            );
            let right_matrix: &[Vec<f64>] = &right_matrix;
            lesson_trace::trace_step!(right_matrix);
            lesson_trace::trace_note!(
                "Создаём набор значений `left_input_rates_of_change` для следующего шага примера."
            );
            let mut left_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix.len()]; left_matrix.len()];
            lesson_trace::trace_step!(left_input_rates_of_change);
            lesson_trace::trace_note!(
                "Создаём набор значений `right_input_rates_of_change` для следующего шага примера."
            );
            let mut right_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix[0].len()]; right_matrix.len()];
            lesson_trace::trace_step!(right_input_rates_of_change);
            lesson_trace::trace_note!(
                "Производная суммы элементов A·B по A[i,k] — сумма строки B[k,*]."
            );
            for row_index in 0..left_matrix.len() {
                lesson_trace::trace_step!(row_index);
                lesson_trace::trace_note!(
                    "Повторяем следующий блок для каждого элемента указанной последовательности."
                );
                for shared_index in 0..right_matrix.len() {
                    lesson_trace::trace_step!(shared_index);
                    lesson_trace::trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for column_index in 0..right_matrix[0].len() {
                        lesson_trace::trace_step!(column_index);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        lesson_trace::trace_note!(
                            "Выполняем очередное действие, после которого продолжаем следующий шаг."
                        );
                        left_input_rates_of_change[row_index][shared_index] +=
                            right_matrix[shared_index][column_index];
                        lesson_trace::trace_step!(left_input_rates_of_change);
                    }
                }
            }
            lesson_trace::trace_note!("Производная по B[k,j] — сумма столбца A[*,k].");
            for shared_index in 0..right_matrix.len() {
                lesson_trace::trace_step!(shared_index);
                lesson_trace::trace_note!(
                    "Повторяем следующий блок для каждого элемента указанной последовательности."
                );
                for column_index in 0..right_matrix[0].len() {
                    lesson_trace::trace_step!(column_index);
                    lesson_trace::trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for row_index in 0..left_matrix.len() {
                        lesson_trace::trace_step!(row_index);
                        lesson_trace::trace_note!(
                            "Прибавляем очередной вклад к ранее накопленному результату."
                        );
                        lesson_trace::trace_note!(
                            "Выполняем очередное действие, после которого продолжаем следующий шаг."
                        );
                        right_input_rates_of_change[shared_index][column_index] +=
                            left_matrix[row_index][shared_index];
                        lesson_trace::trace_step!(right_input_rates_of_change);
                    }
                }
            }
            lesson_trace::trace_note!(
                "Составляем результат из вычисленных значений в указанном порядке."
            );
            (left_input_rates_of_change, right_input_rates_of_change)
        })();
    lesson_trace::trace_step!(left_input_rates_of_change);
    lesson_trace::trace_step!(right_input_rates_of_change);

    lesson_trace::trace_note!("Шаг: Выводим loss и обе матрицы градиентов.");
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!(
        "Складываем элементы матрицы, полученной после умножения двух матриц."
    );
    lesson_trace::trace_note!("Собираем значения для `left_matrix` в коллекцию.");
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `right_matrix` для следующих операций."
    );
    lesson_trace::trace_note!(
        "Выполняем встроенный расчёт один раз и сохраняем результат в `result`."
    );
    lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
    lesson_trace::trace_note!("Для L=sum(A*B) обратный проход: dA=1*B^T, dB=A^T*1.");
    lesson_trace::trace_note!("Собираем значения для `left_matrix` в коллекцию.");
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `right_matrix` для следующих операций."
    );
    lesson_trace::trace_note!(
        "Отдельно обрабатываем пустой набор, чтобы избежать неверного расчёта."
    );
    lesson_trace::trace_note!("Задаём параметры короткого локального вычисления.");
    lesson_trace::trace_note!("Задаём параметры короткого локального вычисления.");
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Проверяем логическое условие для текущих элементов.");
    lesson_trace::trace_note!("Задаём параметры короткого локального вычисления.");
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Проверяем логическое условие для текущих элементов.");
    lesson_trace::trace_note!("Прерываем расчёт и явно сообщаем причину некорректного входа.");
    lesson_trace::trace_note!("Создаём набор значений `result` для следующего шага примера.");
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Добавляем этот член в составное арифметическое выражение.");
    lesson_trace::trace_note!("Возвращаем успешное значение в типе `Result`.");
    lesson_trace::trace_note!("Извлекаем значение: выше в примере обеспечено отсутствие ошибки.");
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `output_sum` начальным состоянием."
    );
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    lesson_trace::trace_note!(
        "Используем ранее рассчитанное значение `output_sum` в текущем выражении."
    );
    println!(
        "loss={}, dA={left_input_rates_of_change:?}, dB={right_input_rates_of_change:?}",
        (|| -> f64 {
            let left_matrix: &[Vec<f64>] = &left_matrix;
            lesson_trace::trace_step!(left_matrix);
            lesson_trace::trace_step!(left_matrix);

            let right_matrix: &[Vec<f64>] = &right_matrix;
            lesson_trace::trace_step!(right_matrix);
            lesson_trace::trace_step!(right_matrix);

            let result: Vec<Vec<f64>> = (|| -> Result<Vec<Vec<f64>>, &'static str> {
                let left_matrix: &[Vec<f64>] = left_matrix;
                lesson_trace::trace_step!(left_matrix);
                lesson_trace::trace_step!(left_matrix);

                let right_matrix: &[Vec<f64>] = right_matrix;
                lesson_trace::trace_step!(right_matrix);
                lesson_trace::trace_step!(right_matrix);

                if left_matrix.is_empty()
                    || right_matrix.is_empty()
                    || left_matrix
                        .iter()
                        .any(|row| row.len() != right_matrix.len())
                    || right_matrix
                        .iter()
                        .any(|row| row.len() != right_matrix[0].len())
                {
                    return Err("несовместимые формы");
                }

                let mut result: Vec<Vec<f64>> =
                    vec![vec![0.0; right_matrix[0].len()]; left_matrix.len()];
                lesson_trace::trace_step!(result);
                lesson_trace::trace_step!(result);

                for row_index in 0..left_matrix.len() {
                    lesson_trace::trace_step!(row_index);

                    for column_index in 0..right_matrix[0].len() {
                        lesson_trace::trace_step!(column_index);

                        for shared_index in 0..right_matrix.len() {
                            lesson_trace::trace_step!(shared_index);

                            result[row_index][column_index] += left_matrix[row_index][shared_index]
                                * right_matrix[shared_index][column_index];
                            lesson_trace::trace_step!(result);
                        }
                    }
                }

                Ok(result)
            })()
            .unwrap();
            lesson_trace::trace_step!(result);
            lesson_trace::trace_step!(result);

            let mut output_sum: f64 = 0.0;
            lesson_trace::trace_step!(output_sum);
            lesson_trace::trace_step!(output_sum);

            for row in &result {
                lesson_trace::trace_step!(row);

                for &value in row {
                    lesson_trace::trace_step!(value);

                    output_sum += value;
                    lesson_trace::trace_step!(output_sum);
                }
            }

            output_sum
        })()
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_output_rates_of_change_for_left_and_right_matrix_entries(
        left_input_rates_of_change,
        right_input_rates_of_change,
    );
}

// Строим график по результатам урока.
fn plot_output_rates_of_change_for_left_and_right_matrix_entries(
    left_input_rates_of_change: std::vec::Vec<std::vec::Vec<f64>>,
    right_input_rates_of_change: std::vec::Vec<std::vec::Vec<f64>>,
) {
    lesson_trace::trace_note!(
        "Цветом показываем вклад каждого элемента входных матриц в градиент."
    );
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    for (name, title, values) in [
        (
            "left-gradient",
            "Производная по левой матрице",
            &left_input_rates_of_change,
        ),
        (
            "right-gradient",
            "Производная по правой матрице",
            &right_input_rates_of_change,
        ),
    ] {
        lesson_trace::trace_note!(
            "Строим график по рассчитанным значениям и сохраняем его как SVG."
        );
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf =
            lesson_visualization::heatmap(env!("CARGO_MANIFEST_DIR"), name, title, values)
                .expect("не удалось сохранить график градиента");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
