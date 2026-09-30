// Самопроверка для урока 077. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Измени доли классов в обучении при неизменных условных вероятностях признаков.";
    let _choices: [&str; 3] = [
        "Покажи, как априорная вероятность влияет на итоговый выбор класса.",
        "Приведи пример зависимых признаков и объясни, как их двойной учёт влияет на оценку.",
        "После сглаживания вероятность положительна; сумма вероятностей слов остаётся 1.",
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
