// Урок 24.6. Практика: перемещение фильтра по изображению, нулевые края и выбор локальных максимумов.
// Связь с принятой терминологией: Свёртка изображения с шагом, дополнением и pooling.
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
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Шаг: Создаём одноканальное изображение 3×3.");
    let image: Vec<Vec<f64>> = vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]];
    trace_step!(image);
    trace_note!("Шаг: Задаём ядро 2×2, реагирующее на локальную разницу значений.");
    trace_note!("Небольшой набор весов свёрточного фильтра называют kernel.");
    let filter_weights: Vec<Vec<f64>> = vec![vec![1., 0.], vec![0., -1.]];
    trace_step!(filter_weights);

    trace_note!("Скользящую взвешенную сумму называют convolution (свёрткой).");
    trace_note!("Шаг: Проводим свёртку, затем уменьшаем карту признаков max pooling.");
    let feature_map: Vec<Vec<f64>> = (|| -> Vec<Vec<f64>> {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!(
            "Сдвигаем ядро по изображению и умножаем соответствующие значения и складываем результаты."
        );
        trace_note!("Собираем значения для `image` в коллекцию.");
        let image: &[Vec<f64>] = &image;
        trace_step!(image);
        trace_note!("Сохраняем рассчитанное значение `filter_weights` для следующих операций.");
        let filter_weights: &[Vec<f64>] = &filter_weights;
        trace_step!(filter_weights);
        trace_note!("Сохраняем рассчитанное значение `filter_step_size` для следующих операций.");
        trace_note!("Шаг перемещения фильтра по входу называют stride.");
        let filter_step_size: usize = 1;
        trace_step!(filter_step_size);
        trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
        assert!(filter_step_size > 0);
        trace_note!("Считаем количество элементов и сохраняем его в `rows`.");
        let rows: usize = (image.len() - filter_weights.len()) / filter_step_size + 1;
        trace_step!(rows);
        trace_note!("Считаем количество элементов и сохраняем его в `column_count`.");
        let column_count: usize = (image[0].len() - filter_weights[0].len()) / filter_step_size + 1;
        trace_step!(column_count);
        trace_note!("Создаём набор значений `output` для следующего шага примера.");
        let mut output: Vec<Vec<f64>> = vec![vec![0.0; column_count]; rows];
        trace_step!(output);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for output_row in 0..rows {
            trace_step!(output_row);
            trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            for output_column in 0..column_count {
                trace_step!(output_column);
                trace_note!(
                    "Повторяем следующий блок для каждого элемента указанной последовательности."
                );
                for filter_row in 0..filter_weights.len() {
                    trace_step!(filter_row);
                    trace_note!(
                        "Повторяем следующий блок для каждого элемента указанной последовательности."
                    );
                    for filter_column in 0..filter_weights[0].len() {
                        trace_step!(filter_column);
                        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                        trace_note!("Добавляем этот член в составное арифметическое выражение.");
                        trace_note!(
                            "Составляем результат из вычисленных значений в указанном порядке."
                        );
                        output[output_row][output_column] += filter_weights[filter_row]
                            [filter_column]
                            * image[output_row * filter_step_size + filter_row]
                                [output_column * filter_step_size + filter_column];
                        trace_step!(output);
                    }
                }
            }
        }
        trace_note!("Используем ранее рассчитанное значение `output` в текущем выражении.");
        output
    })();
    trace_step!(feature_map);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("В каждом окне 2×2 оставляем максимальный элемент.");
    trace_note!("Собираем значения для `image` в коллекцию.");
    trace_note!("Создаём набор значений `local_maximum_values` для следующего шага примера.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Создаём изменяемое значение `largest_value` для следующих операций.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Сохраняем рассчитанное значение `candidate` для следующих операций.");
    trace_note!("Умножаем величины согласно используемой формуле.");
    trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
    trace_note!("Обновляем `largest_value` результатом текущего шага.");
    trace_note!(
        "Обновляем `local_maximum_values[output_row][output_column]` результатом текущего шага."
    );
    trace_note!(
        "Используем ранее рассчитанное значение `local_maximum_values` в текущем выражении."
    );
    println!(
        "convolution={feature_map:?}, local_maximum_values={:?}",
        (|| -> Vec<Vec<f64>> {
            let image: &[Vec<f64>] = &feature_map;
            trace_step!(image);
            trace_step!(image);

            let mut local_maximum_values: Vec<Vec<f64>> =
                vec![vec![0.0; image[0].len() / 2]; image.len() / 2];
            trace_step!(local_maximum_values);
            trace_step!(local_maximum_values);

            for output_row in 0..local_maximum_values.len() {
                trace_step!(output_row);

                for output_column in 0..local_maximum_values[0].len() {
                    trace_step!(output_column);

                    let mut largest_value: f64 = f64::NEG_INFINITY;
                    trace_step!(largest_value);
                    trace_step!(largest_value);

                    for local_row in 0..2 {
                        trace_step!(local_row);

                        for local_column in 0..2 {
                            trace_step!(local_column);

                            let candidate: f64 =
                                image[2 * output_row + local_row][2 * output_column + local_column];
                            trace_step!(candidate);
                            trace_step!(candidate);

                            if candidate > largest_value {
                                largest_value = candidate;
                                trace_step!(largest_value);
                            }
                        }
                    }

                    local_maximum_values[output_row][output_column] = largest_value;
                    trace_step!(local_maximum_values);
                }
            }

            local_maximum_values
        })()
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_image_feature_map_as_local_weighted_pixel_sums(feature_map);
}

// Строим график по результатам урока.
fn plot_image_feature_map_as_local_weighted_pixel_sums(
    feature_map: std::vec::Vec<std::vec::Vec<f64>>,
) {
    trace_note!("Значения ячеек видны по цвету и подписи.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок тепловой карты.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Карта признаков после свёртки",
        &feature_map,
    )
    .expect("не удалось сохранить тепловую карту");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
