// Самопроверка для урока 041. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Добавь категорию, отсутствующую при обучении словаря.";
    let _choices: [&str; 3] = [
        "Новая категория получает заранее предусмотренный код, не меняя коды известных категорий.",
        "Подставляемое значение не должно измениться, потому что оно вычислено только по train.",
        "Покажи, как второй вариант меняет подготовку данных и почему это утечка.",
    ];

    let selected_choice: Option<usize> = None;
    let selected_choice: usize =
        selected_choice.expect("замени None на Some(1), Some(2) или Some(3)");
    assert!(
        (1..=3).contains(&selected_choice),
        "номер варианта должен быть от 1 до 3"
    );
    assert_eq!(
        selected_choice, 1,
        "вернись к примеру и проверь своё объяснение"
    );
}
