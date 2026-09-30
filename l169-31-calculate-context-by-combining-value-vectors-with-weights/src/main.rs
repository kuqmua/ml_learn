// Урок 31.3. Контекст позиции: объединение векторов значений с заданными весами.
// Связь с принятой терминологией: Вектор значения V для позиции механизма внимания.
// Зачем здесь эта тема: Высокая оценка совпадения указывает куда смотреть, но ещё не содержит
//   передаваемой информации.
// Почему код устроен так: Храним отдельный V, который попадёт во взвешенную сумму после расчёта
//   весов.
// Представь: Оценка говорит «куда смотреть», а V содержит числа, которые оттуда забираем.
//
// Что изучаем: Вектор значения V.
// Зачем это нужно: Value содержит информацию, которая будет перенесена в выход внимания после взвешивания.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `values` для следующего шага примера.");
    let values: [[f64; 2]; 2] = [[1.0, 0.0], [0.0, 2.0]];
    lesson_trace::trace_step!(values);
    lesson_trace::trace_note!("Создаём набор значений `weights` для следующего шага примера.");
    let weights: [f64; 2] = [0.25, 0.75];
    lesson_trace::trace_step!(weights);
    lesson_trace::trace_note!("Создаём набор значений `output` для следующего шага примера.");
    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
    let output: [f64; 2] = [
        weights[0] * values[0][0] + weights[1] * values[1][0],
        weights[0] * values[0][1] + weights[1] * values[1][1],
    ];
    lesson_trace::trace_step!(output);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("взвешенное значение = {output:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_value_coordinates_after_weighted_summing(output);
}

// Строим график по результатам урока.
fn plot_value_coordinates_after_weighted_summing(output: [f64; 2]) {
    lesson_trace::trace_note!("Сравнение величин из этого урока.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Взвешенная сумма V",
        "компонента",
        &[("выход 0", output[0]), ("выход 1", output[1])],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
