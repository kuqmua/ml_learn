// Самопроверка для урока 203. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let question = "Посчитай смесь сигнала и шума при сохранении исходной дисперсии 1 и 0.";
    let choices = [
        "Ошибка на отложенных примерах меньше baseline либо ясно объяснено, почему улучшения нет.",
        "При 1 выход равен чистому сигналу, при 0 — выбранному шуму.",
        "Первый результат совпадает с исходным, второй отклоняется; объясни чувствительность восстановления.",
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
