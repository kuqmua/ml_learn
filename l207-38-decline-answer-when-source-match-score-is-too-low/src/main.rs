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
    lesson_trace::enable();
    lesson_trace::trace_note!("Сохраняем результат этого шага в `minimum_reliable_score`.");
    let minimum_reliable_score: f64 = 0.5;
    lesson_trace::trace_step!(minimum_reliable_score);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!(
        "Поиск подходящих документов и оценку их релевантности называют retrieval."
    );
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, document_relevance_score, expected_answer) in [
        ("слабый источник", 0.1, "нет надёжного источника"),
        ("ровно на пороге", 0.5, "подтверждённый ответ"),
        ("сильный источник", 0.9, "подтверждённый ответ"),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(document_relevance_score);
        lesson_trace::trace_step!(expected_answer);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((0.0..=1.0).contains(&document_relevance_score));
        lesson_trace::trace_note!("Сохраняем результат этого шага в `answer`.");
        let answer: &str = if document_relevance_score >= minimum_reliable_score {
            lesson_trace::trace_note!(
                "Передаём подпись или текстовое значение для следующего шага."
            );
            "подтверждённый ответ"
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Передаём подпись или текстовое значение для следующего шага."
            );
            "нет надёжного источника"
        };
        lesson_trace::trace_step!(answer);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(answer, expected_answer);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: score={document_relevance_score} → {answer}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_source_score_compared_with_acceptance_threshold(minimum_reliable_score);
}

// Строим график по результатам урока.
fn plot_source_score_compared_with_acceptance_threshold(minimum_reliable_score: f64) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
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
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
