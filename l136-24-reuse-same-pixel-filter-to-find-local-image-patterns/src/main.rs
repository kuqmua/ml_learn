// Урок 24.5. Поиск локальных особенностей изображения одним и тем же фильтром.
// Связь с принятой терминологией: Поиск локальных признаков изображения одним фильтром.
// Зачем здесь эта тема: Набор локальных откликов выявляет повторяющийся рисунок, например край.
// Почему код устроен так: Применяем один и тот же фильтр к разным участкам и сравниваем карту
//   откликов.
// Представь: Один фильтр может сильно отвечать там, где встречается вертикальный край, и слабо в
//   других местах.
//
// Что изучаем: Локальные признаки.
// Зачем это нужно: Один и тот же фильтр применяется в разных областях изображения и ищет одинаковый шаблон
// независимо от позиции.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let filter_weights: [f64; 2] = [-1.0, 1.0];
    assert_eq!(
        filter_weights.len(),
        2,
        "этот пример рассчитан на ядро из двух значений"
    );
    let image: [f64; 4] = [1.0, 3.0, 2.0, 5.0];
    assert!(
        !filter_weights.is_empty() && image.len() >= filter_weights.len(),
        "ядро должно быть непустым и не длиннее изображения"
    );
    for start in 0..=image.len() - filter_weights.len() {
        let _: f64 = image[start] * filter_weights[0] + image[start + 1] * filter_weights[1];
    }

    plot_image_filter_response_as_local_weighted_pixel_sums_at_each_position(image, filter_weights);
}

// Строим график по результатам урока.
fn plot_image_filter_response_as_local_weighted_pixel_sums_at_each_position(
    image: [f64; 4],
    filter_weights: [f64; 2],
) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Отклик ядра на локальные границы",
        "позиция",
        "отклик",
        &[lesson_visualization::Series {
            name: "свёртка",

            points: &(0..=image.len() - filter_weights.len())
                .map(|plot_step_index| {
                    (
                        plot_step_index as f64,
                        image[plot_step_index] * filter_weights[0]
                            + image[plot_step_index + 1] * filter_weights[1],
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
