// Самопроверка для урока 140. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Измени входные и забывающие ворота LSTM по отдельности.";
    let _choices: [&str; 3] = [
        "Покажи, когда прежнее состояние сохраняется и когда новые данные доминируют.",
        "Запиши произведение производных по шагам и сравни с численной оценкой.",
        "Объясни, какой из режимов оставляет прежнее состояние.",
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
