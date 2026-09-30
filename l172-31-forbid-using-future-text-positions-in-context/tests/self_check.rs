// Самопроверка для урока 172. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Измени будущий токен в последовательности с причинной маской.";
    let _choices: [&str; 3] = [
        "Сумма вероятностей равна 1; увеличение второй оценки повышает её вероятность.",
        "Проверь, что каждый вес неотрицателен, суммы весов равны 1 и маска соблюдена.",
        "Вес внимания к будущему равен 0, прежний выход не меняется.",
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
