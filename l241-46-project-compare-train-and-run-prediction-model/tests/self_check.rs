// Самопроверка для урока 241. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Проведи обучение, выбор параметра и итоговую оценку на отдельных частях данных.";
    let _choices: [&str; 3] = [
        "Сравни модель с baseline, сохрани конфигурацию и один раз отчитайся по test.",
        "Для каждого искусственно созданного сбоя покажи, какой сигнал мониторинга его обнаруживает.",
        "Два проверяемых вопроса получают обоснованные ответы, неизвестный вопрос — честный отказ.",
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
