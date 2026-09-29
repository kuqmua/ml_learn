// Самопроверка для урока 222. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Проверь загрузку пустого файла, NaN и файла с лишним полем.";
    let choices: [&str; 3] = [
        "Известная версия читается, неизвестная получает понятное сообщение об ошибке.",
        "Загрузчик либо корректно переводит старую версию, либо объясняет отсутствие поддержки.",
        "Все три случая отклоняются до использования параметров модели.",
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
