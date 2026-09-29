// Урок 30.5. Практика: поиск документов по совпадениям слов, их редкости и направлениям векторов.
// Связь с принятой терминологией: Поиск документов с лексическим поиском, TF-IDF и косинусным ранжированием.
// Зачем здесь эта тема: Поиск соединяет токенизацию, вес слов, нормировку и выбор лучших
//   документов.
// Почему код устроен так: Проводим один запрос через весь конвейер и показываем вклад каждой стадии
//   в порядок результатов.
// Представь: Запрос сначала превращается в слова, затем в веса, затем получает оценку для каждого
//   документа.
//
// Что повторяем вместе: лексический поиск, TF-IDF, косинусное сходство, top-k.
// Зачем это нужно: Поиск сопоставляет запрос с документами и учитывает не только частоту слова, но и его
//   редкость в коллекции.
// Что показывает программа: Токенизируем запрос, рассчитываем TF-IDF по документам и возвращаем top-k
//   источников.
// Что проверить при изменении примера: Проверь запрос без совпадений, повторные слова и отсутствие
//   документов из test в индексе оценки.
// Дополнительная практика: Индексируй локальный набор документов и возвращай top-k с оценками и источниками.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Учебные реализации математических операций для этого урока.

    // Фиксируем демонстрационные данные на время выполнения программы.
    const DOCUMENTS: [(&str, &str); 3] = [
        // Составляем результат из вычисленных значений в указанном порядке.
        ("rust", "Rust управляет памятью без сборщика мусора"),
        // Составляем результат из вычисленных значений в указанном порядке.
        ("ml", "Модель учится по обучающим данным"),
        // Составляем результат из вычисленных значений в указанном порядке.
        ("math", "Матрица хранит числа в строках и столбцах"),
    ];

    // Объявляем повторно используемое вычисление `split_text_into_lowercase_words`; параметры ниже задают его входы.
    fn split_text_into_lowercase_words(text: &str) -> Vec<String> {
        // Приводим текст к нижнему регистру для одинакового сравнения слов.
        text.to_lowercase()
            // Разделяем текст по пробельным символам на отдельные слова.
            .split_whitespace()
            // Преобразуем каждый элемент последовательности.
            .map(str::to_string)
            // Собираем элементы итератора в итоговую коллекцию.
            .collect()
    }

    // Шаг: Токенизируем запрос, рассчитываем TF-IDF по документам и возвращаем top-k источников.
    let highest_ranked_items: Vec<(&str, f64)> = (|| -> Vec<(&'static str, f64)> {
        // Используем подготовленное значение в следующем шаге примера.
        /* Оцениваем совпадения слов запроса и документов с поправкой на частоту слова. */
        // Сохраняем результат этого шага в `query`.
        let query: &str = "модель данные";
        lesson_trace::trace_step!(query);
        // Один и тот же разбор текста используем для запроса и документов.
        // Единицу текста, которую модель обрабатывает как одно целое, называют token.
        let query_text_units: Vec<String> = split_text_into_lowercase_words(query);
        lesson_trace::trace_step!(query_text_units);
        // Создаём набор значений `ranked_results` для следующего шага примера.
        let mut ranked_results: Vec<(&str, f64)> = vec![];
        lesson_trace::trace_step!(ranked_results);
        // Повторяем следующий блок для каждого элемента указанной последовательности.
        for &(document_identifier, document_text) in &DOCUMENTS {
            lesson_trace::trace_step!(document_identifier);
            lesson_trace::trace_step!(document_text);
            // Сохраняем рассчитанное значение `document_text_units` для следующих операций.
            let document_text_units: Vec<String> = split_text_into_lowercase_words(document_text);
            lesson_trace::trace_step!(document_text_units);
            // Частота слова внутри документа составляет компонент TF.
            let mut text_unit_counts: std::collections::BTreeMap<&String, usize> =
                std::collections::BTreeMap::new();
            lesson_trace::trace_step!(text_unit_counts);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for text_unit in &document_text_units {
                lesson_trace::trace_step!(text_unit);
                // Прибавляем очередной вклад к ранее накопленному результату.
                *text_unit_counts.entry(text_unit).or_insert(0usize) += 1;
                lesson_trace::trace_step!(text_unit_counts);
            }
            // Инициализируем изменяемый накопитель `relevance_score` начальным состоянием.
            let mut relevance_score: f64 = 0.;
            lesson_trace::trace_step!(relevance_score);
            // Редкие во всём корпусе слова получают больший вес IDF.
            for word in query_text_units
                // Перебираем слова запроса без копирования строк.
                .iter()
                // Убираем повторы, чтобы слово запроса учитывалось один раз.
                .collect::<std::collections::BTreeSet<_>>()
            {
                lesson_trace::trace_step!(word);
                // Сохраняем рассчитанное значение `document_frequency` для следующих операций.
                let document_frequency: usize = DOCUMENTS
                    // Перебираем элементы по ссылке, не копируя исходную коллекцию.
                    .iter()
                    // Оставляем только элементы, прошедшие указанную проверку.
                    .filter(|(_, document)| {
                        // Вызываем нужное вычисление с подготовленными аргументами.
                        split_text_into_lowercase_words(document).contains(word)
                    })
                    // Подсчитываем число элементов после отбора.
                    .count();
                lesson_trace::trace_step!(document_frequency);
                // Выполняем встроенный расчёт один раз и сохраняем результат в `inverse_document_frequency`.
                let inverse_document_frequency: f64 = (|| -> f64 {
                    // Обновляем значение результатом текущего вычисления.
                    /* ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1). */
                    // Сохраняем результат этого шага в `value`.
                    let value: f64 =
                            // Составляем результат из вычисленных значений в указанном порядке.
                            (DOCUMENTS.len() + 1) as f64 / (document_frequency + 1) as f64;
                    lesson_trace::trace_step!(value);
                    // Проверяем обязательное условие до дальнейшего вычисления.
                    assert!(
                        // Передаём очередное значение в составе результата или вызова.
                        value > 0.0,
                        // Подставляем результаты в этот шаблон вывода или текстового значения.
                        "логарифм определён только для положительных чисел"
                    );
                    // Проверяем условие и выбираем соответствующую ветку алгоритма.
                    if value == f64::INFINITY {
                        // Завершаем текущий расчёт и возвращаем найденное значение.
                        return f64::INFINITY;
                    }
                    // Создаём изменяемое значение `scaled` для следующих операций.
                    let mut scaled: f64 = value;
                    lesson_trace::trace_step!(scaled);
                    // Инициализируем изменяемый накопитель `power_of_two` начальным состоянием.
                    let mut power_of_two: i32 = 0i32;
                    lesson_trace::trace_step!(power_of_two);
                    // Повторяем вычисление, пока выполняется указанное условие.
                    while scaled >= 2.0 {
                        // Масштабируем текущую величину делением.
                        scaled /= 2.0;
                        lesson_trace::trace_step!(scaled);
                        // Прибавляем очередной вклад к ранее накопленному результату.
                        power_of_two += 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    // Повторяем вычисление, пока выполняется указанное условие.
                    while scaled < 1.0 {
                        // Умножаем накопленное значение на очередной множитель.
                        scaled *= 2.0;
                        lesson_trace::trace_step!(scaled);
                        // Вычитаем очередной вклад из текущего значения параметра.
                        power_of_two -= 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    // Этот ряд — учебное раскрытие `value.ln()`; он может работать медленнее и отличаться по точности.
                    // Объявляем повторно используемое вычисление `twice_sum_odd_powers_of_ratio_over_odd_numbers`; параметры ниже задают его входы.
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn twice_sum_odd_powers_of_ratio_over_odd_numbers(value: f64) -> f64 {
                        // Нормируем или усредняем величину делением и сохраняем её в `ratio`.
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        lesson_trace::trace_step!(ratio);
                        // Умножаем значения и сохраняем результат в `ratio_squared`.
                        let ratio_squared: f64 = ratio * ratio;
                        lesson_trace::trace_step!(ratio_squared);
                        // Создаём изменяемое значение `term` для следующих операций.
                        let mut term: f64 = ratio;
                        lesson_trace::trace_step!(term);
                        // Инициализируем изменяемый накопитель `result` начальным состоянием.
                        let mut result: f64 = 0.0;
                        lesson_trace::trace_step!(result);
                        // Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1).
                        // Это конечное приближение: для положительного value выполняется |ratio| < 1.
                        for term_index in 0..40 {
                            lesson_trace::trace_step!(term_index);
                            // Прибавляем очередной вклад к ранее накопленному результату.
                            result += term / (2 * term_index + 1) as f64;
                            lesson_trace::trace_step!(result);
                            // Умножаем накопленное значение на очередной множитель.
                            term *= ratio_squared;
                            lesson_trace::trace_step!(term);
                        }
                        // Умножаем величины согласно используемой формуле.
                        2.0 * result
                    }
                    // Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций.
                    let logarithm_of_two: f64 = twice_sum_odd_powers_of_ratio_over_odd_numbers(2.0);
                    lesson_trace::trace_step!(logarithm_of_two);
                    // Умножаем величины согласно используемой формуле.
                    twice_sum_odd_powers_of_ratio_over_odd_numbers(scaled)
                        + power_of_two as f64 * logarithm_of_two
                    // Вычисляем значение по указанной формуле.
                })() + 1.;
                lesson_trace::trace_step!(inverse_document_frequency);
                // Прибавляем очередной вклад к ранее накопленному результату.
                relevance_score += *text_unit_counts.get(word).unwrap_or(&0) as f64
                        // Делим значения, получая нормированную величину или среднее.
                        / document_text_units.len() as f64
                        // Добавляем этот член в составное арифметическое выражение.
                        * inverse_document_frequency;
                lesson_trace::trace_step!(relevance_score);
            }
            // Проверяем условие и выбираем соответствующую ветку алгоритма.
            if relevance_score > 0. {
                // Сохраняем очередной рассчитанный элемент в коллекции.
                ranked_results.push((document_identifier, relevance_score));
            }
        }
        // Самые релевантные документы ставим первыми.
        ranked_results
            // Упорядочиваем данные для следующего шага алгоритма.
            .sort_by(|first_result, second_result| second_result.1.total_cmp(&first_result.1));
        // Используем ранее рассчитанное значение `ranked_results` в текущем выражении.
        ranked_results
    })()
    // Передаём владение элементами итератору для дальнейшей обработки.
    .into_iter()
    // Оставляем только заданное число лучших элементов.
    .take(2)
    // Собираем полученные элементы в вектор.
    .collect::<Vec<_>>();
    lesson_trace::trace_step!(highest_ranked_items);
    // Печатаем рассчитанные значения для проверки примера.
    println!("top-k: {highest_ranked_items:?}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_document_scores_from_word_frequency_and_rarity(highest_ranked_items);
}

// Строим график по результатам урока.
fn plot_document_scores_from_word_frequency_and_rarity(
    highest_ranked_items: std::vec::Vec<(&str, f64)>,
) {
    // Собираем значения для `chart_values` в коллекцию.
    let chart_values: Vec<(&str, f64)> = highest_ranked_items
        .iter()
        .map(|(document_identifier, score)| (*document_identifier, *score))
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Оценки найденных документов",
        // Указываем подпись вертикальной оси.
        "TF-IDF",
        // Используем подготовленное значение в следующем шаге примера.
        &chart_values,
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
