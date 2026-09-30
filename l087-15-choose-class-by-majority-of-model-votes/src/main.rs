// Урок 15.3. Выбор класса большинством голосов моделей.
// Связь с принятой терминологией: Голосование большинства по бинарным прогнозам моделей.
// Зачем здесь эта тема: Несколько моделей дают несколько ответов; для итогового класса нужна
//   агрегация.
// Почему код устроен так: Считаем голоса и отдельно фиксируем правило для ничьей.
// Представь: Если три модели ответили A, A, B, большинство выбирает A.
//
// Больше половины голосов true даёт true, меньше — false.
// При равенстве голосов правило этого примера выбирает false.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[bool], bool); 4] = [
        ("большинство за true", &[true, true, false], true),
        ("большинство за false", &[true, false, false], false),
        ("ничья", &[true, false], false),
        ("один голос", &[true], true),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, votes, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(votes);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(!votes.is_empty(), "для решения нужен хотя бы один голос");
        lesson_trace::trace_note!("Вычисляем `positives` по элементам исходной коллекции.");
        let positives: usize = votes.iter().filter(|&&vote| vote).count();
        lesson_trace::trace_step!(positives);
        lesson_trace::trace_note!("Определяем размер данных и сохраняем его в `result`.");
        let result: bool = positives * 2 > votes.len();
        lesson_trace::trace_step!(result);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(result, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {votes:?} → {result}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_number_of_votes_for_each_class();
}

// Строим график по результатам урока.
fn plot_number_of_votes_for_each_class() {
    lesson_trace::trace_note!("Сравнение величин из этого урока.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Большинство голосов",
        "количество голосов",
        &[("за", 3.0), ("против", 2.0)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
