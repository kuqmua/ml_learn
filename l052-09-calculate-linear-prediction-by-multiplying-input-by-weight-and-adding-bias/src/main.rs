// Урок 09.3. Линейный прогноз: умножение входного значения на вес и прибавление смещения.
// Связь с принятой терминологией: Вес и смещение линейной регрессионной модели.
// Зачем здесь эта тема: После метрики нужна модель, параметры которой можно подбирать для
//   уменьшения ошибки.
// Почему код устроен так: Считаем линейный прогноз как сумму признаков с весами и отдельным
//   смещением.
// Представь: При формуле y=2x+1 число 2 — вес признака, а 1 — смещение прогноза.
//
// Что изучаем: Коэффициенты линейной модели.
// Зачем это нужно: Вес задаёт изменение прогноза при увеличении признака на единицу, а смещение задаёт
// прогноз при нулевом признаке.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `weight` для следующих операций.");
    let weight: f64 = 2.0;
    lesson_trace::trace_step!(weight);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `bias` для следующих операций.");
    let bias: f64 = 1.0;
    lesson_trace::trace_step!(bias);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for feature in [0.0, 1.0, 3.0] {
        lesson_trace::trace_step!(feature);
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `prediction`.");
        let prediction: f64 = weight * feature + bias;
        lesson_trace::trace_step!(prediction);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("x={feature} -> y={prediction}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_linear_prediction_as_weighted_input_plus_bias();
}

// Строим график по результатам урока.
fn plot_linear_prediction_as_weighted_input_plus_bias() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let coefficients_points: Vec<(f64, f64)> = (0..=50)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (horizontal_value, 2.0 * horizontal_value + 1.0)
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
        "Линейная модель",
        "признак x",
        "предсказание",
        &[lesson_visualization::Series {
            name: "y=2x+1",

            points: &coefficients_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
