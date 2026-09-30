// Урок 24.2. Перемещение фильтра по изображению с заданным шагом.
// Связь с принятой терминологией: Сдвиг ядра свёртки по изображению на заданное число пикселей.
// Зачем здесь эта тема: Одно окно даёт только один отклик; шаг определяет, какие окна посетит
//   фильтр.
// Почему код устроен так: Перемещаем ядро на фиксированное число пикселей и проверяем размер
//   выходной карты.
// Представь: При шаге 2 ядро перескакивает через одну позицию, поэтому выходная карта становится
//   меньше.
//
// Что изучаем: Шаг свёртки stride.
// Зачем это нужно: Stride определяет, на сколько пикселей сдвигается ядро после одного применения.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let image_width: i32 = 5;
    let filter_width: i32 = 2;
    let filter_step_size: i32 = 2;
    let output_width: i32 = (image_width - filter_width) / filter_step_size + 1;
    // Число позиций зависит от ширины изображения, ядра и шага фильтра.
    let positions: Vec<i32> = (0..output_width)
        .map(|index| index * filter_step_size)
        .collect();

    plot_image_positions_visited_with_fixed_step_size(positions);
}

// Строим график по результатам урока.
fn plot_image_positions_visited_with_fixed_step_size(positions: std::vec::Vec<i32>) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Stride и позиции ядра",
        "позиция",
        &[
            ("первая", positions[0] as f64),
            ("последняя", *positions.last().unwrap() as f64),
        ],
    )
    .expect("не удалось сохранить график");
}
