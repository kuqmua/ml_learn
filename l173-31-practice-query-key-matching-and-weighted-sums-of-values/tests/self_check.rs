// Самопроверка для урока 173. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question: &str =
        "Рассчитай внимание для двух позиций вручную: оценки, softmax и смесь значений.";
    let choices: [&str; 3] = [
        "Проверь, что каждый вес неотрицателен, суммы весов равны 1 и маска соблюдена.",
        "Вес внимания к будущему равен 0, прежний выход не меняется.",
        "Сумма вероятностей равна 1; увеличение второй оценки повышает её вероятность.",
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
