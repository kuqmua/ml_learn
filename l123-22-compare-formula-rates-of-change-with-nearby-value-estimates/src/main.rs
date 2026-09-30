// Урок 22.4. Сравнение скоростей изменения по формуле с оценками по соседним значениям.
// Связь с принятой терминологией: Сравнение аналитического градиента с центральной разностью.
// Зачем здесь эта тема: Сложный обратный проход легко реализовать с неверным знаком или формой.
// Почему код устроен так: Сравниваем аналитический градиент с центральной разностью на тех же
//   параметрах.
// Представь: Если формула дала градиент 6, а малое изменение параметра показывает около −6, знак в
//   формуле неверен.
//
// Что изучаем: Проверка градиента.
// Зачем это нужно: Сравниваем аналитическую производную с центральной численной разностью на малом
// примере.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `input_value` для следующих операций."
    );
    let input_value: f64 = 3.0;
    lesson_trace::trace_step!(input_value);
    lesson_trace::trace_note!(
        "h=10⁻⁴ в центральной разности: небольшой сдвиг для приближения без сильного округления f64."
    );
    let step_size: f64 = 0.0001;
    lesson_trace::trace_step!(step_size);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `analytical`.");
    let analytical: f64 = 2.0 * input_value;
    lesson_trace::trace_step!(analytical);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `right`.");
    let right: f64 = (input_value + step_size) * (input_value + step_size);
    lesson_trace::trace_step!(right);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `left`.");
    let left: f64 = (input_value - step_size) * (input_value - step_size);
    lesson_trace::trace_step!(left);
    lesson_trace::trace_note!(
        "Нормируем или усредняем величину делением и сохраняем её в `numerical`."
    );
    let numerical: f64 = (right - left) / (2.0 * step_size);
    lesson_trace::trace_step!(numerical);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("аналитически={analytical}, численно={numerical}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_difference_between_formula_slope_and_two_point_estimate();
}

// Строим график по результатам урока.
fn plot_difference_between_formula_slope_and_two_point_estimate() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let derivative_comparison_points: Vec<(f64, f64)> = (1..=100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `step_size`.");
            let step_size: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (
                step_size,
                (((3.0 + step_size) * (3.0 + step_size) - 9.0) / step_size - 6.0).abs(),
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
        "Проверка градиента",
        "шаг h",
        "разность",
        &[lesson_visualization::Series {
            name: "x² в x=3",

            points: &derivative_comparison_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
