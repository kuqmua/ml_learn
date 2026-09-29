// Самопроверка для урока 097. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str =
        "Запусти k-means с близкими и удалёнными друг от друга начальными центрами.";
    let choices: [&str; 3] = [
        "Сравни итоговые группы и ошибки; объясни влияние инициализации.",
        "Получается [2,0]; после добавления [10,0] центр меняется предсказуемо.",
        "Каждая точка отнесена к ближайшему центру, квадраты расстояний суммированы.",
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
