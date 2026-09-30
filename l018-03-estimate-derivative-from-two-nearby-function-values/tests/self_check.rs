// Самопроверка для урока 018. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Сравни численную производную x² при трёх значениях шага.";
    let _choices: [&str; 3] = [
        "В минимуме обе компоненты равны 0; рядом направление роста определяется знаками компонент.",
        "Покажи, когда результат близок к аналитическому, и почему очень маленький шаг тоже может мешать.",
        "Проверь обе компоненты в трёх точках и укажи допустимую погрешность.",
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
