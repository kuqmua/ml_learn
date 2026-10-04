// Урок 28.5. Практика: номера частей текста и векторы координат слов.
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
    let known_text_units: std::collections::BTreeMap<String, usize> =
        (|| -> std::collections::BTreeMap<String, usize> {
            let corpus: [&str; 2] = ["кот спит", "пёс спит"];
            let corpus: &[&str] = &corpus;
            let mut known_text_units: std::collections::BTreeMap<String, usize> =
                std::collections::BTreeMap::new();
            known_text_units.insert("<unk>".into(), 0);
            for word in corpus
                .iter()
                .flat_map(|sentence| sentence.split_whitespace())
            {
                if !known_text_units.contains_key(word) {
                    known_text_units.insert(word.into(), known_text_units.len());
                }
            }
            known_text_units
        })();

    let mut dense_numeric_representations: Vec<[f64; 2]> = vec![[0.0, 0.0]; known_text_units.len()];
    for (text_unit_index, row) in dense_numeric_representations.iter_mut().enumerate() {
        *row = [text_unit_index as f64 * 0.1, text_unit_index as f64 * 0.2];
    }
    let text_unit_indices: Vec<usize> = (|| -> Vec<usize> {
        let known_text_units: &std::collections::BTreeMap<String, usize> = &known_text_units;
        let text: &str = "кот неизвестно";
        text.split_whitespace()
            .map(|word| *known_text_units.get(word).unwrap_or(&0))
            .collect()
    })();
    let _ = &(text_unit_indices
        .iter()
        .map(|&token_index| dense_numeric_representations[token_index])
        .collect::<Vec<_>>());

    // Выполняем вычисления из примера.
    let _ = (known_text_units, text_unit_indices);
}
