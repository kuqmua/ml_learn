// Самопроверка для урока 181. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Измени один патч и пересчитай представление CLS-позиции.";
    let _choices: [&str; 3] = [
        "Первый патч может обратить внимание на последний; объясни отличие от причинной маски.",
        "Получается четыре патча; каждый исходный пиксель встречается ровно в одном.",
        "Представление для классификации меняется, хотя специальный начальный вектор прежний.",
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
