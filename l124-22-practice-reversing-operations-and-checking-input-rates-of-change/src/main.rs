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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Шаг: Задаём две малые матрицы, результат умножения можно проверить вручную.");
    let left_matrix: Vec<Vec<f64>> = vec![vec![1., 2.]];
    trace_step!(left_matrix);
    trace_note!("Создаём набор значений `right_matrix` для следующего шага примера.");
    let right_matrix: Vec<Vec<f64>> = vec![vec![3.], vec![4.]];
    trace_step!(right_matrix);

    trace_note!("Шаг: Вычисляем производные суммы элементов результата по обеим матрицам.");
    trace_note!("Производную функции по параметру или вектор таких производных называют gradient.");
    let (left_input_rates_of_change, right_input_rates_of_change): (Vec<Vec<f64>>, Vec<Vec<f64>>) =
        (|| -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
            trace_note!("Используем подготовленное значение в следующем шаге примера.");
            trace_note!(
                "Для суммы элементов результата умножения матриц вычисляем производные по обоим входам."
            );
            trace_note!("Собираем значения для `left_matrix` в коллекцию.");
            let left_matrix: &[Vec<f64>] = &left_matrix;
            trace_step!(left_matrix);
            trace_note!("Сохраняем рассчитанное значение `right_matrix` для следующих операций.");
            let right_matrix: &[Vec<f64>] = &right_matrix;
            trace_step!(right_matrix);
            trace_note!(
                "Создаём набор значений `left_input_rates_of_change` для следующего шага примера."
            );
            let mut left_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix.len()]; left_matrix.len()];
            trace_step!(left_input_rates_of_change);
            trace_note!(
                "Создаём набор значений `right_input_rates_of_change` для следующего шага примера."
            );
            let mut right_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix[0].len()]; right_matrix.len()];
            trace_step!(right_input_rates_of_change);
            trace_note!("Производная суммы элементов A·B по A[i,k] — сумма строки B[k,*].");
            for row_index in 0..left_matrix.len() {
                trace_step!(row_index);
                trace_note!(
                    "Повторяем следующий блок для каждого элемента указанной последовательности."
                );
                for shared_index in 0..right_matrix.len() {
                    trace_step!(shared_index);
                    trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for column_index in 0..right_matrix[0].len() {
                        trace_step!(column_index);
                        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                        trace_note!(
                            "Выполняем очередное действие, после которого продолжаем следующий шаг."
                        );
                        left_input_rates_of_change[row_index][shared_index] +=
                            right_matrix[shared_index][column_index];
                        trace_step!(left_input_rates_of_change);
                    }
                }
            }
            trace_note!("Производная по B[k,j] — сумма столбца A[*,k].");
            for shared_index in 0..right_matrix.len() {
                trace_step!(shared_index);
                trace_note!(
                    "Повторяем следующий блок для каждого элемента указанной последовательности."
                );
                for column_index in 0..right_matrix[0].len() {
                    trace_step!(column_index);
                    trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for row_index in 0..left_matrix.len() {
                        trace_step!(row_index);
                        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                        trace_note!(
                            "Выполняем очередное действие, после которого продолжаем следующий шаг."
                        );
                        right_input_rates_of_change[shared_index][column_index] +=
                            left_matrix[row_index][shared_index];
                        trace_step!(right_input_rates_of_change);
                    }
                }
            }
            trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
            (left_input_rates_of_change, right_input_rates_of_change)
        })();
    trace_step!(left_input_rates_of_change);
    trace_step!(right_input_rates_of_change);

    trace_note!("Шаг: Выводим loss и обе матрицы градиентов.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Складываем элементы матрицы, полученной после умножения двух матриц.");
    trace_note!("Собираем значения для `left_matrix` в коллекцию.");
    trace_note!("Сохраняем рассчитанное значение `right_matrix` для следующих операций.");
    trace_note!("Выполняем встроенный расчёт один раз и сохраняем результат в `result`.");
    trace_note!("Обновляем значение результатом текущего вычисления.");
    trace_note!("Для L=sum(A*B) обратный проход: dA=1*B^T, dB=A^T*1.");
    trace_note!("Собираем значения для `left_matrix` в коллекцию.");
    trace_note!("Сохраняем рассчитанное значение `right_matrix` для следующих операций.");
    trace_note!("Отдельно обрабатываем пустой набор, чтобы избежать неверного расчёта.");
    trace_note!("Задаём параметры короткого локального вычисления.");
    trace_note!("Задаём параметры короткого локального вычисления.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Проверяем логическое условие для текущих элементов.");
    trace_note!("Задаём параметры короткого локального вычисления.");
    trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    trace_note!("Проверяем логическое условие для текущих элементов.");
    trace_note!("Прерываем расчёт и явно сообщаем причину некорректного входа.");
    trace_note!("Создаём набор значений `result` для следующего шага примера.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Добавляем этот член в составное арифметическое выражение.");
    trace_note!("Возвращаем успешное значение в типе `Result`.");
    trace_note!("Извлекаем значение: выше в примере обеспечено отсутствие ошибки.");
    trace_note!("Инициализируем изменяемый накопитель `output_sum` начальным состоянием.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
    trace_note!("Используем ранее рассчитанное значение `output_sum` в текущем выражении.");
    println!(
        "loss={}, dA={left_input_rates_of_change:?}, dB={right_input_rates_of_change:?}",
        (|| -> f64 {
            let left_matrix: &[Vec<f64>] = &left_matrix;
            trace_step!(left_matrix);
            trace_step!(left_matrix);

            let right_matrix: &[Vec<f64>] = &right_matrix;
            trace_step!(right_matrix);
            trace_step!(right_matrix);

            let result: Vec<Vec<f64>> = (|| -> Result<Vec<Vec<f64>>, &'static str> {
                let left_matrix: &[Vec<f64>] = left_matrix;
                trace_step!(left_matrix);
                trace_step!(left_matrix);

                let right_matrix: &[Vec<f64>] = right_matrix;
                trace_step!(right_matrix);
                trace_step!(right_matrix);

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
                trace_step!(result);
                trace_step!(result);

                for row_index in 0..left_matrix.len() {
                    trace_step!(row_index);

                    for column_index in 0..right_matrix[0].len() {
                        trace_step!(column_index);

                        for shared_index in 0..right_matrix.len() {
                            trace_step!(shared_index);

                            result[row_index][column_index] += left_matrix[row_index][shared_index]
                                * right_matrix[shared_index][column_index];
                            trace_step!(result);
                        }
                    }
                }

                Ok(result)
            })()
            .unwrap();
            trace_step!(result);
            trace_step!(result);

            let mut output_sum: f64 = 0.0;
            trace_step!(output_sum);
            trace_step!(output_sum);

            for row in &result {
                trace_step!(row);

                for &value in row {
                    trace_step!(value);

                    output_sum += value;
                    trace_step!(output_sum);
                }
            }

            output_sum
        })()
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
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
    trace_note!("Цветом показываем вклад каждого элемента входных матриц в градиент.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
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
        trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
        trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
        let chart: std::path::PathBuf =
            lesson_visualization::heatmap(env!("CARGO_MANIFEST_DIR"), name, title, values)
                .expect("не удалось сохранить график градиента");
        trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
