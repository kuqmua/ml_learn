// Урок 38.4. Отказ от ответа при слишком низкой оценке соответствия источника.
// Связь с принятой терминологией: Отказ от ответа RAG при низкой оценке источника.
// Зачем здесь эта тема: Слабое совпадение источника может сделать ответ выдуманным.
// Почему код устроен так: Сравниваем оценку найденного фрагмента с порогом и при недостаточной
//   опоре отказываемся отвечать.
// Представь: Если лучший найденный документ почти не связан с вопросом, лучше честно отказаться от
//   ответа.
//
// Ответ допускается при оценке источника не ниже порога; ниже порога система воздерживается.

fn main() {
    let minimum_reliable_score: f64 = 0.5;
    for (_description, document_relevance_score, expected_answer) in [
        ("слабый источник", 0.1, "нет надёжного источника"),
        ("ровно на пороге", 0.5, "подтверждённый ответ"),
        ("сильный источник", 0.9, "подтверждённый ответ"),
    ] {
        assert!((0.0..=1.0).contains(&document_relevance_score));
        let answer: &str = if document_relevance_score >= minimum_reliable_score {
            "подтверждённый ответ"
        } else {
            "нет надёжного источника"
        };
        assert_eq!(answer, expected_answer);
    }

    plot_source_score_compared_with_acceptance_threshold(minimum_reliable_score);
}

// Строим график по результатам урока.
fn plot_source_score_compared_with_acceptance_threshold(minimum_reliable_score: f64) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Порог проверки источника",
        "оценка",
        &[
            ("ниже", 0.3),
            ("порог", minimum_reliable_score),
            ("выше", 0.8),
        ],
    )
    .expect("не удалось сохранить график");
}
