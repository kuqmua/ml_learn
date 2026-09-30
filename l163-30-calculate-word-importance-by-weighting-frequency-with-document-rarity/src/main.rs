// Урок 30.2. Значимость слова для поиска: умножение частоты на меру редкости среди документов.
// Связь с принятой терминологией: Взвешивание слов документа методом TF-IDF.
// Зачем здесь эта тема: Частое слово во всех документах хуже различает темы, чем редкое слово.
// Почему код устроен так: Умножаем частоту слова в документе на вес обратной частоты по корпусу.
// Представь: Слово, встречающееся во всех документах, помогает поиску меньше, чем редкое
//   характерное слово.
//
// Что изучаем: TF-IDF.
// Зачем это нужно: Оценка повышает вес слова, частого в документе, и понижает вес слова, встречающегося во
// многих документах.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Сохраняем рассчитанное значение `term_frequency` для следующих операций.");
    let term_frequency: f64 = 3.0;
    trace_step!(term_frequency);
    trace_note!("Сохраняем рассчитанное значение `document_count` для следующих операций.");
    let document_count: f64 = 10.0;
    trace_step!(document_count);
    trace_note!("Сохраняем рассчитанное значение `documents_with_term` для следующих операций.");
    let documents_with_term: f64 = 2.0;
    trace_step!(documents_with_term);
    trace_note!("Нормируем или усредняем величину делением и сохраняем её в `rarity_ratio`.");
    let rarity_ratio: f64 = (document_count + 1.0) / (documents_with_term + 1.0);
    trace_step!(rarity_ratio);
    trace_note!("IDF — логарифм отношения; вычисляем его рядом для положительного аргумента.");
    let normalized: f64 = (rarity_ratio - 1.0) / (rarity_ratio + 1.0);
    trace_step!(normalized);
    trace_note!("Создаём изменяемое значение `term` для следующих операций.");
    let mut term: f64 = normalized;
    trace_step!(term);
    trace_note!("Инициализируем изменяемый накопитель `logarithm` начальным состоянием.");
    let mut logarithm: f64 = 0.0;
    trace_step!(logarithm);
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    for odd_divisor in (1..=99).step_by(2) {
        trace_step!(odd_divisor);
        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        logarithm += term / odd_divisor as f64;
        trace_step!(logarithm);
        trace_note!("Умножаем накопленное значение на очередной множитель.");
        term *= normalized * normalized;
        trace_step!(term);
    }
    trace_note!("Умножаем значения и сохраняем результат в `inverse_document_frequency`.");
    let inverse_document_frequency: f64 = 2.0 * logarithm;
    trace_step!(inverse_document_frequency);
    trace_note!("Умножаем значения и сохраняем результат в `score`.");
    let score: f64 = term_frequency * inverse_document_frequency;
    trace_step!(score);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("частота={term_frequency}, IDF={inverse_document_frequency:.3}, TF-IDF={score:.3}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_word_frequency_weighted_by_rarity_across_documents(inverse_document_frequency);
}

// Строим график по результатам урока.
fn plot_word_frequency_weighted_by_rarity_across_documents(inverse_document_frequency: f64) {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let term_frequency_inverse_document_frequency_points: Vec<(f64, f64)> = (0..=10)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `term_frequency`.");
            let term_frequency: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (term_frequency, term_frequency * inverse_document_frequency)
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "TF-IDF",
        "частота токена",
        "оценка TF-IDF",
        &[lesson_visualization::Series {
            name: "idf из примера",

            points: &term_frequency_inverse_document_frequency_points,
        }],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
