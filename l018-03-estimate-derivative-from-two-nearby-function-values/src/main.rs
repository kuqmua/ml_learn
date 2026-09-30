// Урок 03.5. Приближённая производная: оценка скорости изменения по значениям слева и справа от точки.
// Связь с принятой терминологией: Приближение производной функции центральной разностью.
// Зачем здесь эта тема: Аналитическую производную легко вывести с ошибкой; центральная разность
//   даёт независимую численную проверку.
// Почему код устроен так: Считаем функцию по обе стороны точки: симметрия уменьшает ошибку по
//   сравнению с односторонней разностью.
// Представь: Чтобы проверить наклон в точке x, сравниваем значения функции чуть левее и чуть правее
//   x.
//
// Что изучаем: Численная производная.
// Зачем это нужно: Центральная разность оценивает производную по значениям функции слева и справа от
// точки. Сравнение с точной формулой показывает погрешность.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `input_value` для следующих операций."
    );
    let input_value: f64 = 3.0;
    lesson_trace::trace_step!(input_value);
    lesson_trace::trace_note!(
        "h = 0.0001 достаточно мал для приближения производной и достаточно велик для f64."
    );
    let step: f64 = 0.0001;
    lesson_trace::trace_step!(step);
    lesson_trace::trace_note!("Для f(x)=x² считаем значения в x+h и x−h.");
    let right: f64 = (input_value + step) * (input_value + step);
    lesson_trace::trace_step!(right);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `left`.");
    let left: f64 = (input_value - step) * (input_value - step);
    lesson_trace::trace_step!(left);
    lesson_trace::trace_note!(
        "Делим разность f(x+h)−f(x−h) на расстояние между точками: (x+h)−(x−h)=2h."
    );
    let numerical_derivative: f64 = (right - left) / (2.0 * step);
    lesson_trace::trace_step!(numerical_derivative);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `analytical_derivative`.");
    let analytical_derivative: f64 = 2.0 * input_value;
    lesson_trace::trace_step!(analytical_derivative);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("численно={numerical_derivative}, точно={analytical_derivative}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_slope_estimation_error_for_shrinking_step(input_value, analytical_derivative);
}

// Строим график по результатам урока.
fn plot_slope_estimation_error_for_shrinking_step(
    horizontal_value: f64,
    analytical_derivative: f64,
) {
    lesson_trace::trace_note!(
        "На малом шаге проявляется погрешность округления центральной разности."
    );
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let points: Vec<(f64, f64)> = (1..=12)
        .map(|step_exponent| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `step_size`.");
            let step_size: f64 = 10f64.powi(-step_exponent);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `numeric`.");
            let numeric: f64 = ((horizontal_value + step_size) * (horizontal_value + step_size)
                - (horizontal_value - step_size) * (horizontal_value - step_size))
                / (2.0 * step_size);
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                step_exponent as f64,
                (numeric - analytical_derivative).abs(),
            )
        })
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
        "Ошибка центральной разности",
        "k для h=10⁻ᵏ",
        "абсолютная ошибка",
        &[lesson_visualization::Series {
            name: "x² в x=3",

            points: &points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
