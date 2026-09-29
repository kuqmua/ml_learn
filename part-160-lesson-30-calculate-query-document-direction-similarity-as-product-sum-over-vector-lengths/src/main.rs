// Урок 30.3. Сходство запроса и документа: сумма произведений координат, делённая на произведение длин векторов.
// Связь с принятой терминологией: Косинусное сходство векторов запроса и документа.
// Зачем здесь эта тема: Весов слов недостаточно для ранжирования, если длины документов
//   различаются.
// Почему код устроен так: Сравниваем векторы запроса и документа через косинус, убирая общий
//   масштаб.
// Представь: Два документа одинаковой темы, но разной длины, сравниваются по направлению TF-IDF
//   векторов.
//
// Что изучаем: Косинусное сходство документов.
// Зачем это нужно: Нормировка суммы после попарного умножения координат позволяет сравнивать направления векторов слов, а не
// длину документов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `query` для следующего шага примера.
    let query: [f64; 2] = [1.0, 0.0];
    lesson_trace::trace_step!(query);
    // Создаём набор значений `document` для следующего шага примера.
    let document: [f64; 2] = [2.0, 0.0];
    lesson_trace::trace_step!(document);
    // Тот же косинус из урока 01.5 теперь сравнивает векторы слов документов.
    let similarity: f64 = part_005_lesson_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&query, &document)
        // Используем результат, ожидая успешного выполнения шага.
        .expect("ненулевые векторы слов одинаковой размерности");
    lesson_trace::trace_step!(similarity);
    // Печатаем рассчитанные значения для проверки примера.
    println!("косинусное сходство={similarity}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_document_direction_similarity_as_coordinate_product_sum_divided_by_lengths(query);
}

// Строим график по результатам урока.
fn plot_document_direction_similarity_as_coordinate_product_sum_divided_by_lengths(
    query: [f64; 2],
) {
    // Собираем значения для `points` в коллекцию.
    let points: Vec<(f64, f64)> = (0..=180)
        // Настраиваем или преобразуем результат предыдущего шага.
        .step_by(5)
        // Преобразуем каждый элемент в новое значение.
        .map(|degrees| {
            // Сохраняем результат этого шага в `angle`.
            let angle: f64 = (degrees as f64).to_radians();
            // Задаём учебные значения для `rotated_document`.
            let rotated_document: [f64; 2] = [angle.cos(), angle.sin()];
            (
                // Используем подготовленное значение в следующем шаге примера.
                degrees as f64,
                // Задаём именованное поле или параметр.
                part_005_lesson_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&query, &rotated_document)
                    .unwrap(),
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Сходство документов при изменении направления",
        // Указываем подпись горизонтальной оси.
        "угол, градусы",
        // Указываем подпись вертикальной оси.
        "косинус",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "сходство",
            // Передаём рассчитанные координаты точек.
            points: &points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
