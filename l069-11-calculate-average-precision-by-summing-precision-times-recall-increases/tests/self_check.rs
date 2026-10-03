// Самопроверка для урока 069. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str = "Построй точки correct_positive_prediction_share_where_1_means_no_false_alarms-actual_positive_detection_share_where_1_means_none_missed для четырёх объектов при последовательном снижении порога.";
    let _choices: [&str; 3] = [
        "Проверь, что идеальное ранжирование даёт ROC-AUC 1, а обратное — 0.",
        "Укажи, в каких шагах добавление ложноположительного объекта уменьшает correct_positive_prediction_share_where_1_means_no_false_alarms.",
        "Сравни их с accuracy модели, всегда выдающей отрицательный класс.",
    ];

    let selected_choice: Option<usize> = None;
    let selected_choice: usize =
        selected_choice.expect("замени None на Some(1), Some(2) или Some(3)");
    assert!(
        (1..=3).contains(&selected_choice),
        "номер варианта должен быть от 1 до 3"
    );
    assert_eq!(
        selected_choice, 2,
        "вернись к примеру и проверь своё объяснение"
    );
}
