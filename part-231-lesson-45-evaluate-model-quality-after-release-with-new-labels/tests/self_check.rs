// Самопроверка для урока 231. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Добавь после релиза примеры с известными ответами и вычисли новую метрику.";
    let choices = [
        "Укажи, какой интервал изменился сильнее и почему одного среднего недостаточно.",
        "Сравни среднее и высокий процентиль; объясни, какая величина показывает худший опыт пользователей.",
        "Сравни её с исходной на одинаковом определении метрики и учти задержку появления меток.",
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
        selected_choice, 3,
        "вернись к примеру и проверь своё объяснение"
    );
}
