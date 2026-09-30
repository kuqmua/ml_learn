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
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Сохраняем рассчитанное значение `input_value` для следующих операций.");
    let input_value: f64 = 3.0;
    trace_step!(input_value);
    trace_note!(
        "h=10⁻⁴ в центральной разности: небольшой сдвиг для приближения без сильного округления f64."
    );
    let step_size: f64 = 0.0001;
    trace_step!(step_size);
    trace_note!("Умножаем значения и сохраняем результат в `analytical`.");
    let analytical: f64 = 2.0 * input_value;
    trace_step!(analytical);
    trace_note!("Умножаем значения и сохраняем результат в `right`.");
    let right: f64 = (input_value + step_size) * (input_value + step_size);
    trace_step!(right);
    trace_note!("Умножаем значения и сохраняем результат в `left`.");
    let left: f64 = (input_value - step_size) * (input_value - step_size);
    trace_step!(left);
    trace_note!("Нормируем или усредняем величину делением и сохраняем её в `numerical`.");
    let numerical: f64 = (right - left) / (2.0 * step_size);
    trace_step!(numerical);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("аналитически={analytical}, численно={numerical}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_difference_between_formula_slope_and_two_point_estimate();
}

// Строим график по результатам урока.
fn plot_difference_between_formula_slope_and_two_point_estimate() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Производную функции по параметру или вектор таких производных называют gradient.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let derivative_comparison_points: Vec<(f64, f64)> = (1..=100)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `step_size`.");
            let step_size: f64 = plot_step_index as f64 / 100.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (
                step_size,
                (((3.0 + step_size) * (3.0 + step_size) - 9.0) / step_size - 6.0).abs(),
            )
        })
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
        "Проверка градиента",
        "шаг h",
        "разность",
        &[lesson_visualization::Series {
            name: "x² в x=3",

            points: &derivative_comparison_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
