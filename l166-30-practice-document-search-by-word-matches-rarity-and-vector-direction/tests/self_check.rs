// Самопроверка для урока 166. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Собери поиск по нескольким текстам и ответ из найденного фрагмента.";
    let _choices: [&str; 3] = [
        "Проверь порядок, размер ответа и правило при равных оценках.",
        "Объясни, почему нормировка влияет на сравнение длинного и короткого документа.",
        "Ответ содержит только сведения найденного текста; при отсутствии совпадения система воздерживается.",
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
