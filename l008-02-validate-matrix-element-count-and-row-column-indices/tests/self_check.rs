// Самопроверка для урока 008. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Запиши форму матрицы из двух строк и трёх столбцов и попробуй добавить строку другой длины.";
    let _choices: [&str; 3] = [
        "Покажи форму 2×3 и причину, по которой неровные строки нельзя использовать как обычную матрицу.",
        "После двух операций получи исходные элементы и форму 2×3.",
        "Сверь результат [17, 39] с программой и проверь случай неверной длины вектора.",
    ];

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
