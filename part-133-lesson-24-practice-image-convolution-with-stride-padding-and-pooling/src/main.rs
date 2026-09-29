// Сводная практика 24. Свёртка изображения с шагом, дополнением и pooling.
// Зачем здесь эта тема: Сверточный блок определяется совместно ядром, шагом, дополнением и pooling.
// Почему код устроен так: На маленькой картинке проверяем промежуточную и итоговую форму после
//   каждого шага.
// Представь: Чтобы предсказать размер выхода, нужно учитывать размер ядра, шаг и рамку ещё до
//   pooling.
//
// Что повторяем вместе: ядро, stride, padding, pooling, локальные признаки.
// Зачем это нужно: Свёртка ищет локальные шаблоны изображения, а pooling уменьшает пространственный размер
//   карты признаков.
// Что показывает программа: Создаём одноканальное изображение 3×3. Задаём ядро 2×2, реагирующее на
//   локальную разницу значений. Проводим свёртку, затем уменьшаем карту признаков max pooling.
// Что проверить при изменении примера: Сверь небольшой результат с ручным расчётом; проверь выходную форму
//   для разных stride/padding.
// Дополнительная практика: Реализуй 2D свёртку для одноканального изображения и max pooling.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Создаём одноканальное изображение 3×3.
    let image: Vec<Vec<f64>> = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];
    lesson_trace::trace_step!(image);
    // Шаг: Задаём ядро 2×2, реагирующее на локальную разницу значений.
    // Небольшой набор весов свёрточного фильтра называют kernel.
    let filter_weights: Vec<Vec<f64>> = vec![vec![1., 0.], vec![0., -1.]];
    lesson_trace::trace_step!(filter_weights);

    // Скользящую взвешенную сумму называют convolution (свёрткой).
    // Шаг: Проводим свёртку, затем уменьшаем карту признаков max pooling.
    let feature_map: Vec<Vec<f64>> = (|| -> Vec<Vec<f64>> {
        // Используем подготовленное значение в следующем шаге примера.
        /* Сдвигаем ядро по изображению и умножаем соответствующие значения и складываем результаты. */
        // Собираем значения для `image` в коллекцию.
        let image: &[Vec<f64>] = &image;
        lesson_trace::trace_step!(image);
        // Сохраняем рассчитанное значение `filter_weights` для следующих операций.
        let filter_weights: &[Vec<f64>] = &filter_weights;
        lesson_trace::trace_step!(filter_weights);
        // Сохраняем рассчитанное значение `filter_step_size` для следующих операций.
        // Шаг перемещения фильтра по входу называют stride.
        let filter_step_size: usize = 1;
        lesson_trace::trace_step!(filter_step_size);
        // Проверяем обязательное условие до дальнейшего вычисления.
        assert!(filter_step_size > 0);
        // Считаем количество элементов и сохраняем его в `rows`.
        let rows: usize = (image.len() - filter_weights.len()) / filter_step_size + 1;
        lesson_trace::trace_step!(rows);
        // Считаем количество элементов и сохраняем его в `column_count`.
        let column_count: usize = (image[0].len() - filter_weights[0].len()) / filter_step_size + 1;
        lesson_trace::trace_step!(column_count);
        // Создаём набор значений `output` для следующего шага примера.
        let mut output: Vec<Vec<f64>> = vec![vec![0.0; column_count]; rows];
        lesson_trace::trace_step!(output);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for output_row in 0..rows {
            lesson_trace::trace_step!(output_row);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for output_column in 0..column_count {
                lesson_trace::trace_step!(output_column);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for filter_row in 0..filter_weights.len() {
                    lesson_trace::trace_step!(filter_row);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for filter_column in 0..filter_weights[0].len() {
                        lesson_trace::trace_step!(filter_column);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        output[output_row][output_column] += filter_weights[filter_row][filter_column]
                            // Добавляем этот член в составное арифметическое выражение.
                            * image[output_row * filter_step_size + filter_row]
                                // Составляем результат из вычисленных значений в указанном порядке.
                                [output_column * filter_step_size + filter_column];
                        lesson_trace::trace_step!(output);
                    }
                }
            }
        }
        // Используем ранее рассчитанное значение `output` в текущем выражении.
        output
    })();
    lesson_trace::trace_step!(feature_map);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "convolution={feature_map:?}, local_maximum_values={:?}",
        // Составляем результат из вычисленных значений в указанном порядке.
        (|| -> Vec<Vec<f64>> {
            // Используем подготовленное значение в следующем шаге примера.
            /* В каждом окне 2×2 оставляем максимальный элемент. */
            // Собираем значения для `image` в коллекцию.
            let image: &[Vec<f64>] = &feature_map;
            lesson_trace::trace_step!(image);
            lesson_trace::trace_step!(image);
            // Создаём набор значений `local_maximum_values` для следующего шага примера.
            let mut local_maximum_values: Vec<Vec<f64>> =
                vec![vec![0.0; image[0].len() / 2]; image.len() / 2];
            lesson_trace::trace_step!(local_maximum_values);
            lesson_trace::trace_step!(local_maximum_values);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for output_row in 0..local_maximum_values.len() {
                lesson_trace::trace_step!(output_row);
                // Повторяем следующий блок для каждого элемента указанной последовательности.
                for output_column in 0..local_maximum_values[0].len() {
                    lesson_trace::trace_step!(output_column);
                    // Создаём изменяемое значение `largest_value` для следующих операций.
                    let mut largest_value: f64 = f64::NEG_INFINITY;
                    lesson_trace::trace_step!(largest_value);
                    lesson_trace::trace_step!(largest_value);
                    // Повторяем следующий блок для каждого элемента указанной последовательности.
                    for local_row in 0..2 {
                        lesson_trace::trace_step!(local_row);
                        // Повторяем следующий блок для каждого элемента указанной последовательности.
                        for local_column in 0..2 {
                            lesson_trace::trace_step!(local_column);
                            // Сохраняем рассчитанное значение `candidate` для следующих операций.
                            let candidate: f64 =
                                // Умножаем величины согласно используемой формуле.
                                image[2 * output_row + local_row][2 * output_column + local_column];
                            lesson_trace::trace_step!(candidate);
                            lesson_trace::trace_step!(candidate);
                            // Проверяем условие и выбираем соответствующую ветку алгоритма.
                            if candidate > largest_value {
                                // Обновляем `largest_value` результатом текущего шага.
                                largest_value = candidate;
                                lesson_trace::trace_step!(largest_value);
                            }
                        }
                    }
                    // Обновляем `local_maximum_values[output_row][output_column]` результатом текущего шага.
                    local_maximum_values[output_row][output_column] = largest_value;
                    lesson_trace::trace_step!(local_maximum_values);
                }
            }
            // Используем ранее рассчитанное значение `local_maximum_values` в текущем выражении.
            local_maximum_values
        })()
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_image_convolution_with_stride_padding_and_pooling(feature_map);
}

// Строим график по результатам урока.
fn visualize_practice_image_convolution_with_stride_padding_and_pooling(
    feature_map: std::vec::Vec<std::vec::Vec<f64>>,
) {
    // Значения ячеек видны по цвету и подписи.
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок тепловой карты.
        "Карта признаков после свёртки",
        // Используем подготовленное значение в следующем шаге примера.
        &feature_map,
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить тепловую карту");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
