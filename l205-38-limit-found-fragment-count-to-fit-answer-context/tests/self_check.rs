// Самопроверка для урока 205. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Поменяй порядок фрагментов при ограниченной длине контекста.";
    let _choices: [&str; 3] = [
        "Покажи, какой фрагмент отбрасывается и может ли измениться ответ.",
        "Первый ответ опирается на контекст, второй явно признаёт нехватку сведений.",
        "Каждая ссылка ведёт к фрагменту, который действительно поддерживает утверждение.",
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
