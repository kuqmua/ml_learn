// Самопроверка для урока 128. Практическое задание находится в SELF_CHECK.md.
// Сначала реши задачу, затем выбери верный критерий и замени None на Some(1), Some(2) или Some(3).
#[test]
#[ignore = "заполни ответ в tests/self_check.rs и запусти тест с --ignored --nocapture"]
fn choose_correct_check_for_lesson() {
    let _question: &str =
        "Нормализуй признак по среднему и разбросу train, затем обработай новое значение.";
    let _choices: [&str; 3] = [
        "Среднее train после преобразования близко к нулю; новое значение не меняет статистики train.",
        "Покажи скользящие оценки первого и второго моментов и поправку на начальный сдвиг.",
        "Получаются -1, 0.5 и 1 при покоординатном ограничении; поясни другое правило для нормы вектора.",
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
