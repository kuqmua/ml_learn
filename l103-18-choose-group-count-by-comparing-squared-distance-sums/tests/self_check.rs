// Самопроверка для урока 103. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Посчитай инерцию для одного, двух и трёх кластеров.";
    let _choices: [&str; 3] = [
        "Каждая точка отнесена к ближайшему центру, квадраты расстояний суммированы.",
        "Сравни назначения и итоговую инерцию; проверь, что пустой кластер обработан явно.",
        "Инерция не растёт с увеличением числа центров; объясни, почему минимум не равен автоматически лучшему k.",
    ];

    let selected_choice: Option<usize> = None;
    let selected_choice: usize =
        selected_choice.expect("замени None на Some(1), Some(2) или Some(3)");
    assert!(
        (1..=3).contains(&selected_choice),
        "номер варианта должен быть от 1 до 3"
    );
    assert_eq!(
        selected_choice, 3,
        "вернись к примеру и проверь своё объяснение"
    );
}
