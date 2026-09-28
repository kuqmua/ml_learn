// Самопроверка для урока 120. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question =
        "Построй граф для функции двух переменных и получи обе производные автоматически.";
    let choices = [
        "Относительная ошибка мала при разумном шаге; внесённая ошибка в производную обнаруживается.",
        "Первый случай согласует размеры по столбцам, второй отклоняется или требует иного правила.",
        "Результаты совпадают с аналитическим расчётом и численной проверкой.",
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
