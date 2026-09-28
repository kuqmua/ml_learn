// Урок 37.187: Символы, байты и токены.
// UTF-8 кодирует символы несколькими байтами; байт не обязан быть целым символом.

fn main() {
    // Сравниваем три разных размера одной и той же строки.
    let text = "кот 🐈";
    let bytes = text.as_bytes();
    let characters: Vec<char> = text.chars().collect();
    assert_eq!(String::from_utf8(bytes.to_vec()).unwrap(), text);
    assert!(bytes.len() > characters.len());
    // Один токен модели может содержать часть слова, слово или несколько слов.
    println!(
        "строка: {text:?}; символов: {}; байтов: {}",
        characters.len(),
        bytes.len()
    );
    println!("UTF-8: {bytes:?}");
}
