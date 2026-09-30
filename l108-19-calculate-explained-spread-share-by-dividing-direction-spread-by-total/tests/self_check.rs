// Самопроверка для урока 108. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Найди долю объяснённой дисперсии для собственных значений 3 и 1.";
    let _choices: [&str; 3] = [
        "Главным выбрано направление с большим разбросом проекций.",
        "Первая ось объясняет 0.75 общей дисперсии; сумма долей равна 1.",
        "Восстанови одну точку из проекции и сравни потерянную информацию со второй осью.",
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
