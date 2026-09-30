// Самопроверка для урока 157. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str =
        "Для короткого текста посчитай частоты следующих слов после одного выбранного слова.";
    let choices: [&str; 3] = [
        "Меньшая вероятность правильного ответа даёт большую cross-entropy.",
        "При нулевой ошибке ответ 1; при увеличении ошибки perplexity растёт.",
        "Прогнозируемое распределение суммируется в 1; назови наиболее вероятное продолжение.",
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
