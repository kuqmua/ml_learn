// Самопроверка для урока 187. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Измени значение токена после рассматриваемой позиции.";
    let _choices: [&str; 3] = [
        "Покажи, как позиционные добавки различают их итоговые представления.",
        "Её выход остаётся прежним; изменение более раннего токена может повлиять.",
        "Покажи разные веса или выходы и объясни пользу нескольких голов.",
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
