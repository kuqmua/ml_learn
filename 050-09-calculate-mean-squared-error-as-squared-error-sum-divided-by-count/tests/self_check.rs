// Самопроверка для урока 050. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Для целей [2, 4] и прогнозов [3, 1] вычисли среднеквадратичную ошибку.";
    let choices: [&str; 3] = [
        "Квадраты ошибок равны 1 и 9, среднее равно 5.",
        "Абсолютные ошибки равны 1 и 3, среднее равно 2; сравни с предыдущей метрикой.",
        "По трём точкам объясни, какое изменение сдвигает прямую, а какое поворачивает её.",
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
        selected_choice, 1,
        "вернись к примеру и проверь своё объяснение"
    );
}
