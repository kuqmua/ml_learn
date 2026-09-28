// Урок 28.2. TF-IDF.
//
// Что изучаем: TF-IDF.
// Зачем это нужно: Оценка повышает вес слова, частого в документе, и понижает вес слова, встречающегося во
// многих документах.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Сохраняем рассчитанное значение `term_frequency` для следующих операций.
    let term_frequency = 3.0;
    // Сохраняем рассчитанное значение `document_count` для следующих операций.
    let document_count = 10.0;
    // Сохраняем рассчитанное значение `documents_with_term` для следующих операций.
    let documents_with_term = 2.0;
    // Нормируем или усредняем величину делением и сохраняем её в `rarity_ratio`.
    let rarity_ratio = (document_count + 1.0) / (documents_with_term + 1.0);
    // IDF — логарифм отношения; вычисляем его рядом для положительного аргумента.
    let normalized = (rarity_ratio - 1.0) / (rarity_ratio + 1.0);
    // Создаём изменяемое значение `term` для следующих операций.
    let mut term = normalized;
    // Инициализируем изменяемый накопитель `logarithm` начальным состоянием.
    let mut logarithm = 0.0;
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for odd in (1..=99).step_by(2) {
        // Прибавляем очередной вклад к ранее накопленному результату.
        logarithm += term / odd as f64;
        // Умножаем накопленное значение на очередной множитель.
        term *= normalized * normalized;
    }
    // Умножаем значения и сохраняем результат в `inverse_document_frequency`.
    let inverse_document_frequency = 2.0 * logarithm;
    // Умножаем значения и сохраняем результат в `score`.
    let score = term_frequency * inverse_document_frequency;
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("частота={term_frequency}, IDF={inverse_document_frequency:.3}, TF-IDF={score:.3}");
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (0..=10)
        .map(|i| {
            let tf = i as f64 / 10.0;
            (tf, tf * inverse_document_frequency)
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "TF-IDF",
        "частота токена",
        "оценка TF-IDF",
        &[lesson_visualization::Series {
            name: "idf из примера",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
