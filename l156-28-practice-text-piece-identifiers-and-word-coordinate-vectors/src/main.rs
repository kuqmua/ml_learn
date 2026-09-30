// Урок 28.5. Практика: номера частей текста и векторы координат слов.
// Связь с принятой терминологией: Словарь токенов и плотные эмбеддинги слов.
// Зачем здесь эта тема: Словарь, особые токены и эмбеддинги должны согласованно преобразовывать
//   текст во вход модели.
// Почему код устроен так: Проводим пример от строки до id и плотных векторов, проверяя неизвестные
//   слова.
// Представь: Строка превращается в токены, затем в id и только после этого в числа, которые
//   получает модель.
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
    lesson_trace::trace_note!("Шаг: Создаём корпус из двух коротких предложений.");
    let corpus: [&str; 2] = ["кот спит", "пёс спит"];
    lesson_trace::trace_step!(corpus);

    lesson_trace::trace_note!(
        "Шаг: Назначаем индекс каждому слову и резервируем индекс для неизвестных слов."
    );
    lesson_trace::trace_note!("Набор известных модели текстовых единиц называют vocabulary.");
    let known_text_units: std::collections::BTreeMap<String, usize> =
        (|| -> std::collections::BTreeMap<String, usize> {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!(
                "Нумеруем слова корпуса; нулевой индекс оставляем неизвестному токену."
            );
            lesson_trace::trace_note!("Сохраняем результат этого шага в `corpus`.");
            let corpus: &[&str] = &corpus;
            lesson_trace::trace_step!(corpus);
            lesson_trace::trace_note!(
                "Инициализируем изменяемый накопитель `known_text_units` начальным состоянием."
            );
            let mut known_text_units: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            lesson_trace::trace_step!(known_text_units);
            lesson_trace::trace_note!(
                "Выполняем очередное действие, после которого продолжаем следующий шаг."
            );
            known_text_units.insert("<unk>".into(), 0);
            lesson_trace::trace_note!(
                "Повторяем следующий блок для каждого элемента указанной последовательности."
            );
            lesson_trace::trace_note!(
                "Перебираем элементы по ссылке, не копируя исходную коллекцию."
            );
            lesson_trace::trace_note!("Разделяем текст по пробельным символам на отдельные слова.");
            for word in corpus
                .iter()
                .flat_map(|sentence| sentence.split_whitespace())
            {
                lesson_trace::trace_step!(word);
                lesson_trace::trace_note!(
                    "Проверяем условие и выбираем соответствующую ветку алгоритма."
                );
                if !known_text_units.contains_key(word) {
                    lesson_trace::trace_note!(
                        "Считаем количество элементов и сохраняем его в `text_unit_identifier`."
                    );
                    lesson_trace::trace_note!(
                        "Единицу текста, которую модель обрабатывает как одно целое, называют token."
                    );
                    let text_unit_identifier: usize = known_text_units.len();
                    lesson_trace::trace_step!(text_unit_identifier);
                    lesson_trace::trace_note!(
                        "Выполняем очередное действие, после которого продолжаем следующий шаг."
                    );
                    known_text_units.insert(word.into(), text_unit_identifier);
                }
            }
            lesson_trace::trace_note!(
                "Используем ранее рассчитанное значение `known_text_units` в текущем выражении."
            );
            known_text_units
        })();
    lesson_trace::trace_step!(known_text_units);

    lesson_trace::trace_note!("Шаг: Создаём таблицу векторов и читаем строки по индексам токенов.");
    lesson_trace::trace_note!("Плотное числовое представление объекта называют embedding.");
    let mut dense_numeric_representations: Vec<[f64; 2]> = vec![[0., 0.]; known_text_units.len()];
    lesson_trace::trace_step!(dense_numeric_representations);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for (text_unit_index, row) in dense_numeric_representations.iter_mut().enumerate() {
        lesson_trace::trace_step!(text_unit_index);
        lesson_trace::trace_step!(row);
        lesson_trace::trace_note!(
            "Передаём данные по ссылке или разыменовываем их для следующей операции."
        );
        *row = [text_unit_index as f64 * 0.1, text_unit_index as f64 * 0.2];
        lesson_trace::trace_step!(row);
    }
    lesson_trace::trace_note!(
        "Выполняем встроенный расчёт один раз и сохраняем результат в `text_unit_indices`."
    );
    let text_unit_indices: Vec<usize> = (|| -> Vec<usize> {
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!(
            "Каждое слово превращаем в индекс словаря, неизвестным словам даём нулевой индекс."
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `text`.");
        let text: &str = "кот неизвестно";
        lesson_trace::trace_step!(text);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `known_text_units` для следующих операций."
        );
        let known_text_units: &std::collections::BTreeMap<String, usize> = &known_text_units;
        lesson_trace::trace_step!(known_text_units);
        lesson_trace::trace_note!("Разделяем текст по пробельным символам на отдельные слова.");
        lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        text.split_whitespace()
            .map(|word| *known_text_units.get(word).unwrap_or(&0))
            .collect()
    })();
    lesson_trace::trace_step!(text_unit_indices);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!(
        "Используем ранее рассчитанное значение `text_unit_indices` в текущем выражении."
    );
    lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
    lesson_trace::trace_note!("Собираем полученные элементы в вектор.");
    println!(
        "vocab={known_text_units:?}, ids={text_unit_indices:?}, vectors={:?}",
        text_unit_indices
            .iter()
            .map(|&token_index| dense_numeric_representations[token_index])
            .collect::<Vec<_>>()
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_number_of_known_text_units_and_sequence_length(known_text_units, text_unit_indices);
}

// Строим график по результатам урока.
fn plot_number_of_known_text_units_and_sequence_length(
    known_text_units: std::collections::BTreeMap<std::string::String, usize>,
    text_unit_indices: std::vec::Vec<usize>,
) {
    lesson_trace::trace_note!("Наглядное сравнение результатов сводной практики.");
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
        "Размер словаря и последовательности",
        "число элементов",
        &[
            ("словарь", known_text_units.len() as f64),
            ("токены", text_unit_indices.len() as f64),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
