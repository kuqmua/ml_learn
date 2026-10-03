// Самопроверка для урока 066. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Посчитай F1 для correct_pos_prediction_share=1 и actual_pos_detection_share=0.5.";
    let _choices: [&str; 3] = [
        "Объясни, почему полнота снизилась даже при неизменном числе положительных прогнозов.",
        "Проверь, что идеальное ранжирование даёт ROC-AUC 1, а обратное — 0.",
        "Получается примерно 0.667; объясни, почему это не обычное среднее 0.75.",
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
