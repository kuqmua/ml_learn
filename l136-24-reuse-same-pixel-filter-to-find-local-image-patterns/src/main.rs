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
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `image` для следующего шага примера.");
    let image: [f64; 4] = [1.0, 3.0, 2.0, 5.0];
    lesson_trace::trace_step!(image);
    lesson_trace::trace_note!(
        "Создаём набор значений `filter_weights` для следующего шага примера."
    );
    lesson_trace::trace_note!("Небольшой набор весов свёрточного фильтра называют kernel.");
    let filter_weights: [f64; 2] = [-1.0, 1.0];
    lesson_trace::trace_step!(filter_weights);
    lesson_trace::trace_note!(
        "Формула ниже использует ровно два коэффициента ядра и два соседних пикселя."
    );
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert_eq!(
        filter_weights.len(),
        2,
        "этот пример рассчитан на ядро из двух значений"
    );
    lesson_trace::trace_note!("Ядро должно помещаться в изображение, иначе окно считать нельзя.");
    lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !filter_weights.is_empty() && image.len() >= filter_weights.len(),
        "ядро должно быть непустым и не длиннее изображения"
    );
    lesson_trace::trace_note!("Повторяем следующий блок для каждого положения окна.");
    for start in 0..=image.len() - filter_weights.len() {
        lesson_trace::trace_step!(start);
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `response`.");
        let response: f64 = image[start] * filter_weights[0] + image[start + 1] * filter_weights[1];
        lesson_trace::trace_step!(response);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("позиция={start}, отклик на границу={response}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_image_filter_response_as_local_weighted_pixel_sums_at_each_position(image, filter_weights);
}

// Строим график по результатам урока.
fn plot_image_filter_response_as_local_weighted_pixel_sums_at_each_position(
    image: [f64; 4],
    filter_weights: [f64; 2],
) {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let local_features_points: Vec<(f64, f64)> = (0..=image.len() - filter_weights.len())
        .map(|plot_step_index| {
            (
                plot_step_index as f64,
                image[plot_step_index] * filter_weights[0]
                    + image[plot_step_index + 1] * filter_weights[1],
            )
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Отклик ядра на локальные границы",
        "позиция",
        "отклик",
        &[lesson_visualization::Series {
            name: "свёртка",

            points: &local_features_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
