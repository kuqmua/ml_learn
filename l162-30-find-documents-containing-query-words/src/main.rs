// Урок 30.1. Поиск документов, содержащих слова запроса.
// Связь с принятой терминологией: Совпадение слов запроса и документов при лексическом поиске.
// Зачем здесь эта тема: Перед сложным поиском полезно увидеть, что даёт точное совпадение слов
//   запроса и документа.
// Почему код устроен так: Считаем общие слова на малых текстах и видим ограничение буквального
//   поиска.
// Представь: Запрос «рыжий кот» найдёт документ с теми же словами, но может пропустить текст с
//   синонимом.
//
// Что изучаем: Лексический поиск.
// Зачем это нужно: Совпадение слов запроса с документом даёт простой поиск без обученной модели.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `documents` для следующего шага примера.");
    let documents: [(&str, &str); 2] = [("rust cargo", "guide"), ("машинное обучение", "ml")];
    lesson_trace::trace_step!(documents);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `query` для следующих операций.");
    let query: &str = "cargo";
    lesson_trace::trace_step!(query);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for (text, document_identifier) in documents {
        lesson_trace::trace_step!(text);
        lesson_trace::trace_step!(document_identifier);
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if text.split_whitespace().any(|word| word == query) {
            lesson_trace::trace_note!(
                "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
            );
            println!("найден документ {document_identifier}");
        }
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_number_of_documents_with_and_without_query_matches(documents, query);
}

// Строим график по результатам урока.
fn plot_number_of_documents_with_and_without_query_matches(
    documents: [(&str, &str); 2],
    query: &str,
) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Оставляем элементы, отвечающие условию.");
    lesson_trace::trace_note!("Подсчитываем число подходящих элементов.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Оставляем элементы, отвечающие условию.");
    lesson_trace::trace_note!("Подсчитываем число подходящих элементов.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Лексический поиск",
        "документов",
        &[
            (
                "найдены",
                documents
                    .iter()
                    .filter(|(text, _)| text.contains(query))
                    .count() as f64,
            ),
            (
                "не найдены",
                documents
                    .iter()
                    .filter(|(text, _)| !text.contains(query))
                    .count() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
