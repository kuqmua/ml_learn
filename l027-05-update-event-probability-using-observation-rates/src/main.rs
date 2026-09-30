// Урок 05.3. Пересчёт вероятности события с учётом частоты полученного наблюдения.
// Связь с принятой терминологией: Пересчёт вероятности события после наблюдения по формуле Байеса.
// Зачем здесь эта тема: Диагностический сигнал меняет исходную вероятность события; формула Байеса
//   соединяет prior и качество сигнала.
// Почему код устроен так: Разделяем истинные и ложные сигналы, чтобы увидеть влияние редкости
//   события.
// Представь: Даже хороший тест даёт много ложных тревог, когда проверяемое событие очень редкое.
//
// Что изучаем: Формула Байеса.
// Зачем это нужно: Формула пересчитывает вероятность причины после наблюдения результата. Редкая болезнь
// остаётся редкой даже после несовершенного теста.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Инициализируем значение `prevalence` начальным состоянием.");
    let prevalence: f64 = 0.01;
    lesson_trace::trace_step!(prevalence);
    lesson_trace::trace_note!("Инициализируем значение `sensitivity` начальным состоянием.");
    let sensitivity: f64 = 0.90;
    lesson_trace::trace_step!(sensitivity);
    lesson_trace::trace_note!("Инициализируем значение `specificity` начальным состоянием.");
    let specificity: f64 = 0.95;
    lesson_trace::trace_step!(specificity);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `true_positive`.");
    let true_positive: f64 = prevalence * sensitivity;
    lesson_trace::trace_step!(true_positive);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `false_positive`.");
    let false_positive: f64 = (1.0 - prevalence) * (1.0 - specificity);
    lesson_trace::trace_step!(false_positive);
    lesson_trace::trace_note!(
        "Нормируем или усредняем величину делением и сохраняем её в `posterior`."
    );
    let posterior: f64 = true_positive / (true_positive + false_positive);
    lesson_trace::trace_step!(posterior);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("P(болен | положительный тест) = {posterior:.3}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_disease_probability_before_and_after_positive_test(posterior);
}

// Строим график по результатам урока.
fn plot_disease_probability_before_and_after_positive_test(posterior: f64) {
    lesson_trace::trace_note!("Сравнение величин из этого урока.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Байес: до и после теста",
        "вероятность",
        &[("до теста", 0.01), ("после теста", posterior)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
