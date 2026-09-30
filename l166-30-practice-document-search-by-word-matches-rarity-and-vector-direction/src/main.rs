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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Учебные реализации математических операций для этого урока.");

    trace_note!("Фиксируем демонстрационные данные на время выполнения программы.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    const DOCUMENTS: [(&str, &str); 3] = [
        ("rust", "Rust управляет памятью без сборщика мусора"),
        ("ml", "Модель учится по обучающим данным"),
        ("math", "Матрица хранит числа в строках и столбцах"),
    ];

    trace_note!(
        "Объявляем повторно используемое вычисление `split_text_into_lowercase_words`; параметры ниже задают его входы."
    );
    fn split_text_into_lowercase_words(text: &str) -> Vec<String> {
        trace_note!("Приводим текст к нижнему регистру для одинакового сравнения слов.");
        trace_note!("Разделяем текст по пробельным символам на отдельные слова.");
        trace_note!("Преобразуем каждый элемент последовательности.");
        trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        text.to_lowercase()
            .split_whitespace()
            .map(str::to_string)
            .collect()
    }

    trace_note!(
        "Шаг: Токенизируем запрос, рассчитываем TF-IDF по документам и возвращаем top-k источников."
    );
    trace_note!("Передаём владение элементами итератору для дальнейшей обработки.");
    trace_note!("Оставляем только заданное число лучших элементов.");
    trace_note!("Собираем полученные элементы в вектор.");
    let highest_ranked_items: Vec<(&str, f64)> = (|| -> Vec<(&'static str, f64)> {
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Оцениваем совпадения слов запроса и документов с поправкой на частоту слова.");
        trace_note!("Сохраняем результат этого шага в `query`.");
        let query: &str = "модель данные";
        trace_step!(query);
        trace_note!("Один и тот же разбор текста используем для запроса и документов.");
        trace_note!("Единицу текста, которую модель обрабатывает как одно целое, называют token.");
        let query_text_units: Vec<String> = split_text_into_lowercase_words(query);
        trace_step!(query_text_units);
        trace_note!("Создаём набор значений `ranked_results` для следующего шага примера.");
        let mut ranked_results: Vec<(&str, f64)> = vec![];
        trace_step!(ranked_results);
        trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
        for &(document_identifier, document_text) in &DOCUMENTS {
            trace_step!(document_identifier);
            trace_step!(document_text);
            trace_note!("Сохраняем рассчитанное значение `document_text_units` для следующих операций.");
            let document_text_units: Vec<String> = split_text_into_lowercase_words(document_text);
            trace_step!(document_text_units);
            trace_note!("Частота слова внутри документа составляет компонент TF.");
            let mut text_unit_counts: std::collections::BTreeMap<&String, usize> =
                std::collections::BTreeMap::new();
            trace_step!(text_unit_counts);
            trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
            for text_unit in &document_text_units {
                trace_step!(text_unit);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                *text_unit_counts.entry(text_unit).or_insert(0usize) += 1;
                trace_step!(text_unit_counts);
            }
            trace_note!("Инициализируем изменяемый накопитель `relevance_score` начальным состоянием.");
            let mut relevance_score: f64 = 0.;
            trace_step!(relevance_score);
            trace_note!("Редкие во всём корпусе слова получают больший вес IDF.");
            trace_note!("Перебираем слова запроса без копирования строк.");
            trace_note!("Убираем повторы, чтобы слово запроса учитывалось один раз.");
            for word in query_text_units

                .iter()

                .collect::<std::collections::BTreeSet<_>>()
            {
                trace_step!(word);
                trace_note!("Сохраняем рассчитанное значение `document_frequency` для следующих операций.");
                trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
                trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
                trace_note!("Подсчитываем число элементов после отбора.");
                let document_frequency: usize = DOCUMENTS

                    .iter()

                    .filter(|(_, document)| {
                        trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
                        split_text_into_lowercase_words(document).contains(word)
                    })

                    .count();
                trace_step!(document_frequency);
                trace_note!("Выполняем встроенный расчёт один раз и сохраняем результат в `inverse_document_frequency`.");
                let inverse_document_frequency: f64 = (|| -> f64 {
                    trace_note!("Обновляем значение результатом текущего вычисления.");
                    trace_note!("ln(x) через ряд 2 * (t + t³/3 + t⁵/5 + ...), t=(x-1)/(x+1).");
                    trace_note!("Сохраняем результат этого шага в `value`.");
                    trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
                    let value: f64 =

                            (DOCUMENTS.len() + 1) as f64 / (document_frequency + 1) as f64;
                    trace_step!(value);
                    trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
                    trace_note!("Передаём очередное значение в составе результата или вызова.");
                    trace_note!("Подставляем результаты в этот шаблон вывода или текстового значения.");
                    assert!(

                        value > 0.0,

                        "логарифм определён только для положительных чисел"
                    );
                    trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
                    if value == f64::INFINITY {
                        trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
                        return f64::INFINITY;
                    }
                    trace_note!("Создаём изменяемое значение `scaled` для следующих операций.");
                    let mut scaled: f64 = value;
                    trace_step!(scaled);
                    trace_note!("Инициализируем изменяемый накопитель `power_of_two` начальным состоянием.");
                    let mut power_of_two: i32 = 0i32;
                    trace_step!(power_of_two);
                    trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
                    while scaled >= 2.0 {
                        trace_note!("Масштабируем текущую величину делением.");
                        scaled /= 2.0;
                        trace_step!(scaled);
                        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                        power_of_two += 1;
                        trace_step!(power_of_two);
                    }
                    trace_note!("Повторяем вычисление, пока выполняется указанное условие.");
                    while scaled < 1.0 {
                        trace_note!("Умножаем накопленное значение на очередной множитель.");
                        scaled *= 2.0;
                        trace_step!(scaled);
                        trace_note!("Вычитаем очередной вклад из текущего значения параметра.");
                        power_of_two -= 1;
                        trace_step!(power_of_two);
                    }
                    trace_note!("Этот ряд — учебное раскрытие `value.ln()`; он может работать медленнее и отличаться по точности.");
                    trace_note!("Объявляем повторно используемое вычисление `approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers`; параметры ниже задают его входы.");
                    /// Ряд для ln(x): 2·(t + t³/3 + t⁵/5 + …), где t = (x−1)/(x+1).
                    fn approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        value: f64,
                    ) -> f64 {
                        trace_note!("Нормируем или усредняем величину делением и сохраняем её в `ratio`.");
                        let ratio: f64 = (value - 1.0) / (value + 1.0);
                        trace_step!(ratio);
                        trace_note!("Умножаем значения и сохраняем результат в `ratio_squared`.");
                        let ratio_squared: f64 = ratio * ratio;
                        trace_step!(ratio_squared);
                        trace_note!("Создаём изменяемое значение `term` для следующих операций.");
                        let mut term: f64 = ratio;
                        trace_step!(term);
                        trace_note!("Инициализируем изменяемый накопитель `result` начальным состоянием.");
                        let mut result: f64 = 0.0;
                        trace_step!(result);
                        trace_note!("Используем 40 первых членов ряда ln(value) = 2·Σ ratio^(2k+1)/(2k+1).");
                        trace_note!("Это конечное приближение: для положительного value выполняется |ratio| < 1.");
                        for term_index in 0..40 {
                            trace_step!(term_index);
                            trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                            result += term / (2 * term_index + 1) as f64;
                            trace_step!(result);
                            trace_note!("Умножаем накопленное значение на очередной множитель.");
                            term *= ratio_squared;
                            trace_step!(term);
                        }
                        trace_note!("Умножаем величины согласно используемой формуле.");
                        2.0 * result
                    }
                    trace_note!("Сохраняем рассчитанное значение `logarithm_of_two` для следующих операций.");
                    let logarithm_of_two: f64 =
                        approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                            2.0,
                        );
                    trace_step!(logarithm_of_two);
                    trace_note!("Умножаем величины согласно используемой формуле.");
                    trace_note!("Вычисляем значение по указанной формуле.");
                    approximate_natural_log_as_twice_sum_of_odd_ratio_powers_over_odd_numbers(
                        scaled,
                    ) + power_of_two as f64 * logarithm_of_two

                })() + 1.;
                trace_step!(inverse_document_frequency);
                trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
                trace_note!("Делим значения, получая нормированную величину или среднее.");
                trace_note!("Добавляем этот член в составное арифметическое выражение.");
                relevance_score += *text_unit_counts.get(word).unwrap_or(&0) as f64

                        / document_text_units.len() as f64

                        * inverse_document_frequency;
                trace_step!(relevance_score);
            }
            trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
            if relevance_score > 0. {
                trace_note!("Сохраняем очередной рассчитанный элемент в коллекции.");
                ranked_results.push((document_identifier, relevance_score));
            }
        }
        trace_note!("Самые релевантные документы ставим первыми.");
        trace_note!("Упорядочиваем данные для следующего шага алгоритма.");
        ranked_results

            .sort_by(|first_result, second_result| second_result.1.total_cmp(&first_result.1));
        trace_note!("Используем ранее рассчитанное значение `ranked_results` в текущем выражении.");
        ranked_results
    })()

    .into_iter()

    .take(2)

    .collect::<Vec<_>>();
    trace_step!(highest_ranked_items);
    trace_note!("Печатаем рассчитанные значения для проверки примера.");
    println!("top-k: {highest_ranked_items:?}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_document_scores_from_word_frequency_and_rarity(highest_ranked_items);
}

// Строим график по результатам урока.
fn plot_document_scores_from_word_frequency_and_rarity(
    highest_ranked_items: std::vec::Vec<(&str, f64)>,
) {
    trace_note!("Собираем значения для `chart_values` в коллекцию.");
    let chart_values: Vec<(&str, f64)> = highest_ranked_items
        .iter()
        .map(|(document_identifier, score)| (*document_identifier, *score))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Оценки найденных документов",
        "TF-IDF",
        &chart_values,
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
