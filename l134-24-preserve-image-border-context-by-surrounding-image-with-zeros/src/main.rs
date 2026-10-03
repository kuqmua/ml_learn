// Урок 24.3. Обработка краёв изображения: добавление нулевой рамки перед фильтрацией.
// Зачем здесь эта тема: Без дополнения фильтр теряет края и уменьшает размер изображения.
// Почему код устроен так: Добавляем нулевую рамку до свёртки и сравниваем доступные окна.
// Представь: Нулевая рамка даёт фильтру окно и около угловых пикселей изображения.
//
// Что изучаем: Дополнение изображения padding.
// Зачем это нужно: Нулевые значения вокруг изображения позволяют ядру обработать крайние пиксели.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let image: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    let mut image_with_zero_border: [[i32; 4]; 4] = [[0; 4]; 4];
    for row in 0..2 {
        for column in 0..2 {
            image_with_zero_border[row + 1][column + 1] = image[row][column];
        }
    }

    plot_image_surrounded_by_zeros(image_with_zero_border);
}

// Строим график по результатам урока.
fn plot_image_surrounded_by_zeros(image_with_zero_border: [[i32; 4]; 4]) {
    lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Padding: дополненное изображение",
        &image_with_zero_border
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&element_value| element_value as f64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .expect("не удалось сохранить тепловую карту");
}
