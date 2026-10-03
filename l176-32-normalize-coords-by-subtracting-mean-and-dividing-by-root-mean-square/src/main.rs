// Урок 32.3. Нормализация координат: вычитание среднего и деление на корень из среднего квадрата отклонений.
// Связь с принятой терминологией: Нормализация координат одного токена в слое LayerNorm.
// Зачем здесь эта тема: Величины координат токена могут меняться между слоями и мешать устойчивому
//   обучению.
// Почему код устроен так: Вычитаем среднее и делим на масштаб внутри одного токена, добавляя
//   epsilon.
// Представь: Нормируем координаты внутри одного токена, чтобы следующий слой не зависел от случайно
//   большого масштаба.
//
// Что изучаем: Layer normalization.
// Зачем это нужно: Нормализуем координаты одного токена по его среднему и дисперсии, затем добавляем малое
// epsilon для устойчивости.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let text_unit: [f64; 2] = [1.0, 3.0];
    let mean: f64 = (text_unit[0] + text_unit[1]) / 2.0;
    let variance: f64 = ((text_unit[0] - mean) * (text_unit[0] - mean)
        + (text_unit[1] - mean) * (text_unit[1] - mean))
        / 2.0;
    let squared_scale: f64 = variance + 0.00001;
    let mut scale: f64 = squared_scale;
    for _ in 0..80 {
        scale = (scale + squared_scale / scale) / 2.0;
    }
    let coords_in_standard_deviation_units_where_0_means_mean_and_sign_shows_side_of_mean: [f64;
        2] = [(text_unit[0] - mean) / scale, (text_unit[1] - mean) / scale];

    plot_normalized_coords_after_subtracting_mean_and_dividing_by_spread(
        coords_in_standard_deviation_units_where_0_means_mean_and_sign_shows_side_of_mean,
    );
}

// Строим график по результатам урока.
fn plot_normalized_coords_after_subtracting_mean_and_dividing_by_spread(
    coords_in_standard_deviation_units_where_0_means_mean_and_sign_shows_side_of_mean: [f64; 2],
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Layer normalization",
        "измерение",
        "значение",
        &[lesson_visualization::Series {
            name: "нормализованный токен",

            points:
                &coords_in_standard_deviation_units_where_0_means_mean_and_sign_shows_side_of_mean
                    .iter()
                    .enumerate()
                    .map(|(item_index, &element_value)| (item_index as f64, element_value))
                    .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
