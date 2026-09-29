// Самопроверка для урока 016. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str = "Вычисли градиент f(x,y)=(x-2)²+3(y+1)² в точке (2,-1) и рядом с ней.";
    let choices: [&str; 3] = [
        "В минимуме обе компоненты равны 0; рядом направление роста определяется знаками компонент.",
        "Раскрытие скобок и правило цепочки дают одинаковые ответы.",
        "Покажи, когда результат близок к аналитическому, и почему очень маленький шаг тоже может мешать.",
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
