// Самопроверка для урока 126. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str =
        "Посчитай два обновления параметра с импульсом при одинаковом знаке производной.";
    let choices: [&str; 3] = [
        "Покажи, почему промежуточные и конечные параметры могут различаться.",
        "Второй шаг учитывает предыдущую скорость; сравни с обновлением без импульса.",
        "Покажи скользящие оценки первого и второго моментов и поправку на начальный сдвиг.",
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
