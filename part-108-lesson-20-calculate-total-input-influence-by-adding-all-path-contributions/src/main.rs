// Урок 20.3. Полное влияние входа: сложение вкладов всех путей к результату.
// Связь с принятой терминологией: Сложение вкладов градиента из нескольких путей вычислительного графа.
// Зачем здесь эта тема: Один параметр может влиять на результат несколькими путями графа.
// Почему код устроен так: Складываем вклады всех путей, иначе производная общего входа будет
//   неполной.
// Представь: Если x используется в двух ветвях, обе ветви влияют на производную по x.
//
// Что изучаем: Накопление градиентов.
// Зачем это нужно: Если один узел участвует в нескольких путях, градиенты этих путей складываются.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Сохраняем рассчитанное значение `input_value` для следующих операций.
    let input_value: f64 = 3.0;
    lesson_trace::trace_step!(input_value);
    // f(x)=x*x: вход x участвует как левый и правый множитель.
    // Производную функции по параметру или вектор таких производных называют gradient.
    let left_path_rate_of_change: f64 = input_value;
    lesson_trace::trace_step!(left_path_rate_of_change);
    // Сохраняем рассчитанное значение `right_path_rate_of_change` для следующих операций.
    let right_path_rate_of_change: f64 = input_value;
    lesson_trace::trace_step!(right_path_rate_of_change);
    // Комбинируем исходные величины и сохраняем результат в `combined_rate_of_change`.
    let combined_rate_of_change: f64 = left_path_rate_of_change + right_path_rate_of_change;
    lesson_trace::trace_step!(combined_rate_of_change);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        // Присваиваем вычисленное значение соответствующей переменной или полю.
        "градиент слева={left_path_rate_of_change}, справа={right_path_rate_of_change}, всего={combined_rate_of_change}"
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_contributions_to_input_rate_of_change_from_each_path(
        left_path_rate_of_change,
        right_path_rate_of_change,
        combined_rate_of_change,
    );
}

// Строим график по результатам урока.
fn plot_contributions_to_input_rate_of_change_from_each_path(
    left_path_rate_of_change: f64,
    right_path_rate_of_change: f64,
    combined_rate_of_change: f64,
) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Накопление градиентов",
        // Указываем подпись вертикальной оси.
        "вклад",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("левый путь", left_path_rate_of_change),
            // Добавляем пару значений для сравнения или построения графика.
            ("правый путь", right_path_rate_of_change),
            // Добавляем пару значений для сравнения или построения графика.
            ("всего", combined_rate_of_change),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
