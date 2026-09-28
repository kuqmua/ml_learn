// Самопроверка для урока 126. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Создай историю validation-ошибки, которая сначала падает, затем растёт.";
    let choices = [
        "Получаются -1, 0.5 и 1 при покоординатном ограничении; поясни другое правило для нормы вектора.",
        "Зафиксируй, какая мера предотвратила нестабильность, и сравни качество с baseline.",
        "Определи шаг остановки по заданному patience; test не используется для остановки.",
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
        selected_choice, 3,
        "вернись к примеру и проверь своё объяснение"
    );
}
