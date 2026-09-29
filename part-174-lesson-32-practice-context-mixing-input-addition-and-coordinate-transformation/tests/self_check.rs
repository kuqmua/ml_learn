// Самопроверка для урока 174. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Проследи формы тензоров и значения одного токена через внимание, остаточную связь и feed-forward.";
    let choices: [&str; 3] = [
        "Одинаковые веса действуют на обе позиции отдельно; изменение одной позиции не меняет другую.",
        "Среднее нормализованных координат близко к 0; объясни роль малого epsilon.",
        "После каждого подслоя форма сохраняется, а итог отличается от входа.",
    ];
    println!("{question}");
    for (choice_number, choice) in choices.iter().enumerate() {
        println!("{}. {choice}", choice_number + 1);
    }
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
