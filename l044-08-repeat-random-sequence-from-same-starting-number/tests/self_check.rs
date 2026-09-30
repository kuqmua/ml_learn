// Самопроверка для урока 044. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Запусти генератор дважды с одинаковым seed и один раз с другим.";
    let _choices: [&str; 3] = [
        "Первые две последовательности совпадают; поясни, что seed не делает выборку более точной.",
        "Обе ошибки посчитаны одинаковой метрикой; сделай вывод, даёт ли обучение выигрыш.",
        "Сохрани оба значения параметра рядом с результатами, чтобы объяснить разницу запусков.",
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
