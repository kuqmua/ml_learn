// Урок 27.4. Обозначение начала, роли и конца сообщений чата.
// Связь с принятой терминологией: Управляющие токены для границ сообщений чата.
// Зачем здесь эта тема: В чате одной строки текста недостаточно: модель должна различать роли и
//   границы сообщений.
// Почему код устроен так: Добавляем специальные токены отдельно от обычного содержимого сообщений.
// Представь: В записи чата текст «assistant» как обычное слово отличается от специальной метки роли
//   assistant.
// Границы сообщений представлены отдельными управляющими токенами, а не строками пользователя.

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Start,
    Role(&'static str),
    Text(String),
    End,
}

// Структура сообщения отделяет роль от содержимого пользователя.
/// Служебные токены: представляем сообщение как начало, роль, текст и конец.
fn serialize_chat_message_by_adding_start_role_and_end_markers(
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
    let message: Vec<Item> =
        serialize_chat_message_by_adding_start_role_and_end_markers(role, text);
    lesson_trace::trace_step!(message);
    assert_eq!(message.len(), 4);
    assert!(matches!(&message[2], Item::Text(text) if text.starts_with("<|end|>")));
    println!("структурированные токены: {message:?}");
}
