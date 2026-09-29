// Самопроверка для урока 187. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Обучи только выходные веса GPT и сохрани остальные фиксированными.";
    let choices: [&str; 3] = [
        "Сравни ошибку до и после, затем отдельно проверь качество на отложенной последовательности.",
        "Ранние скрытые состояния сохраняются из-за причинной маски.",
        "Цель каждой позиции — следующий, а не текущий токен; средняя ошибка проверена вручную.",
    ];
    println!("{question}");
    for (choice_number, choice) in choices.iter().enumerate() {
        println!("{}. {choice}", choice_number + 1);
    }
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
