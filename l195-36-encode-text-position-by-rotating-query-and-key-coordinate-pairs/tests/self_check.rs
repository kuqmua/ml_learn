// Самопроверка для урока 195. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Поверни запрос и ключ на одинаковую позицию и сравни их скалярное произведение.";
    let _choices: [&str; 3] = [
        "При нулевом либо малом epsilon направление нормированного вектора сохраняется; проверь влияние gamma.",
        "Покажи, какие запросы разделяют одни ключи и значения.",
        "Длина вектора сохраняется; зависимость от относительных позиций объяснена.",
    ];

    let selected_choice: Option<usize> = None;
    let selected_choice: usize =
        selected_choice.expect("замени None на Some(1), Some(2) или Some(3)");
    assert!(
        (1..=3).contains(&selected_choice),
        "номер варианта должен быть от 1 до 3"
    );
    assert_eq!(
        selected_choice, 3,
        "вернись к примеру и проверь своё объяснение"
    );
}
