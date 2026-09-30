// Урок 32.3. Нормализация координат: вычитание среднего и деление на корень из среднего квадрата отклонений.
// Связь с принятой терминологией: Нормализация координат одного токена в слое LayerNorm.
// Зачем здесь эта тема: Величины координат токена могут меняться между слоями и мешать устойчивому
//   обучению.
// Почему код устроен так: Вычитаем среднее и делим на масштаб внутри одного токена, добавляя
//   epsilon.
// Представь: Нормируем координаты внутри одного токена, чтобы следующий слой не зависел от случайно
//   большого масштаба.
//
// Что изучаем: Layer normalization.
// Зачем это нужно: Нормализуем координаты одного токена по его среднему и дисперсии, затем добавляем малое
// epsilon для устойчивости.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `text_unit` для следующего шага примера.");
    lesson_trace::trace_note!(
        "Единицу текста, которую модель обрабатывает как одно целое, называют token."
    );
    let text_unit: [f64; 2] = [1.0, 3.0];
    lesson_trace::trace_step!(text_unit);
    lesson_trace::trace_note!("Нормируем или усредняем величину делением и сохраняем её в `mean`.");
    let mean: f64 = (text_unit[0] + text_unit[1]) / 2.0;
    lesson_trace::trace_step!(mean);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `variance` для следующих операций.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    let variance: f64 = ((text_unit[0] - mean) * (text_unit[0] - mean)
        + (text_unit[1] - mean) * (text_unit[1] - mean))
        / 2.0;
    lesson_trace::trace_step!(variance);
    lesson_trace::trace_note!("ε = 0.00001 не даёт делить на ноль, если обе координаты одинаковы.");
    lesson_trace::trace_note!(
        "Добавка мала по сравнению с обычной дисперсией, но влияет на почти постоянный токен."
    );
    let squared_scale: f64 = variance + 0.00001;
    lesson_trace::trace_step!(squared_scale);
    lesson_trace::trace_note!("Создаём изменяемое значение `scale` для следующих операций.");
    let mut scale: f64 = squared_scale;
    lesson_trace::trace_step!(scale);
    lesson_trace::trace_note!(
        "80 шагов Ньютона превращают дисперсию с добавкой ε в делитель √(variance + ε)."
    );
    for _ in 0..80 {
        lesson_trace::trace_note!("Среднее scale и squared_scale/scale приближает нужный корень.");
        scale = (scale + squared_scale / scale) / 2.0;
        lesson_trace::trace_step!(scale);
    }
    lesson_trace::trace_note!("Создаём набор значений `normalized` для следующего шага примера.");
    let normalized: [f64; 2] = [(text_unit[0] - mean) / scale, (text_unit[1] - mean) / scale];
    lesson_trace::trace_step!(normalized);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("нормализованный токен = {normalized:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_normalized_coordinates_after_subtracting_mean_and_dividing_by_spread(normalized);
}

// Строим график по результатам урока.
fn plot_normalized_coordinates_after_subtracting_mean_and_dividing_by_spread(normalized: [f64; 2]) {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let layer_norm_points: Vec<(f64, f64)> = normalized
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
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
        "Layer normalization",
        "измерение",
        "значение",
        &[lesson_visualization::Series {
            name: "нормализованный токен",

            points: &layer_norm_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
