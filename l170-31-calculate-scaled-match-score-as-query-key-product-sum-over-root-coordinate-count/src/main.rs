// Урок 31.4. Масштабированная оценка совпадения: сумма произведений запроса и ключа, делённая на корень из числа координат.
// Связь с принятой терминологией: Масштабирование скалярного произведения векторов запроса и ключа.
// Зачем здесь эта тема: При большой размерности скалярные оценки Q·K растут и softmax может стать
//   слишком резким.
// Почему код устроен так: Делим оценку на корень из размерности ключа до нормировки.
// Представь: При длинных Q и K сумма произведений может стать большой; деление удерживает оценки в
//   более удобном масштабе.
//
// Что изучаем: Масштабирование скалярного произведения векторов запроса и ключа.
// Зачем это нужно: Оценку внимания делим на корень из размерности ключа, чтобы крупные векторы не делали
// softmax слишком резким.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `query` для следующего шага примера.");
    let query: [f64; 2] = [1.0, 1.0];
    lesson_trace::trace_step!(query);
    lesson_trace::trace_note!("Создаём набор значений `key` для следующего шага примера.");
    let key: [f64; 2] = [2.0, 2.0];
    lesson_trace::trace_step!(key);
    lesson_trace::trace_note!(
        "Умножаем соответствующие координаты запроса и ключа, затем складываем результаты."
    );
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем результат, ожидая успешного выполнения шага.");
    let sum_after_multiplying_coordinates: f64 =

        l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&query, &key)

            .expect("запрос и ключ имеют одинаковую размерность");
    lesson_trace::trace_step!(sum_after_multiplying_coordinates);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `dimension` для следующих операций."
    );
    let dimension: f64 = 2.0;
    lesson_trace::trace_step!(dimension);
    lesson_trace::trace_note!("Создаём изменяемое значение `scale` для следующих операций.");
    let mut scale: f64 = dimension;
    lesson_trace::trace_step!(scale);
    lesson_trace::trace_note!(
        "Для внимания нужен делитель √размерности; 80 шагов Ньютона дают его оценку в f64."
    );
    for _ in 0..80 {
        lesson_trace::trace_note!("Среднее scale и dimension/scale приближает √dimension.");
        scale = (scale + dimension / scale) / 2.0;
        lesson_trace::trace_step!(scale);
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
    println!(
        "Q·K={sum_after_multiplying_coordinates}, после масштабирования={}",
        sum_after_multiplying_coordinates / scale
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_attention_scale_as_one_divided_by_square_root_of_coordinate_count();
}

// Строим график по результатам урока.
fn plot_attention_scale_as_one_divided_by_square_root_of_coordinate_count() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let multiply_coordinates_add_and_scale_points: Vec<(f64, f64)> = (1..=64)
        .map(|vector_dimension| {
            (
                vector_dimension as f64,
                1.0 / (vector_dimension as f64).sqrt(),
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
        "Масштабирование скалярного произведения векторов запроса и ключа",
        "размерность d",
        "множитель 1/√d",
        &[lesson_visualization::Series {
            name: "масштаб",

            points: &multiply_coordinates_add_and_scale_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
