// Урок 27.1. Кодирование символов Unicode в байты UTF-8.
// UTF-8 кодирует символы несколькими байтами; байт не обязан быть целым символом.

fn main() {
    lesson_trace::enable();
    // Сравниваем три разных размера одной и той же строки.
    let text: &str = "кот 🐈";
    lesson_trace::trace_step!(text);
    let bytes: &[u8] = text.as_bytes();
    lesson_trace::trace_step!(bytes);
    let characters: Vec<char> = text.chars().collect();
    lesson_trace::trace_step!(characters);
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
