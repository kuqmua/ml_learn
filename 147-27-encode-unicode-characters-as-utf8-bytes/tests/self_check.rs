// Самопроверка для урока 147. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Сравни количество символов и байтов в строке с кириллицей и эмодзи.";
    let choices: [&str; 3] = [
        "На каждом шаге укажи самую частую соседнюю пару и результат её слияния.",
        "Объясни, почему один видимый символ может занимать несколько байтов UTF-8.",
        "Декодирование восстанавливает исходные байты; укажи, как обработан незнакомый фрагмент.",
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
        selected_choice, 2,
        "вернись к примеру и проверь своё объяснение"
    );
}
