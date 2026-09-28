// Самопроверка для урока 009. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Умножь матрицу [[1, 2], [3, 4]] на [5, 6] вручную.";
    let choices = [
        "После двух операций получи исходные элементы и форму 2×3.",
        "Покажи ручной расчёт одной ячейки; единичная матрица не меняет вторую.",
        "Сверь результат [17, 39] с программой и проверь случай неверной длины вектора.",
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
