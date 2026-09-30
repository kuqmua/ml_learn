// Урок 27.1. Представление символов текста байтами UTF-8.
// Связь с принятой терминологией: Кодирование символов Unicode в байты UTF-8.
// Зачем здесь эта тема: Токенизатор сначала должен получить однозначное представление любого
//   символа.
// Почему код устроен так: Показываем UTF-8 байты, включая символы вне ASCII, чтобы дальнейшие
//   слияния имели полное покрытие.
// Представь: Буква «я» представляется несколькими байтами UTF-8, а не одним ASCII-байтом.
// UTF-8 кодирует символы несколькими байтами; байт не обязан быть целым символом.

use lesson_trace::{enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Сравниваем три разных размера одной и той же строки.");
    let text: &str = "кот 🐈";
    trace_step!(text);
    let bytes: &[u8] = text.as_bytes();
    trace_step!(bytes);
    let characters: Vec<char> = text.chars().collect();
    trace_step!(characters);
    assert_eq!(String::from_utf8(bytes.to_vec()).unwrap(), text);
    assert!(bytes.len() > characters.len());
    trace_note!("Один токен модели может содержать часть слова, слово или несколько слов.");
    println!(
        "строка: {text:?}; символов: {}; байтов: {}",
        characters.len(),
        bytes.len()
    );
    println!("UTF-8: {bytes:?}");
}
