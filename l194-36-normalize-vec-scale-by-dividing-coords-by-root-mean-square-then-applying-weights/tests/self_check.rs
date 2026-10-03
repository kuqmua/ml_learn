// Самопроверка для урока 194. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Умножь вход на константу и сравни результат RMSNorm до и после.";
    let _choices: [&str; 3] = [
        "Длина вектора сохраняется; зависимость от относительных позиций объяснена.",
        "При нулевом либо малом pos_stabilizer_preventing_division_by_zero_for_zero_vec направление нормированного вектора сохраняется; проверь влияние gamma.",
        "Покажи, какие запросы разделяют одни ключи и значения.",
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
