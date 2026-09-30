// Самопроверка для урока 242. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Собери итоговую систему ответа с поиском источников, проверкой ссылок и отказом при нехватке данных.";
    let _choices: [&str; 3] = [
        "Сравни модель с baseline, сохрани конфигурацию и один раз отчитайся по test.",
        "Два проверяемых вопроса получают обоснованные ответы, неизвестный вопрос — честный отказ.",
        "Для каждого искусственно созданного сбоя покажи, какой сигнал мониторинга его обнаруживает.",
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
