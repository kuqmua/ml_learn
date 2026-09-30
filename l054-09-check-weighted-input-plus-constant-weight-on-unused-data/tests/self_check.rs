// Самопроверка для урока 054. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Подбери модель по train и оцени её на отдельно заданных test-точках.";
    let _choices: [&str; 3] = [
        "Сравни величину коэффициентов и ошибку на отложенных данных.",
        "Покажи пример, где хорошее качество на train не гарантирует такое же на test.",
        "Запиши коэффициенты и ошибки train/test; проверь один прогноз вручную.",
    ];

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
