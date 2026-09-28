// Самопроверка для урока 007. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Запиши форму матрицы из двух строк и трёх столбцов и попробуй добавить строку другой длины.";
    let choices = [
        "Покажи форму 2×3 и причину, по которой неровные строки нельзя использовать как обычную матрицу.",
        "После двух операций получи исходные элементы и форму 2×3.",
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
        selected_choice, 1,
        "вернись к примеру и проверь своё объяснение"
    );
}
