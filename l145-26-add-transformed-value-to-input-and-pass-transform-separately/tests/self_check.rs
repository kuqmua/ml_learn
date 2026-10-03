// Самопроверка для урока 145. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Отключи остаточную связь и сравни выход блока.";
    let _choices: [&str; 3] = [
        "Покажи, как ворота ослабляют или пропускают сигнал.",
        "Каждый следующий выход использует только уже доступные значения; изменение будущего входа не влияет на прошлое.",
        "Покажи, какую часть исходного сигнала добавляла связь и где действует transformed_signal_passed_to_output_without_adding_input-путь.",
    ];

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
