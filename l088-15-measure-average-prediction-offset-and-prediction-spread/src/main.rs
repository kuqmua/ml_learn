// Урок 15.4. Измерение среднего смещения прогнозов и их разброса.
// Связь с принятой терминологией: Смещение и разброс прогнозов модели.
// Зачем здесь эта тема: Ансамбль полезен, когда ошибки отдельных моделей меняются от выборки к
//   выборке.
// Почему код устроен так: Сравниваем средний прогноз и его разброс, чтобы различить систематическую
//   ошибку и нестабильность.
// Представь: Если разные обучающие выборки дают сильно разные прогнозы одного объекта, у модели
//   высокий разброс.
//
// Что изучаем: Смещение и разброс.
// Зачем это нужно: Ошибка может происходить из систематического смещения модели или высокой изменчивости
// при разных обучающих наборах.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `predictions` для следующего шага примера.");
    let predictions: [f64; 3] = [2.0, 4.0, 6.0];
    lesson_trace::trace_step!(predictions);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !predictions.is_empty(),
        "для оценки разброса нужен хотя бы один прогноз"
    );
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `target` для следующих операций.");
    let target: f64 = 5.0;
    lesson_trace::trace_step!(target);
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `mean`."
    );
    let mean: f64 = predictions.iter().sum::<f64>() / predictions.len() as f64;
    lesson_trace::trace_step!(mean);
    lesson_trace::trace_note!("Комбинируем исходные величины и сохраняем результат в `bias`.");
    let bias: f64 = mean - target;
    lesson_trace::trace_step!(bias);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `variance` для следующих операций.");
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
    lesson_trace::trace_note!("Складываем результаты для всех элементов последовательности.");
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    let variance: f64 = predictions
        .iter()
        .map(|&input_value| (input_value - mean) * (input_value - mean))
        .sum::<f64>()
        / predictions.len() as f64;
    lesson_trace::trace_step!(variance);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("смещение={bias}, разброс={variance:.2}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_squared_average_error_and_prediction_spread(bias, variance);
}

// Строим график по результатам урока.
fn plot_squared_average_error_and_prediction_spread(bias: f64, variance: f64) {
    lesson_trace::trace_note!("Сравниваем компоненты ошибки на том же наборе прогнозов.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Смещение и разброс",
        "вклад в MSE",
        &[("смещение²", bias * bias), ("разброс", variance)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
