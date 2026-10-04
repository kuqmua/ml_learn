// Урок 152. Присваивать каждому уникальному слову постоянный числовой идентификатор.
// Повторное слово использует прежний номер, что позволяет перейти от текста к числовой обработке.

fn main() {
    let mut vocabulary = std::collections::BTreeMap::new();
    let words = ["кот", "спит", "кот"];
    let mut ids = Vec::new();
    for word in words {
        let next = vocabulary.len() + 1;
        let id = *vocabulary.entry(word).or_insert(next);
        ids.push(id);
    }
    println!("Слова={words:?}; словарь={vocabulary:?}; номера={ids:?}");
    assert_eq!(ids[0], ids[2]);
    assert_ne!(ids[0], ids[1]);
}

// Чему учит этот урок:
// Учимся присваивать каждому уникальному слову постоянный числовой идентификатор.
// Повторное слово использует прежний номер, что позволяет перейти от текста к числовой обработке.
