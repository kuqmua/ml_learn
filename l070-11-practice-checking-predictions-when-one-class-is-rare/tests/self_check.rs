// Самопроверка для урока 070. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Вычисли correct_pos_prediction_share, actual_pos_detection_share и F1 для набора с редким положительным классом.";
    let _choices: [&str; 3] = [
        "Укажи, в каких шагах добавление ложноположительного объекта уменьшает correct_pos_prediction_share.",
        "Проверь, что идеальное ранжирование даёт ROC-AUC 1, а обратное — 0.",
        "Сравни их с accuracy модели, всегда выдающей отрицательный класс.",
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
