// Самопроверка для урока 234. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Измерь время обработки 100 и 1000 однотипных запросов.";
    let _choices: [&str; 3] = [
        "Программа сообщает о входной ошибке без паники и не выдаёт фиктивный прогноз.",
        "Ответы воспроизводимы, ошибки ввода понятны, формат вывода подходит для автоматической обработки.",
        "Сравни время на запрос и объясни накладные расходы запуска и пакетной обработки.",
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
