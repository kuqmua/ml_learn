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

fn main() {
    // Размер карты вычисляется из размера изображения, ядра и шага фильтра.
    let feature_map: Vec<Vec<f64>> = (|| -> Vec<Vec<f64>> {
        let image: [[f64; 3]; 3] = [[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
        let filter_weights: [[f64; 2]; 2] = [[1., 0.], [0., -1.]];
        let image: &[[f64; 3]; 3] = &image;
        let filter_weights: &[[f64; 2]; 2] = &filter_weights;
        let filter_step_size: usize = 1;
        assert!(filter_step_size > 0);
        let rows: usize = (image.len() - filter_weights.len()) / filter_step_size + 1;
        let column_count: usize = (image[0].len() - filter_weights[0].len()) / filter_step_size + 1;
        let mut output: Vec<Vec<f64>> = vec![vec![0.0; column_count]; rows];
        for output_row in 0..rows {
            for output_column in 0..column_count {
                for filter_row in 0..filter_weights.len() {
                    for filter_column in 0..filter_weights[0].len() {
                        output[output_row][output_column] += filter_weights[filter_row]
                            [filter_column]
                            * image[output_row * filter_step_size + filter_row]
                                [output_column * filter_step_size + filter_column];
                    }
                }
            }
        }
        output
    })();
    let _ = &((|| -> Vec<Vec<f64>> {
        let image: &[Vec<f64>] = &feature_map;

        let mut local_maximum_values: Vec<Vec<f64>> =
            vec![vec![0.0; image[0].len() / 2]; image.len() / 2];

        for output_row in 0..local_maximum_values.len() {
            for output_column in 0..local_maximum_values[0].len() {
                let mut largest_value: f64 = f64::NEG_INFINITY;

                for local_row in 0..2 {
                    for local_column in 0..2 {
                        let candidate: f64 =
                            image[2 * output_row + local_row][2 * output_column + local_column];

                        if candidate > largest_value {
                            largest_value = candidate;
                        }
                    }
                }

                local_maximum_values[output_row][output_column] = largest_value;
            }
        }

        local_maximum_values
    })());

    plot_image_feature_map_as_local_weighted_pixel_sums(feature_map);
}

// Строим график по результатам урока.
fn plot_image_feature_map_as_local_weighted_pixel_sums(
    feature_map: std::vec::Vec<std::vec::Vec<f64>>,
) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Карта признаков после свёртки",
        &feature_map,
    )
    .expect("не удалось сохранить тепловую карту");
}
