// Самопроверка для урока 192. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Подай короткую последовательность в учебный GPT и измени последний токен.";
    let _choices: [&str; 3] = [
        "Цель каждой позиции — следующий, а не текущий токен; средняя ошибка проверена вручную.",
        "Сравни ошибку до и после, затем отдельно проверь качество на отложенной последовательности.",
        "Ранние скрытые состояния сохраняются из-за причинной маски.",
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
