// Урок 24.4. Уменьшение изображения: выбор максимума в каждом локальном окне.
// Зачем здесь эта тема: После фильтра соседние сильные отклики можно свести к одному признаку.
// Почему код устроен так: Берём максимум в каждом локальном окне и показываем потерю точного
//   положения.
// Представь: Из окна значений [1, 3, 2, 0] max pooling оставит 3 и забудет точное место этого пика.
//
// Что изучаем: Max pooling.
// Зачем это нужно: Pooling оставляет наиболее сильный отклик в локальном окне и уменьшает пространственный
// размер.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let feature_map: [[f64; 2]; 2] = [[1.0, 4.0], [3.0, 2.0]];
    assert!(
        !feature_map.is_empty() && !feature_map[0].is_empty(),
        "карта признаков не должна быть пустой"
    );
    let mut maximum: f64 = feature_map[0][0];
    for row in feature_map {
        for value in row {
            if value > maximum {
                maximum = value;
            }
        }
    }

    plot_largest_values_in_local_image_windows(feature_map, maximum);
}

// Строим график по результатам урока.
fn plot_largest_values_in_local_image_windows(feature_map: [[f64; 2]; 2], maximum: f64) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Max pooling",
        "значение",
        &[
            ("1", feature_map[0][0]),
            ("2", feature_map[0][1]),
            ("3", feature_map[1][0]),
            ("4", feature_map[1][1]),
            ("максимум", maximum),
        ],
    )
    .expect("не удалось сохранить график");
}
