// Сводная практика 28. Словарь токенов и плотные эмбеддинги слов.
// Почему этот урок сейчас: Словарь, особые токены и эмбеддинги должны согласованно преобразовывать текст во вход модели.
// Почему пример устроен так: Проводим пример от строки до id и плотных векторов, проверяя неизвестные слова.
//
// Что повторяем вместе: словарь, специальные токены, плотные представления, сходство.
// Зачем это нужно: Токенизация превращает слова в индексы, по которым можно выбирать обучаемые векторы
//   представления.
// Что показывает программа: Создаём корпус из двух коротких предложений. Назначаем индекс каждому слову и
//   резервируем индекс для неизвестных слов. Создаём таблицу векторов и читаем строки по индексам токенов.
// Что проверить при изменении примера: Проверь OOV-токен, сохранение/загрузку словаря и одинаковую
//   индексацию в train/inference.
// Дополнительная практика: Построй токенизатор по словам и обучаемую таблицу эмбеддингов для малого корпуса.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Шаг: Создаём корпус из двух коротких предложений.
    let corpus: [&str; 2] = ["кот спит", "пёс спит"];
    lesson_trace::trace_step!(corpus);

    // Шаг: Назначаем индекс каждому слову и резервируем индекс для неизвестных слов.
    // Набор известных модели текстовых единиц называют vocabulary.
    let known_text_units: std::collections::BTreeMap<String, usize> =
        (|| -> std::collections::BTreeMap<String, usize> {
            // Используем подготовленное значение в следующем шаге примера.
            /* Нумеруем слова корпуса; нулевой индекс оставляем неизвестному токену. */
            // Сохраняем результат этого шага в `corpus`.
            let corpus: &[&str] = &corpus;
            lesson_trace::trace_step!(corpus);
            // Инициализируем изменяемый накопитель `known_text_units` начальным состоянием.
            let mut known_text_units: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            lesson_trace::trace_step!(known_text_units);
            // Выполняем очередное действие, после которого продолжаем следующий шаг.
            known_text_units.insert("<unk>".into(), 0);
            // Повторяем следующий блок для каждого элемента указанной последовательности.
            for word in corpus
                // Перебираем элементы по ссылке, не копируя исходную коллекцию.
                .iter()
                // Разделяем текст по пробельным символам на отдельные слова.
                .flat_map(|sentence| sentence.split_whitespace())
            {
                lesson_trace::trace_step!(word);
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if !known_text_units.contains_key(word) {
                    // Считаем количество элементов и сохраняем его в `text_unit_identifier`.
                    // Единицу текста, которую модель обрабатывает как одно целое, называют token.
                    let text_unit_identifier: usize = known_text_units.len();
                    lesson_trace::trace_step!(text_unit_identifier);
                    // Выполняем очередное действие, после которого продолжаем следующий шаг.
                    known_text_units.insert(word.into(), text_unit_identifier);
                }
            }
            // Используем ранее рассчитанное значение `known_text_units` в текущем выражении.
            known_text_units
        })();
    lesson_trace::trace_step!(known_text_units);

    // Шаг: Создаём таблицу векторов и читаем строки по индексам токенов.
    // Плотное числовое представление объекта называют embedding.
    let mut dense_numeric_representations: Vec<[f64; 2]> = vec![[0., 0.]; known_text_units.len()];
    lesson_trace::trace_step!(dense_numeric_representations);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for (text_unit_index, row) in dense_numeric_representations.iter_mut().enumerate() {
        lesson_trace::trace_step!(text_unit_index);
        lesson_trace::trace_step!(row);
        // Передаём данные по ссылке или разыменовываем их для следующей операции.
        *row = [text_unit_index as f64 * 0.1, text_unit_index as f64 * 0.2];
        lesson_trace::trace_step!(row);
    }
    // Выполняем встроенный расчёт один раз и сохраняем результат в `text_unit_indices`.
    let text_unit_indices: Vec<usize> = (|| -> Vec<usize> {
        // Используем подготовленное значение в следующем шаге примера.
        /* Каждое слово превращаем в индекс словаря, неизвестным словам даём нулевой индекс. */
        // Сохраняем результат этого шага в `text`.
        let text: &str = "кот неизвестно";
        lesson_trace::trace_step!(text);
        // Сохраняем рассчитанное значение `known_text_units` для следующих операций.
        let known_text_units: &std::collections::BTreeMap<String, usize> = &known_text_units;
        lesson_trace::trace_step!(known_text_units);
        // Разделяем текст по пробельным символам на отдельные слова.
        text.split_whitespace()
            // Преобразуем каждый элемент последовательности.
            .map(|word| *known_text_units.get(word).unwrap_or(&0))
            // Собираем элементы итератора в итоговую коллекцию.
            .collect()
    })();
    lesson_trace::trace_step!(text_unit_indices);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        // Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.
        "vocab={known_text_units:?}, ids={text_unit_indices:?}, vectors={:?}",
        // Используем ранее рассчитанное значение `text_unit_indices` в текущем выражении.
        text_unit_indices
            // Перебираем элементы по ссылке, не копируя исходную коллекцию.
            .iter()
            // Преобразуем каждый элемент последовательности.
            .map(|&token_index| dense_numeric_representations[token_index])
            // Собираем полученные элементы в вектор.
            .collect::<Vec<_>>()
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_token_vocabulary_and_dense_word_embeddings(
        known_text_units,
        text_unit_indices,
    );
}

// Строим график по результатам урока.
fn visualize_practice_token_vocabulary_and_dense_word_embeddings(
    known_text_units: std::collections::BTreeMap<std::string::String, usize>,
    text_unit_indices: std::vec::Vec<usize>,
) {
    // Наглядное сравнение результатов сводной практики.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Размер словаря и последовательности",
        // Указываем подпись вертикальной оси.
        "число элементов",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("словарь", known_text_units.len() as f64),
            // Добавляем пару значений для сравнения или построения графика.
            ("токены", text_unit_indices.len() as f64),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
