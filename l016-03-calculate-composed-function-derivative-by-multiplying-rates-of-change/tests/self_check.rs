// Самопроверка для урока 016. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Для f(x)=(2x+1)² вычисли производную в двух точках двумя способами.";
    let _choices: [&str; 3] = [
        "Полученные изменения соответствуют разным частным производным.",
        "В минимуме обе компоненты равны 0; рядом направление роста определяется знаками компонент.",
        "Раскрытие скобок и правило цепочки дают одинаковые ответы.",
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
