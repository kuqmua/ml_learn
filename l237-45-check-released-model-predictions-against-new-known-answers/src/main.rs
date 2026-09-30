// Урок 45.2. Проверка прогнозов выпущенной модели по новым известным ответам.
// Связь с принятой терминологией: Оценка качества модели после выпуска по новым истинным меткам.
// Зачем здесь эта тема: Сдвиг признаков не всегда означает падение качества; нужны новые истинные
//   метки.
// Почему код устроен так: Считаем прежнюю метрику на новых размеченных данных и сравниваем с
//   исходным уровнем.
// Представь: После выпуска появились новые правильные ответы; по ним можно посчитать свежую
//   accuracy.
//
// Что изучаем: Качество после релиза.
// Зачем это нужно: Когда появляются истинные метки, пересчитываем метрику на новых данных отдельно от
// учебной оценки.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `truth` для следующего шага примера.");
    let truth: [bool; 3] = [true, false, true];
    lesson_trace::trace_step!(truth);
    lesson_trace::trace_note!("Создаём набор значений `predicted` для следующего шага примера.");
    let predicted: [bool; 3] = [true, true, true];
    lesson_trace::trace_step!(predicted);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert_eq!(
        truth.len(),
        predicted.len(),
        "число прогнозов должно совпадать с числом ответов"
    );
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !truth.is_empty(),
        "для accuracy нужна хотя бы одна пара значений"
    );
    lesson_trace::trace_note!("Считаем количество элементов и сохраняем его в `correct`.");
    lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
    lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
    let correct: usize = (0..truth.len())
        .filter(|&index| truth[index] == predicted[index])
        .count();
    lesson_trace::trace_step!(correct);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    lesson_trace::trace_note!(
        "Присваиваем вычисленное значение соответствующей переменной или полю."
    );
    lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
    println!(
        "accuracy новых данных={:.2}",
        correct as f64 / truth.len() as f64
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_counts_of_correct_and_incorrect_predictions_after_release(truth, correct);
}

// Строим график по результатам урока.
fn plot_counts_of_correct_and_incorrect_predictions_after_release(
    truth: [bool; 3],
    correct: usize,
) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Качество после релиза",
        "число объектов",
        &[
            ("верно", correct as f64),
            ("ошибка", (truth.len() - correct) as f64),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
