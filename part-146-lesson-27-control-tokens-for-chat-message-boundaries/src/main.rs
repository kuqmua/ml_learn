// Урок 27.4. Управляющие токены для границ сообщений чата.
// Почему этот урок сейчас: В чате одной строки текста недостаточно: модель должна различать роли и границы сообщений.
// Почему пример устроен так: Добавляем специальные токены отдельно от обычного содержимого сообщений.
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
    lesson_trace::enable();
    // Даже похожая на служебный токен строка остаётся обычным текстом.
    let role: &str = "user";
    lesson_trace::trace_step!(role);
    let text: &str = "<|end|> не завершает сообщение";
    lesson_trace::trace_step!(text);
    let message: Vec<Item> = serialize_chat_message_with_role_and_control_tokens(role, text);
    lesson_trace::trace_step!(message);
    assert_eq!(message.len(), 4);
    assert!(matches!(&message[2], Item::Text(text) if text.starts_with("<|end|>")));
    println!("структурированные токены: {message:?}");
}
