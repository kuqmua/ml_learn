// Самопроверка для урока 233. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Задай порог алерта и подай значения чуть ниже и выше порога.";
    let choices = [
        "Сравни среднее и высокий процентиль; объясни, какая величина показывает худший опыт пользователей.",
        "Алерт срабатывает только по заданному правилу; проверь отсутствие постоянных ложных тревог.",
        "Для каждого искусственно созданного сбоя покажи, какой сигнал мониторинга его обнаруживает.",
    ];
    println!("{question}");
    for (choice_number, choice) in choices.iter().enumerate() {
        println!("{}. {choice}", choice_number + 1);
    }
    let selected_choice: Option<usize> = None;
    let selected_choice = selected_choice.expect("замени None на Some(1), Some(2) или Some(3)");
    assert!(
        (1..=3).contains(&selected_choice),
        "номер варианта должен быть от 1 до 3"
    );
    assert_eq!(
        selected_choice, 2,
        "вернись к примеру и проверь своё объяснение"
    );
}
