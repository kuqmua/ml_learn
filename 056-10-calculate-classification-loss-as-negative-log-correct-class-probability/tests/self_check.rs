// Самопроверка для урока 056. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str =
        "Сравни логарифмическую ошибку уверенного правильного и уверенного ошибочного прогноза.";
    let choices: [&str; 3] = [
        "Значение при 0 равно 0.5, все ответы лежат между 0 и 1.",
        "Ошибочный уверенный прогноз штрафуется сильнее; проверь обе метки 0 и 1.",
        "Проверь, как знак коэффициента меняет порядок вероятностей.",
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
        selected_choice, 2,
        "вернись к примеру и проверь своё объяснение"
    );
}
