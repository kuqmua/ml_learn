// Урок 21.3. Активация ReLU: замена отрицательных выходов слоя нулями.
// Связь с принятой терминологией: Нелинейная активация в слое нейронной сети.
// Зачем здесь эта тема: Цепочка только линейных слоёв сводится к одному линейному преобразованию.
// Почему код устроен так: Добавляем нелинейность между слоями и сравниваем выход с линейным
//   случаем.
// Представь: Без активации два последовательных линейных слоя можно заменить одним; нелинейность
//   меняет возможности сети.
//
// Что изучаем: Функция активации.
// Зачем это нужно: Нелинейная активация позволяет сети описывать зависимости, которые не выражаются одной
// прямой.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    trace_note!("Оценку модели до преобразования в вероятность называют logit.");
    for raw_model_score in [-2.0, 0.0, 2.0] {
        trace_step!(raw_model_score);
        trace_note!("ReLU оставляет положительные значения и обнуляет отрицательные.");
        let rectified_linear_output: f64 = if raw_model_score > 0.0 {
            raw_model_score
        } else {
            0.0
        };
        trace_step!(rectified_linear_output);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        println!("logit={raw_model_score}, ReLU={rectified_linear_output}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_rectified_activation_as_input_with_negative_values_replaced_by_zero();
}

// Строим график по результатам урока.
fn plot_rectified_activation_as_input_with_negative_values_replaced_by_zero() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let activation_points: Vec<(f64, f64)> = (-50..=50)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (horizontal_value, horizontal_value.max(0.0))
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
        "ReLU",
        "вход",
        "выход",
        &[lesson_visualization::Series {
            name: "max(0,x)",

            points: &activation_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
