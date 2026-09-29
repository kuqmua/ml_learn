// Урок 27.4. Управляющие токены для границ сообщений чата.
// Границы сообщений представлены отдельными управляющими токенами, а не строками пользователя.

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Start,
    Role(&'static str),
    Text(String),
    End,
}

// Структура сообщения отделяет роль от содержимого пользователя.
fn serialize_chat_message_with_role_and_control_tokens(
    role: &'static str,
    text: &str,
) -> Vec<Item> {
    vec![
        Item::Start,
        Item::Role(role),
        Item::Text(text.to_owned()),
        Item::End,
    ]
}

fn main() {
    // Даже похожая на служебный токен строка остаётся обычным текстом.
    let message: Vec<Item> = serialize_chat_message_with_role_and_control_tokens(
        "user",
        "<|end|> не завершает сообщение",
    );
    assert_eq!(message.len(), 4);
    assert!(matches!(&message[2], Item::Text(text) if text.starts_with("<|end|>")));
    println!("структурированные токены: {message:?}");
}
