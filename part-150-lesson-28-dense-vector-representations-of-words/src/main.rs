// Урок 28.3. Плотные векторные представления слов.
// Зачем здесь эта тема: Числовой id лишь обозначает токен и не выражает его сходство с другими.
// Почему код устроен так: Сопоставляем каждому id плотный вектор, чтобы последующие слои могли
//   обучать признаки.
// Представь: Id 2 не ближе по смыслу к id 3, чем к id 100; близость можно учить в векторах.
//
// Что изучаем: Плотные представления слов.
// Зачем это нужно: Вместо отдельного разреженного признака на слово используем короткий вектор числовых
// признаков.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `dense_representation_table` для следующего шага примера.
    // Плотное числовое представление объекта называют embedding.
    let dense_representation_table: [[f64; 2]; 3] = [[0.0, 0.0], [0.8, 0.2], [0.7, 0.3]];
    lesson_trace::trace_step!(dense_representation_table);
    // Сохраняем рассчитанное значение `text_unit_identifier` для следующих операций.
    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
    let text_unit_identifier: usize = 2;
    lesson_trace::trace_step!(text_unit_identifier);
    // Сохраняем рассчитанное значение `dense_numeric_representation` для следующих операций.
    let dense_numeric_representation: [f64; 2] = dense_representation_table[text_unit_identifier];
    lesson_trace::trace_step!(dense_numeric_representation);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("токен={text_unit_identifier}, плотный вектор={dense_numeric_representation:?}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_dense_vector_representations_of_words(dense_numeric_representation);
}

// Строим график по результатам урока.
fn visualize_dense_vector_representations_of_words(dense_numeric_representation: [f64; 2]) {
    // График величин и зависимостей, изученных в этом уроке.
    let dense_representations_points: Vec<(f64, f64)> = dense_numeric_representation
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Плотное представление токена",
        // Указываем подпись горизонтальной оси.
        "измерение",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "эмбеддинг",
            // Передаём рассчитанные координаты точек.
            points: &dense_representations_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
