// Урок 05.1. Вероятность события при известном условии: деление совместной вероятности на вероятность условия.
// Связь с принятой терминологией: Условная вероятность одного события при наступлении другого.
// Зачем здесь эта тема: После наблюдения B вероятность A может отличаться от исходной вероятности A.
// Почему код устроен так: Считаем долю A только среди случаев B, чтобы не перепутать
//   вероятность A при B с вероятностью B при A.
// Представь: Среди всех людей болезнь редка, но среди людей с положительным тестом её доля может
//   быть другой.
//
// P(A|B) — доля случаев A среди случаев B. Когда B не встречается, знаменатель равен нулю
// и условная вероятность на этих данных не определена.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, positive_tests, sick_and_positive, expected) in [
        ("часть положительных тестов верна", 20.0, 8.0, Some(0.4)),
        ("все положительные тесты верны", 20.0, 20.0, Some(1.0)),
        ("ни один положительный тест не верен", 20.0, 0.0, Some(0.0)),
        ("положительных тестов не было", 0.0, 0.0, None),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(positive_tests);
        lesson_trace::trace_step!(sick_and_positive);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(positive_tests >= 0.0 && sick_and_positive >= 0.0);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            sick_and_positive <= positive_tests,
            "совместных случаев не может быть больше всех случаев B"
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
        let probability: Option<f64> = if positive_tests == 0.0 {
            lesson_trace::trace_note!("Отмечаем отсутствие подходящего значения.");
            None
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!("Возвращаем присутствующее значение.");
            Some(sick_and_positive / positive_tests)
        };
        lesson_trace::trace_step!(probability);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(probability, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: P(болен | тест положительный)={probability:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_disease_probability_among_positive_tests();
}

// Строим график по результатам урока.
fn plot_disease_probability_among_positive_tests() {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Условная вероятность",
        "P(болен | положительный тест)",
        &[("часть", 8.0 / 20.0), ("все", 1.0), ("никто", 0.0)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
