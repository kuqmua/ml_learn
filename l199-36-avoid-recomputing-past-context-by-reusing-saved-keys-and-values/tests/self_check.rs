// Самопроверка для урока 199. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Сгенерируй два токена с кешем ключей и значений и без кеша.";
    let _choices: [&str; 3] = [
        "Ответы совпадают; перечисли, какие старые ключи и значения повторно не вычисляются.",
        "Выход ослабляется или усиливается согласно воротам; размер выхода не меняется.",
        "Только выбранные эксперты участвуют в смеси; веса их вкладов корректно нормированы.",
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
