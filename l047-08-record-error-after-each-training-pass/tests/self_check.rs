// Самопроверка для урока 047. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Запиши три значения ошибки по шагам обучения и намеренно нарушь порядок шагов.";
    let _choices: [&str; 3] = [
        "Журнал сохраняет понятную историю и либо отклоняет, либо явно показывает нарушение порядка.",
        "Сохрани оба значения параметра рядом с результатами, чтобы объяснить разницу запусков.",
        "Результат эксперимента можно связать с конкретной версией данных.",
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
