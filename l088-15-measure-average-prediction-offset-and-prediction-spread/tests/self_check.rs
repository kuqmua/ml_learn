// Самопроверка для урока 088. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str =
        "Сравни постоянно ошибающуюся модель с моделью, сильно меняющей ответы между выборками.";
    let choices: [&str; 3] = [
        "Итоговый класс меняется; явно задай правило при равенстве голосов.",
        "Сравни ошибку ансамбля с ошибкой каждого участника на общей проверочной части.",
        "Объясни, какая демонстрирует высокое смещение, а какая — высокий разброс.",
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
