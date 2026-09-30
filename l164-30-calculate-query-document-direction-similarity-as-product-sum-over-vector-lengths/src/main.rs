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
    lesson_trace::trace_note!("Создаём набор значений `query` для следующего шага примера.");
    let query: [f64; 2] = [1.0, 0.0];
    lesson_trace::trace_step!(query);
    lesson_trace::trace_note!("Создаём набор значений `document` для следующего шага примера.");
    let document: [f64; 2] = [2.0, 0.0];
    lesson_trace::trace_step!(document);
    lesson_trace::trace_note!(
        "Тот же косинус из урока 01.6 теперь сравнивает векторы слов документов."
    );
    lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
    let similarity: f64 = l006_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&query, &document)

        .expect("ненулевые векторы слов одинаковой размерности");
    lesson_trace::trace_step!(similarity);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("косинусное сходство={similarity}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_document_direction_similarity_as_coordinate_product_sum_divided_by_lengths(query);
}

// Строим график по результатам урока.
fn plot_document_direction_similarity_as_coordinate_product_sum_divided_by_lengths(
    query: [f64; 2],
) {
    lesson_trace::trace_note!("Собираем значения для `points` в коллекцию.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let points: Vec<(f64, f64)> = (0..=180)

        .step_by(5)

        .map(|degrees| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `angle`.");
            let angle: f64 = (degrees as f64).to_radians();
            lesson_trace::trace_note!("Задаём учебные значения для `rotated_document`.");
            let rotated_document: [f64; 2] = [angle.cos(), angle.sin()];
            lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
            lesson_trace::trace_note!("Задаём именованное поле или параметр.");
            (

                degrees as f64,

                l006_01_calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths::calculate_direction_similarity_as_coordinate_product_sum_divided_by_vector_lengths(&query, &rotated_document)
                    .unwrap(),
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
        "Сходство документов при изменении направления",
        "угол, градусы",
        "косинус",
        &[lesson_visualization::Series {
            name: "сходство",

            points: &points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
