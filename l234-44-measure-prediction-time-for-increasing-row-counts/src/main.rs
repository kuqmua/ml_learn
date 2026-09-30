// Урок 44.4. Измерение времени получения прогнозов при увеличении числа строк.
// Связь с принятой терминологией: Измерение времени инференса при росте числа строк.
// Зачем здесь эта тема: Модель может быть точной, но слишком медленной при большом пакете.
// Почему код устроен так: Замеряем один пакет из 1000 строк; график зависимости от размера здесь
//   схематический, не серия измерений.
// Представь: Обработка 1000 строк занимает время; для надёжного сравнения разных размеров
//   потребуются повторные замеры.
//
// Что изучаем: Производительность инференса.
// Зачем это нужно: Стоимость обработки растёт с числом строк; измеряем время пакетного прохода отдельно от
// загрузки входа.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `features` для следующего шага примера.");
    let features: Vec<f64> = vec![1.0; 1000];
    lesson_trace::trace_step!(features);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `start` для следующих операций.");
    let start: std::time::Instant = std::time::Instant::now();
    lesson_trace::trace_step!(start);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `predictions` для следующих операций."
    );
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
    lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
    let predictions: Vec<f64> = features
        .iter()
        .map(|&feature| 2.0 * feature + 1.0)
        .collect();
    lesson_trace::trace_step!(predictions);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    lesson_trace::trace_note!(
        "Присваиваем вычисленное значение соответствующей переменной или полю."
    );
    lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
    lesson_trace::trace_note!("Замер завершается после обработки всей партии объектов.");
    println!(
        "объектов={}, время={:?}",
        predictions.len(),
        start.elapsed()
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_operation_count_for_growing_number_of_input_rows();
}

// Строим график по результатам урока.
fn plot_operation_count_for_growing_number_of_input_rows() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let performance_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `sample_count`.");
            let sample_count: f64 = (plot_step_index * 10) as f64;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (sample_count, 2.0 * sample_count)
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
        "Стоимость пакетного инференса",
        "число строк",
        "условное число операций",
        &[lesson_visualization::Series {
            name: "линейный проход",

            points: &performance_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
