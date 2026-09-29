// Урок 03.5. Приближение производной функции центральной разностью.
//
// Что изучаем: Численная производная.
// Зачем это нужно: Центральная разность оценивает производную по значениям функции слева и справа от
// точки. Сравнение с точной формулой показывает погрешность.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Сохраняем рассчитанное значение `input_value` для следующих операций.
    let input_value: f64 = 3.0;
    lesson_trace::trace_step!(input_value);
    // h = 0.0001 достаточно мал для приближения производной и достаточно велик для f64.
    let step: f64 = 0.0001;
    lesson_trace::trace_step!(step);
    // Для f(x)=x² считаем значения в x+h и x−h.
    let right: f64 = (input_value + step) * (input_value + step);
    lesson_trace::trace_step!(right);
    // Умножаем значения и сохраняем результат в `left`.
    let left: f64 = (input_value - step) * (input_value - step);
    lesson_trace::trace_step!(left);
    // Делим разность f(x+h)−f(x−h) на расстояние между точками: (x+h)−(x−h)=2h.
    let numerical_derivative: f64 = (right - left) / (2.0 * step);
    lesson_trace::trace_step!(numerical_derivative);
    // Умножаем значения и сохраняем результат в `analytical_derivative`.
    let analytical_derivative: f64 = 2.0 * input_value;
    lesson_trace::trace_step!(analytical_derivative);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("численно={numerical_derivative}, точно={analytical_derivative}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_approximate_function_derivative_with_central_difference(
        input_value,
        analytical_derivative,
    );
}

// Строим график по результатам урока.
fn visualize_approximate_function_derivative_with_central_difference(
    horizontal_value: f64,
    analytical_derivative: f64,
) {
    // На малом шаге проявляется погрешность округления центральной разности.
    let points: Vec<(f64, f64)> = (1..=12)
        // Преобразуем каждый элемент в новое значение.
        .map(|step_exponent| {
            // Сохраняем результат этого шага в `step_size`.
            let step_size: f64 = 10f64.powi(-step_exponent);
            // Сохраняем результат этого шага в `numeric`.
            let numeric: f64 = ((horizontal_value + step_size) * (horizontal_value + step_size)
                - (horizontal_value - step_size) * (horizontal_value - step_size))
                / (2.0 * step_size);
            // Добавляем пару значений для сравнения или построения графика.
            (
                step_exponent as f64,
                (numeric - analytical_derivative).abs(),
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Ошибка центральной разности",
        // Указываем подпись горизонтальной оси.
        "k для h=10⁻ᵏ",
        // Указываем подпись вертикальной оси.
        "абсолютная ошибка",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "x² в x=3",
            // Передаём рассчитанные координаты точек.
            points: &points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
