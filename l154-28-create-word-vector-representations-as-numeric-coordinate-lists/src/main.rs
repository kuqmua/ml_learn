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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Создаём набор значений `dense_representation_table` для следующего шага примера.");
    trace_note!("Плотное числовое представление объекта называют embedding.");
    let dense_representation_table: [[f64; 2]; 3] = [[0.0, 0.0], [0.8, 0.2], [0.7, 0.3]];
    trace_step!(dense_representation_table);
    trace_note!("Сохраняем рассчитанное значение `text_unit_identifier` для следующих операций.");
    trace_note!("Единицу текста, которую модель обрабатывает как одно целое, называют token.");
    let text_unit_identifier: usize = 2;
    trace_step!(text_unit_identifier);
    trace_note!(
        "Сохраняем рассчитанное значение `dense_numeric_representation` для следующих операций."
    );
    let dense_numeric_representation: [f64; 2] = dense_representation_table[text_unit_identifier];
    trace_step!(dense_numeric_representation);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("токен={text_unit_identifier}, плотный вектор={dense_numeric_representation:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_numeric_coordinates_representing_one_text_unit(dense_numeric_representation);
}

// Строим график по результатам урока.
fn plot_numeric_coordinates_representing_one_text_unit(dense_numeric_representation: [f64; 2]) {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let dense_representations_points: Vec<(f64, f64)> = dense_numeric_representation
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
