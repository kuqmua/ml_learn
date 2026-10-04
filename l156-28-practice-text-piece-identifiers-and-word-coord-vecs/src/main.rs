// Урок 156. Создаём словарь, получаем векторы по идентификаторам и обучаем строки таблицы.
// Учебная задача: отличать названия животных от действия; это не универсальные смысловые векторы.
fn main() {
    let mut vocabulary = std::collections::BTreeMap::from([("<unk>", 0_usize)]);
    for word in ["кот", "спит", "пёс"] {
        vocabulary.insert(word, vocabulary.len());
    }
    let mut embeddings = vec![[0.0_f64; 2]; vocabulary.len()];
    let training = [("кот", 1.0), ("пёс", 1.0), ("спит", 0.0)];
    let readout = [0.7, -0.4];
    let probability = |row: [f64; 2]| {
        let score = row[0] * readout[0] + row[1] * readout[1];
        1.0 / (1.0 + (-score).exp())
    };
    let loss = |table: &[[f64; 2]]| {
        training
            .iter()
            .map(|&(word, target)| {
                let p = probability(table[vocabulary[word]]);
                -(target * p.ln() + (1.0 - target) * (1.0 - p).ln())
            })
            .sum::<f64>()
            / training.len() as f64
    };
    let before = loss(&embeddings);
    for _ in 0..100 {
        for (word, target) in training {
            let id = vocabulary[word];
            let derivative_by_score = probability(embeddings[id]) - target;
            // Обновляется только строка текущего слова. Оба её числа — параметры модели.
            for coord in 0..2 {
                embeddings[id][coord] -= 0.2 * derivative_by_score * readout[coord];
            }
        }
    }
    let after = loss(&embeddings);
    assert!(after < before);
    println!("Ошибка задачи {before} -> {after}; словарь={vocabulary:?}");
    for (word, target) in training {
        let id = vocabulary[word];
        let p = probability(embeddings[id]);
        assert_eq!(p >= 0.5, target == 1.0);
        println!(
            "{word}: id={id}, вектор={:?}, вероятность животного={p}",
            embeddings[id]
        );
    }
    let ids: Vec<_> = "кот неизвестно кот"
        .split_whitespace()
        .map(|word| *vocabulary.get(word).unwrap_or(&0))
        .collect();
    assert_eq!(ids[0], ids[2]);
    assert_eq!(ids[1], 0);
    assert_eq!(embeddings[0], [0.0, 0.0]);
    println!("Новые слова: ids={ids:?}; неизвестное слово использует неизменённую строку 0");
    // Совпадение ответов на трёх обучающих словах не доказывает качество на новых словах.
}

// Чему учит этот урок:
// Различаем номер слова и обучаемый вектор, обновляем выбранную строку по ошибке задачи.
// Проверяем повторное использование идентификатора и отдельную строку неизвестных слов.
// Значение координат определяется учебной задачей; неизвестные слова автоматически не обучаются.
