// Самопроверка для урока 031. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Сравни медиану наборов [1, 2, 100] и [1, 2, 3, 100].";
    let choices: [&str; 3] = [
        "Покажи, насколько новое значение сдвигает среднее относительно исходного 2.",
        "Среднее равно 2, сумма квадратов отклонений равна 2, дисперсия равна 1.",
        "Получаются 2 и 2.5; поясни, почему сортировка обязательна.",
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
