// Урок 17.2. Сохранение близких долей классов в разных группах данных.
// Связь с принятой терминологией: Стратифицированные блоки с сохранением долей классов.
// Зачем здесь эта тема: При редком классе случайный блок может остаться без его примеров и исказить
//   метрику.
// Почему код устроен так: Распределяем метки по блокам так, чтобы доли классов были близки к
//   исходным.
// Представь: Если положительных случаев мало, распределяем их по блокам, чтобы один блок не
//   оказался пустым.
//
// Если разложить упорядоченные по классу данные подряд, в одной части может оказаться
// только положительный класс, в другой — только отрицательный. Стратификация смешивает классы.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `positive`.");
    let positive: [i32; 4] = [1, 3, 5, 7];
    lesson_trace::trace_step!(positive);
    lesson_trace::trace_note!("Задаём учебные значения для `negative`.");
    let negative: [i32; 4] = [0, 2, 4, 6];
    lesson_trace::trace_step!(negative);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `bad_first`.");
    let bad_first: [i32; 4] = positive;
    lesson_trace::trace_step!(bad_first);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `bad_second`.");
    let bad_second: [i32; 4] = negative;
    lesson_trace::trace_step!(bad_second);
    lesson_trace::trace_note!("Задаём учебные значения для `first_fold`.");
    let first_fold: [i32; 4] = [positive[0], positive[1], negative[0], negative[1]];
    lesson_trace::trace_step!(first_fold);
    lesson_trace::trace_note!("Задаём учебные значения для `second_fold`.");
    let second_fold: [i32; 4] = [positive[2], positive[3], negative[2], negative[3]];
    lesson_trace::trace_step!(second_fold);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, first_fold, second_fold, expected_positive) in [
        ("разбиение подряд", bad_first, bad_second, [4, 0]),
        ("стратификация", first_fold, second_fold, [2, 2]),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first_fold);
        lesson_trace::trace_step!(second_fold);
        lesson_trace::trace_step!(expected_positive);
        lesson_trace::trace_note!("Задаём учебные значения для `counts`.");
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        let counts: [usize; 2] = [
            first_fold.iter().filter(|&&value| value % 2 == 1).count(),
            second_fold.iter().filter(|&&value| value % 2 == 1).count(),
        ];
        lesson_trace::trace_step!(counts);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(counts, expected_positive);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: части {first_fold:?} и {second_fold:?}, положительных {counts:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_positive_example_count_in_each_validation_group();
}

// Строим график по результатам урока.
fn plot_positive_example_count_in_each_validation_group() {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Положительные в каждой части",
        "число",
        &[
            ("подряд: fold 1", 4.0),
            ("подряд: fold 2", 0.0),
            ("страты: fold 1", 2.0),
            ("страты: fold 2", 2.0),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
