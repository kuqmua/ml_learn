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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Сохраняем рассчитанное значение `image_width` для следующих операций.");
    let image_width: i32 = 5;
    trace_step!(image_width);
    trace_note!("Сохраняем рассчитанное значение `filter_width` для следующих операций.");
    trace_note!("Небольшой набор весов свёрточного фильтра называют kernel.");
    let filter_width: i32 = 2;
    trace_step!(filter_width);
    trace_note!("Сохраняем рассчитанное значение `filter_step_size` для следующих операций.");
    trace_note!("Шаг перемещения фильтра по входу называют stride.");
    let filter_step_size: i32 = 2;
    trace_step!(filter_step_size);
    trace_note!("Нормируем или усредняем величину делением и сохраняем её в `output_width`.");
    let output_width: i32 = (image_width - filter_width) / filter_step_size + 1;
    trace_step!(output_width);
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `positions`.");
    let positions: Vec<i32> = (0..output_width)
        .map(|index| index * filter_step_size)
        .collect();
    trace_step!(positions);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("позиции ядра по ширине: {positions:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_image_positions_visited_with_fixed_step_size(positions);
}

// Строим график по результатам урока.
fn plot_image_positions_visited_with_fixed_step_size(positions: std::vec::Vec<i32>) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
