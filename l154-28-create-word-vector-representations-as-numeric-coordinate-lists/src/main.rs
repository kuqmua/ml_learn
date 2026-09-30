// Урок 28.3. Векторное представление слова: список числовых координат.
// Связь с принятой терминологией: Плотные векторные представления слов.
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
    lesson_trace::trace_note!(
        "Создаём набор значений `dense_representation_table` для следующего шага примера."
    );
    lesson_trace::trace_note!("Плотное числовое представление объекта называют embedding.");
    let dense_representation_table: [[f64; 2]; 3] = [[0.0, 0.0], [0.8, 0.2], [0.7, 0.3]];
    lesson_trace::trace_step!(dense_representation_table);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `text_unit_identifier` для следующих операций."
    );
    lesson_trace::trace_note!(
        "Единицу текста, которую модель обрабатывает как одно целое, называют token."
    );
    let text_unit_identifier: usize = 2;
    lesson_trace::trace_step!(text_unit_identifier);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `dense_numeric_representation` для следующих операций."
    );
    let dense_numeric_representation: [f64; 2] = dense_representation_table[text_unit_identifier];
    lesson_trace::trace_step!(dense_numeric_representation);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("токен={text_unit_identifier}, плотный вектор={dense_numeric_representation:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_numeric_coordinates_representing_one_text_unit(dense_numeric_representation);
}

// Строим график по результатам урока.
fn plot_numeric_coordinates_representing_one_text_unit(dense_numeric_representation: [f64; 2]) {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let dense_representations_points: Vec<(f64, f64)> = dense_numeric_representation
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
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
        "Плотное представление токена",
        "измерение",
        "значение",
        &[lesson_visualization::Series {
            name: "эмбеддинг",

            points: &dense_representations_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
