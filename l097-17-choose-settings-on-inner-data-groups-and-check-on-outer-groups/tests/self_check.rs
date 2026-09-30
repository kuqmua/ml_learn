// Самопроверка для урока 097. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Выбери параметр на внутренних фолдах и оцени выбранную модель на внешнем.";
    let _choices: [&str; 3] = [
        "Сравни число положительных объектов в каждом фолде с обычным случайным разбиением.",
        "Сравни оценки и укажи, где данные проверки попали в подготовку.",
        "Внешний фолд ни разу не использован для выбора параметра.",
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
