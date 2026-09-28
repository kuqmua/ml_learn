// Самопроверка для урока 022. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Вычисли обновление параметра по всему набору и по одному наблюдению.";
    let choices = [
        "Сравни первое обновление вручную и объясни разницу в шуме траектории.",
        "Найди старты с разными конечными минимумами и объясни причину.",
        "Для каждой истории укажи минимум, число шагов и признак успешной сходимости.",
    ];
    println!("{question}");
    for (choice_number, choice) in choices.iter().enumerate() {
        println!("{}. {choice}", choice_number + 1);
    }
    let selected_choice: Option<usize> = None;
    let selected_choice = selected_choice.expect("замени None на Some(1), Some(2) или Some(3)");
    assert!(
        (1..=3).contains(&selected_choice),
        "номер варианта должен быть от 1 до 3"
    );
    assert_eq!(
        selected_choice, 1,
        "вернись к примеру и проверь своё объяснение"
    );
}
