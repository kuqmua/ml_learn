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
    // Здесь матрицы остаются динамическими: ниже упражнение проверяет совместимость их форм.
    let left_matrix: Vec<Vec<f64>> = vec![vec![1., 2.]];
    let right_matrix: Vec<Vec<f64>> = vec![vec![3.], vec![4.]];

    let (left_input_rates_of_change, right_input_rates_of_change): (Vec<Vec<f64>>, Vec<Vec<f64>>) =
        (|| -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
            let left_matrix: &[Vec<f64>] = &left_matrix;
            let right_matrix: &[Vec<f64>] = &right_matrix;
            let mut left_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix.len()]; left_matrix.len()];
            let mut right_input_rates_of_change: Vec<Vec<f64>> =
                vec![vec![0.0; right_matrix[0].len()]; right_matrix.len()];
            for row_index in 0..left_matrix.len() {
                for shared_index in 0..right_matrix.len() {
                    for column_index in 0..right_matrix[0].len() {
                        left_input_rates_of_change[row_index][shared_index] +=
                            right_matrix[shared_index][column_index];
                    }
                }
            }
            for shared_index in 0..right_matrix.len() {
                for column_index in 0..right_matrix[0].len() {
                    for row_index in 0..left_matrix.len() {
                        right_input_rates_of_change[shared_index][column_index] +=
                            left_matrix[row_index][shared_index];
                    }
                }
            }
            (left_input_rates_of_change, right_input_rates_of_change)
        })();

    let _ = &((|| -> f64 {
        let left_matrix: &[Vec<f64>] = &left_matrix;

        let right_matrix: &[Vec<f64>] = &right_matrix;

        let result: Vec<Vec<f64>> = (|| -> Result<Vec<Vec<f64>>, &'static str> {
            let left_matrix: &[Vec<f64>] = left_matrix;

            let right_matrix: &[Vec<f64>] = right_matrix;

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

            for row_index in 0..left_matrix.len() {
                for column_index in 0..right_matrix[0].len() {
                    for shared_index in 0..right_matrix.len() {
                        result[row_index][column_index] += left_matrix[row_index][shared_index]
                            * right_matrix[shared_index][column_index];
                    }
                }
            }

            Ok(result)
        })()
        .unwrap();

        let mut output_sum: f64 = 0.0;

        for row in &result {
            for &value in row {
                output_sum += value;
            }
        }

        output_sum
    })());

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
        let _chart: std::path::PathBuf =
            lesson_visualization::heatmap(env!("CARGO_MANIFEST_DIR"), name, title, values)
                .expect("не удалось сохранить график градиента");
    }
}
