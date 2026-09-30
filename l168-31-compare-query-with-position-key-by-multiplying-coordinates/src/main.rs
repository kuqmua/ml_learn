// Урок 31.2. Сравнение запроса с ключом позиции через умножение соответствующих координат.
// Связь с принятой терминологией: Вектор ключа K для позиции механизма внимания.
// Зачем здесь эта тема: Чтобы запрос оценил релевантность позиции, у той должен быть сравнимый
//   ключ.
// Почему код устроен так: Строим K из состояния токена и сравниваем его с Q скалярным
//   произведением.
// Представь: Ключ позиции позволяет запросу решить, насколько эта позиция подходит.
//
// Что изучаем: Вектор ключа K.
// Зачем это нужно: Key описывает, на какой запрос позиция отвечает; сравнение Q·K даёт оценку внимания.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `query` для следующего шага примера.");
    let query: [f64; 2] = [1.0, 0.5];
    lesson_trace::trace_step!(query);
    lesson_trace::trace_note!("Создаём набор значений `key` для следующего шага примера.");
    let key: [f64; 2] = [0.8, 0.2];
    lesson_trace::trace_step!(key);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `score`.");
    lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
    let score: f64 =
        l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
            &query, &key,
        )

        .expect("запрос и ключ имеют одинаковую размерность");
    lesson_trace::trace_step!(score);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("Q·K = {score}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_matching_query_and_key_coordinate_products(query, key, score);
}

// Строим график по результатам урока.
fn plot_matching_query_and_key_coordinate_products(query: [f64; 2], key: [f64; 2], score: f64) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Вклады координат в Q·K",
        "вклад",
        &[
            ("координата 0", query[0] * key[0]),
            ("координата 1", query[1] * key[1]),
            ("сумма", score),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
