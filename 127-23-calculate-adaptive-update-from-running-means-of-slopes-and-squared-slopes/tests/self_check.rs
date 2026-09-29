// Самопроверка для урока 127. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Вычисли первые два шага Adam для заданных небольших производных.";
    let choices: [&str; 3] = [
        "Второй шаг учитывает предыдущую скорость; сравни с обновлением без импульса.",
        "Среднее train после преобразования близко к нулю; новое значение не меняет статистики train.",
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
        selected_choice, 3,
        "вернись к примеру и проверь своё объяснение"
    );
}
