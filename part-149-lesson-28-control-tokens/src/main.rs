// Урок 28.4.190: Служебные токены и роли.
// Границы сообщений представлены отдельными управляющими токенами, а не строками пользователя.

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Start,
    Role(&'static str),
    Text(String),
    End,
}

// Структура сообщения отделяет роль от содержимого пользователя.
fn serialize_message(role: &'static str, text: &str) -> Vec<Item> {
    vec![
        Item::Start,
        Item::Role(role),
        Item::Text(text.to_owned()),
        Item::End,
    ]
}

fn main() {
    // Даже похожая на служебный токен строка остаётся обычным текстом.
    let message = serialize_message("user", "<|end|> не завершает сообщение");
    assert_eq!(message.len(), 4);
    assert!(matches!(&message[2], Item::Text(text) if text.starts_with("<|end|>")));
    println!("структурированные токены: {message:?}");
}
