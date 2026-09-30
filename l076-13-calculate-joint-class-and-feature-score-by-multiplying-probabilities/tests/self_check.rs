// Самопроверка для урока 076. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str =
        "Для двух признаков проверь предположение об условной независимости внутри класса.";
    let choices: [&str; 3] = [
        "Покажи, как априорная вероятность влияет на итоговый выбор класса.",
        "После сглаживания вероятность положительна; сумма вероятностей слов остаётся 1.",
        "Приведи пример зависимых признаков и объясни, как их двойной учёт влияет на оценку.",
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
        selected_choice, 3,
        "вернись к примеру и проверь своё объяснение"
    );
}
