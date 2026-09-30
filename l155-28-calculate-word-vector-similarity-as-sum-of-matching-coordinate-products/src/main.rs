// Урок 28.4. Сходство векторов слов: сумма произведений соответствующих координат.
// Связь с принятой терминологией: Скалярное произведение двух эмбеддингов слов как мера сходства.
// Зачем здесь эта тема: После появления эмбеддингов можно сравнивать токены по направлению и
//   величине векторов.
// Почему код устроен так: Считаем скалярное произведение уже знакомым способом на маленькой
//   таблице.
// Представь: Похожие по направлению векторы могут иметь большое скалярное произведение, но его
//   величина зависит и от длин.
//
// Что изучаем: Сходство эмбеддингов.
// Зачем это нужно: Результат умножения пар координат и сложения двух представлений измеряет близость их
// направлений.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `first` для следующего шага примера.");
    let first: [f64; 2] = [0.8, 0.2];
    trace_step!(first);
    trace_note!("Создаём набор значений `second` для следующего шага примера.");
    let second: [f64; 2] = [0.7, 0.3];
    trace_step!(second);
    trace_note!("Умножаем значения и сохраняем результат в `sum_after_multiplying_coordinates`.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Используем результат, ожидая успешного выполнения шага.");
    let sum_after_multiplying_coordinates: f64 =
        calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&first, &second)
            .expect("представления имеют одинаковую размерность");
    trace_step!(sum_after_multiplying_coordinates);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    println!(
        "сумма после попарного умножения координат представлений = {sum_after_multiplying_coordinates}"
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_coordinates_of_two_word_representations(first, second);
}

// Строим график по результатам урока.
fn plot_coordinates_of_two_word_representations(first: [f64; 2], second: [f64; 2]) {
    trace_note!("Значения из этого урока на графике.");
    trace_note!("Плотное числовое представление объекта называют embedding.");
    let first_dense_representation_points: Vec<(f64, f64)> = vec![(first[0], first[1])];
    trace_note!("Собираем значения для `second_dense_representation_points` в коллекцию.");
    let second_dense_representation_points: Vec<(f64, f64)> = vec![(second[0], second[1])];
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Похожие эмбеддинги",
        "первая координата",
        "вторая координата",
        &[
            lesson_visualization::Series {
                name: "первый",

                points: &first_dense_representation_points,
            },
            lesson_visualization::Series {
                name: "второй",

                points: &second_dense_representation_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
