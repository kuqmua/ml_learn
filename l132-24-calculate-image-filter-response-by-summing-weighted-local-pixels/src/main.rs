// Урок 24.1. Отклик фильтра изображения: сложение пикселей участка, умноженных на веса.
// Связь с принятой терминологией: Применение ядра свёртки к локальному участку изображения.
// Зачем здесь эта тема: Изображение имеет локальную структуру; фильтр ищет один и тот же шаблон в
//   разных местах.
// Почему код устроен так: Сначала считаем произведения ядра и одного окна пикселей по координатам.
// Представь: Ядро 2×2 смотрит только на четыре пикселя одного окна, умножает их на четыре веса и
//   складывает.
//
// Что изучаем: Ядро свёртки.
// Зачем это нужно: Малое ядро умножает локальный участок изображения на веса и суммирует отклики.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `patch` для следующего шага примера.");
    let patch: [[f64; 2]; 2] = [[1.0, 2.0], [3.0, 4.0]];
    lesson_trace::trace_step!(patch);
    lesson_trace::trace_note!(
        "Создаём набор значений `filter_weights` для следующего шага примера."
    );
    lesson_trace::trace_note!("Небольшой набор весов свёрточного фильтра называют kernel.");
    let filter_weights: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, -1.0]];
    lesson_trace::trace_step!(filter_weights);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `response` начальным состоянием."
    );
    let mut response: f64 = 0.0;
    lesson_trace::trace_step!(response);
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
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            response += patch[row][column] * filter_weights[row][column];
            lesson_trace::trace_step!(response);
        }
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("отклик ядра = {response}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_local_pixel_values_and_weighted_sum(patch, response);
}

// Строим график по результатам урока.
fn plot_local_pixel_values_and_weighted_sum(patch: [[f64; 2]; 2], response: f64) {
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
        "Ядро свёртки и отклик",
        "значение",
        &[
            ("патч 0,0", patch[0][0]),
            ("патч 1,1", patch[1][1]),
            ("отклик", response),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
