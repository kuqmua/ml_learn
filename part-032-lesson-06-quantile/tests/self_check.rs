// Самопроверка для урока 032. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question =
        "Найди минимум, медиану и максимум через квантили на пяти упорядоченных значениях.";
    let choices = [
        "Среднее равно 2, сумма квадратов отклонений равна 2, дисперсия равна 1.",
        "Объясни используемое правило индексации и проверь граничные доли 0 и 1.",
        "Сравни ширину интервалов и объясни роль размера выборки.",
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
