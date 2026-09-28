// Самопроверка для урока 041. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Посчитай среднее признака сначала по train, затем по train вместе с test.";
    let choices = [
        "Новая категория получает заранее предусмотренный код, не меняя коды известных категорий.",
        "Покажи, как второй вариант меняет подготовку данных и почему это утечка.",
        "Все обучаемые параметры получены только из train; повтор с тем же seed даёт те же части.",
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
