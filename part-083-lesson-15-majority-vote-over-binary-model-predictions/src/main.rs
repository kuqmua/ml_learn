// Урок 15.3. Голосование большинства по бинарным прогнозам моделей.
// Почему этот урок сейчас: Несколько моделей дают несколько ответов; для итогового класса нужна агрегация.
// Почему пример устроен так: Считаем голоса и отдельно фиксируем правило для ничьей.
//
// Больше половины голосов true даёт true, меньше — false.
// При равенстве голосов правило этого примера выбирает false.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `cases`.
    let cases: [(&str, &[bool], bool); 4] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("большинство за true", &[true, true, false], true),
        // Добавляем пару значений для сравнения или построения графика.
        ("большинство за false", &[true, false, false], false),
        // Добавляем пару значений для сравнения или построения графика.
        ("ничья", &[true, false], false),
        // Добавляем пару значений для сравнения или построения графика.
        ("один голос", &[true], true),
    ];
    lesson_trace::trace_step!(cases);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, votes, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(votes);
        lesson_trace::trace_step!(expected);
        // Проверяем ожидаемое свойство учебного примера.
        assert!(!votes.is_empty(), "для решения нужен хотя бы один голос");
        // Вычисляем `positives` по элементам исходной коллекции.
        let positives: usize = votes.iter().filter(|&&vote| vote).count();
        lesson_trace::trace_step!(positives);
        // Определяем размер данных и сохраняем его в `result`.
        let result: bool = positives * 2 > votes.len();
        lesson_trace::trace_step!(result);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(result, expected);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: {votes:?} → {result}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_majority_vote_over_binary_model_predictions();
}

// Строим график по результатам урока.
fn visualize_majority_vote_over_binary_model_predictions() {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Большинство голосов",
        // Указываем подпись вертикальной оси.
        "количество голосов",
        // Передаём ряды или значения для отрисовки графика.
        &[("за", 3.0), ("против", 2.0)],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
