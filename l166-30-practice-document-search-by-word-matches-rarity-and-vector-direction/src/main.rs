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
    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    lesson_trace::trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    const DOCUMENTS: [(&str, &str); 3] = [
        ("rust", "Rust управляет памятью без сборщика мусора"),
        ("ml", "Модель учится по обучающим данным"),
        ("math", "Матрица хранит числа в строках и столбцах"),
    ];

    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `split_text_into_lowercase_words`; параметры ниже задают его входы."
    );
    fn split_text_into_lowercase_words(text: &str) -> Vec<String> {
        lesson_trace::trace_note!(
            "Приводим текст к нижнему регистру для одинакового сравнения слов."
        );
        lesson_trace::trace_note!("Разделяем текст по пробельным символам на отдельные слова.");
        lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        text.to_lowercase()
            .split_whitespace()
            .map(str::to_string)
            .collect()
    }

    lesson_trace::trace_note!(
        "Шаг: Токенизируем запрос, рассчитываем TF-IDF по документам и возвращаем top-k источников."
    );
    lesson_trace::trace_note!("Передаём владение элементами итератору для дальнейшей обработки.");
    lesson_trace::trace_note!("Оставляем только заданное число лучших элементов.");
    lesson_trace::trace_note!("Собираем полученные элементы в вектор.");
    let highest_ranked_items: Vec<(&str, f64)> = (|| -> Vec<(&'static str, f64)> {
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Оцениваем совпадения слов запроса и документов с поправкой на частоту слова.");
        lesson_trace::trace_note!("Сохраняем результат этого шага в `query`.");
        let query: &str = "модель данные";
        lesson_trace::trace_step!(query);
        lesson_trace::trace_note!("Один и тот же разбор текста используем для запроса и документов.");
        lesson_trace::trace_note!("Единицу текста, которую модель обрабатывает как одно целое, называют token.");
        let query_text_units: Vec<String> = split_text_into_lowercase_words(query);
        lesson_trace::trace_step!(query_text_units);
        lesson_trace::trace_note!("Создаём набор значений `ranked_results` для следующего шага примера.");
        let mut ranked_results: Vec<(&str, f64)> = vec![];
        lesson_trace::trace_step!(ranked_results);
        lesson_trace::trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for &(document_identifier, document_text) in &DOCUMENTS {
            lesson_trace::trace_step!(document_identifier);
            lesson_trace::trace_step!(document_text);
            lesson_trace::trace_note!("Сохраняем рассчитанное значение `document_text_units` для следующих операций.");
            let document_text_units: Vec<String> = split_text_into_lowercase_words(document_text);
            lesson_trace::trace_step!(document_text_units);
            lesson_trace::trace_note!("Частота слова внутри документа составляет компонент TF.");
            let mut text_unit_counts: std::collections::BTreeMap<&String, usize> =
                std::collections::BTreeMap::new();
            lesson_trace::trace_step!(text_unit_counts);
            lesson_trace::trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
            for text_unit in &document_text_units {
                lesson_trace::trace_step!(text_unit);
                lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                *text_unit_counts.entry(text_unit).or_insert(0usize) += 1;
                lesson_trace::trace_step!(text_unit_counts);
            }
            lesson_trace::trace_note!("Инициализируем изменяемый накопитель `relevance_score` начальным состоянием.");
            let mut relevance_score: f64 = 0.;
            lesson_trace::trace_step!(relevance_score);
            lesson_trace::trace_note!("Редкие во всём корпусе слова получают больший вес IDF.");
            lesson_trace::trace_note!("Перебираем слова запроса без копирования строк.");
            lesson_trace::trace_note!("Убираем повторы, чтобы слово запроса учитывалось один раз.");
            for word in query_text_units

                .iter()

                .collect::<std::collections::BTreeSet<_>>()
            {
                lesson_trace::trace_step!(word);
                lesson_trace::trace_note!("Сохраняем рассчитанное значение `document_frequency` для следующих операций.");
                lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
                lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
                lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
                let document_frequency: usize = DOCUMENTS

                    .iter()

                    .filter(|(_, document)| {
                        lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
                        split_text_into_lowercase_words(document).contains(word)
                    })

                    .count();
                lesson_trace::trace_step!(document_frequency);
                lesson_trace::trace_note!("Выполняем встроенный расчёт один раз и сохраняем результат в `inverse_document_frequency`.");
                let inverse_document_frequency: f64 = (|| -> f64 {
                    lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                    lesson_trace::trace_note!("ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1).");
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
                    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                    let value: f64 =

                            (DOCUMENTS.len() + 1) as f64 / (document_frequency + 1) as f64;
                    lesson_trace::trace_step!(value);
                    lesson_trace::trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
                    lesson_trace::trace_note!("Передаём очередное значение в составе результата или вызова.");
                    lesson_trace::trace_note!("Подставляем результаты в этот шаблон вывода или текстового значения.");
                    assert!(

                        value > 0.0,

                        "логарифм определён только для положительных чисел"
                    );
                    lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                    if value == f64::INFINITY {
                        lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
                        return f64::INFINITY;
                    }
                    lesson_trace::trace_note!("Создаём изменяемое значение `scaled` для следующих операций.");
                    let mut scaled: f64 = value;
                    lesson_trace::trace_step!(scaled);
                    lesson_trace::trace_note!("Инициализируем изменяемый накопитель `power_of_two` начальным состоянием.");
                    let mut power_of_two: i32 = 0i32;
                    lesson_trace::trace_step!(power_of_two);
                    lesson_trace::trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
                    while scaled >= 2.0 {
                        lesson_trace::trace_note!("Масштабируем текущую величину делением.");
                        scaled /= 2.0;
                        lesson_trace::trace_step!(scaled);
                        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                        power_of_two += 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    lesson_trace::trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
                    while scaled < 1.0 {
                        lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
                        scaled *= 2.0;
                        lesson_trace::trace_step!(scaled);
                        lesson_trace::trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
                        power_of_two -= 1;
                        lesson_trace::trace_step!(power_of_two);
                    }
                    lesson_trace::trace_note!("Этот ряд — учебное раскрытие `value.ln()`; он может работать медленнее и отличаться по точности.");
                    lesson_trace::trace_note!("Объявляем повторно используемое вычисление `approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers`; параметры ниже задают его входы.");
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        value: f64,
                    ) -> f64 {
                        lesson_trace::trace_note!("Нормируем или усредняем величину делением и сохраняем её в `ratio`.");
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        lesson_trace::trace_step!(ratio);
                        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `ratio_squared`.");
                        let ratio_squared: f64 = ratio * ratio;
                        lesson_trace::trace_step!(ratio_squared);
                        lesson_trace::trace_note!("Создаём изменяемое значение `term` для следующих операций.");
                        let mut term: f64 = ratio;
                        lesson_trace::trace_step!(term);
                        lesson_trace::trace_note!("Инициализируем изменяемый накопитель `result` начальным состоянием.");
                        let mut result: f64 = 0.0;
                        lesson_trace::trace_step!(result);
                        lesson_trace::trace_note!("Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1).");
                        lesson_trace::trace_note!("Это конечное приближение: для положительного value выполняется |ratio| < 1.");
                        for term_index in 0..40 {
                            lesson_trace::trace_step!(term_index);
                            lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                            result += term / (2 * term_index + 1) as f64;
                            lesson_trace::trace_step!(result);
                            lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
                            term *= ratio_squared;
                            lesson_trace::trace_step!(term);
                        }
                        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
                        2.0 * result
                    }
                    lesson_trace::trace_note!("Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций.");
                    let logarithm_of_two: f64 =
                        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        );
                    lesson_trace::trace_step!(logarithm_of_two);
                    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
                    lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64 * logarithm_of_two

                })() + 1.;
                lesson_trace::trace_step!(inverse_document_frequency);
                lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
                lesson_trace::trace_note!("Добавляем этот член в составное арифметическое выражение.");
                relevance_score += *text_unit_counts.get(word).unwrap_or(&0) as f64

                        / document_text_units.len() as f64

                        * inverse_document_frequency;
                lesson_trace::trace_step!(relevance_score);
            }
            lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
            if relevance_score > 0. {
                lesson_trace::trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
                ranked_results.push((document_identifier, relevance_score));
            }
        }
        lesson_trace::trace_note!("Самые релевантные документы ставим первыми.");
        lesson_trace::trace_note!("Упорядочиваем данные для следующего шага алгоритма.");
        ranked_results

            .sort_by(|first_result, second_result| second_result.1.total_cmp(&first_result.1));
        lesson_trace::trace_note!("Используем ранее рассчитанное значение `ranked_results` в текущем выражении.");
        ranked_results
    })()

    .into_iter()

    .take(2)

    .collect::<Vec<_>>();
    lesson_trace::trace_step!(highest_ranked_items);
    lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("top-k: {highest_ranked_items:?}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_document_scores_from_word_frequency_and_rarity(highest_ranked_items);
}

// Строим график по результатам урока.
fn plot_document_scores_from_word_frequency_and_rarity(
    highest_ranked_items: std::vec::Vec<(&str, f64)>,
) {
    lesson_trace::trace_note!("Собираем значения для `chart_values` в коллекцию.");
    let chart_values: Vec<(&str, f64)> = highest_ranked_items
        .iter()
        .map(|(document_identifier, score)| (*document_identifier, *score))
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Оценки найденных документов",
        "TF-IDF",
        &chart_values,
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
