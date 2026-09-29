// Урок 30.2. Взвешивание слов документа методом TF-IDF.
// Почему этот урок сейчас: Частое слово во всех документах хуже различает темы, чем редкое слово.
// Почему пример устроен так: Умножаем частоту слова в документе на вес обратной частоты по корпусу.
//
// Что изучаем: TF-IDF.
// Зачем это нужно: Оценка повышает вес слова, частого в документе, и понижает вес слова, встречающегося во
// многих документах.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Сохраняем рассчитанное значение `term_frequency` для следующих операций.
    let term_frequency: f64 = 3.0;
    lesson_trace::trace_step!(term_frequency);
    // Сохраняем рассчитанное значение `document_count` для следующих операций.
    let document_count: f64 = 10.0;
    lesson_trace::trace_step!(document_count);
    // Сохраняем рассчитанное значение `documents_with_term` для следующих операций.
    let documents_with_term: f64 = 2.0;
    lesson_trace::trace_step!(documents_with_term);
    // Нормируем или усредняем величину делением и сохраняем её в `rarity_ratio`.
    let rarity_ratio: f64 = (document_count + 1.0) / (documents_with_term + 1.0);
    lesson_trace::trace_step!(rarity_ratio);
    // IDF — логарифм отношения; вычисляем его рядом для положительного аргумента.
    let normalized: f64 = (rarity_ratio - 1.0) / (rarity_ratio + 1.0);
    lesson_trace::trace_step!(normalized);
    // Создаём изменяемое значение `term` для следующих операций.
    let mut term: f64 = normalized;
    lesson_trace::trace_step!(term);
    // Инициализируем изменяемый накопитель `logarithm` начальным состоянием.
    let mut logarithm: f64 = 0.0;
    lesson_trace::trace_step!(logarithm);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for odd_divisor in (1..=99).step_by(2) {
        lesson_trace::trace_step!(odd_divisor);
        // Прибавляем очередной вклад к ранее накопленному результату.
        logarithm += term / odd_divisor as f64;
        lesson_trace::trace_step!(logarithm);
        // Умножаем накопленное значение на очередной множитель.
        term *= normalized * normalized;
        lesson_trace::trace_step!(term);
    }
    // Умножаем значения и сохраняем результат в `inverse_document_frequency`.
    let inverse_document_frequency: f64 = 2.0 * logarithm;
    lesson_trace::trace_step!(inverse_document_frequency);
    // Умножаем значения и сохраняем результат в `score`.
    let score: f64 = term_frequency * inverse_document_frequency;
    lesson_trace::trace_step!(score);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("частота={term_frequency}, IDF={inverse_document_frequency:.3}, TF-IDF={score:.3}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_weight_document_terms_by_tf_idf(inverse_document_frequency);
}

// Строим график по результатам урока.
fn visualize_weight_document_terms_by_tf_idf(inverse_document_frequency: f64) {
    // График величин и зависимостей, изученных в этом уроке.
    let term_frequency_inverse_document_frequency_points: Vec<(f64, f64)> = (0..=10)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `term_frequency`.
            let term_frequency: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (term_frequency, term_frequency * inverse_document_frequency)
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "TF-IDF",
        // Указываем подпись горизонтальной оси.
        "частота токена",
        // Указываем подпись вертикальной оси.
        "оценка TF-IDF",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "idf из примера",
            // Передаём рассчитанные координаты точек.
            points: &term_frequency_inverse_document_frequency_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
