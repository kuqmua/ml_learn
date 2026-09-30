// Урок 24.3. Обработка краёв изображения: добавление нулевой рамки перед фильтрацией.
// Связь с принятой терминологией: Дополнение изображения нулями перед свёрткой.
// Зачем здесь эта тема: Без дополнения фильтр теряет края и уменьшает размер изображения.
// Почему код устроен так: Добавляем нулевую рамку до свёртки и сравниваем доступные окна.
// Представь: Нулевая рамка даёт фильтру окно и около угловых пикселей изображения.
//
// Что изучаем: Дополнение изображения padding.
// Зачем это нужно: Нулевые значения вокруг изображения позволяют ядру обработать крайние пиксели.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `image` для следующего шага примера.");
    let image: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    lesson_trace::trace_step!(image);
    lesson_trace::trace_note!(
        "Создаём набор значений `image_with_zero_border` для следующего шага примера."
    );
    lesson_trace::trace_note!("Добавление нулевой рамки к изображению называют padding.");
    let mut image_with_zero_border: [[i32; 4]; 4] = [[0; 4]; 4];
    lesson_trace::trace_step!(image_with_zero_border);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for row in 0..2 {
        lesson_trace::trace_step!(row);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for column in 0..2 {
            lesson_trace::trace_step!(column);
            lesson_trace::trace_note!(
                "Присваиваем вычисленное значение соответствующей переменной или полю."
            );
            image_with_zero_border[row + 1][column + 1] = image[row][column];
            lesson_trace::trace_step!(image_with_zero_border);
        }
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("после padding: {image_with_zero_border:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_image_surrounded_by_zeros(image_with_zero_border);
}

// Строим график по результатам урока.
fn plot_image_surrounded_by_zeros(image_with_zero_border: [[i32; 4]; 4]) {
    lesson_trace::trace_note!("Значения ячеек видны по цвету и подписи.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок тепловой карты.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
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
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
