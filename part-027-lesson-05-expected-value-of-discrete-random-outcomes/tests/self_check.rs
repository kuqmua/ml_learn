// Самопроверка для урока 027. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Посчитай ожидание для исходов -2, 1 и 5 с вероятностями 0.2, 0.5 и 0.3.";
    let choices: [&str; 3] = [
        "Пересчитай вероятность болезни после положительного результата и объясни направление изменения.",
        "Сравни наблюдаемую частоту с теоретической и объясни, почему маленькая выборка может сильно отклоняться.",
        "Взвешенная сумма равна 1.6; объясни, почему ожидание не обязано быть возможным исходом.",
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
