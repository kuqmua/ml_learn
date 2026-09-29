// Урок 17.2. Стратифицированные блоки с сохранением долей классов.
// Почему этот урок сейчас: При редком классе случайный блок может остаться без его примеров и исказить метрику.
// Почему пример устроен так: Распределяем метки по блокам так, чтобы доли классов были близки к исходным.
//
// Если разложить упорядоченные по классу данные подряд, в одной части может оказаться
// только положительный класс, в другой — только отрицательный. Стратификация смешивает классы.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `positive`.
    let positive: [i32; 4] = [1, 3, 5, 7];
    lesson_trace::trace_step!(positive);
    // Задаём учебные значения для `negative`.
    let negative: [i32; 4] = [0, 2, 4, 6];
    lesson_trace::trace_step!(negative);
    // Сохраняем результат этого шага в `bad_first`.
    let bad_first: [i32; 4] = positive;
    lesson_trace::trace_step!(bad_first);
    // Сохраняем результат этого шага в `bad_second`.
    let bad_second: [i32; 4] = negative;
    lesson_trace::trace_step!(bad_second);
    // Задаём учебные значения для `first_fold`.
    let first_fold: [i32; 4] = [positive[0], positive[1], negative[0], negative[1]];
    lesson_trace::trace_step!(first_fold);
    // Задаём учебные значения для `second_fold`.
    let second_fold: [i32; 4] = [positive[2], positive[3], negative[2], negative[3]];
    lesson_trace::trace_step!(second_fold);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, first_fold, second_fold, expected_positive) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("разбиение подряд", bad_first, bad_second, [4, 0]),
        // Добавляем пару значений для сравнения или построения графика.
        ("стратификация", first_fold, second_fold, [2, 2]),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first_fold);
        lesson_trace::trace_step!(second_fold);
        lesson_trace::trace_step!(expected_positive);
        // Задаём учебные значения для `counts`.
        let counts: [usize; 2] = [
            // Обновляем значение результатом текущего вычисления.
            first_fold.iter().filter(|&&value| value % 2 == 1).count(),
            // Обновляем значение результатом текущего вычисления.
            second_fold.iter().filter(|&&value| value % 2 == 1).count(),
        ];
        lesson_trace::trace_step!(counts);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(counts, expected_positive);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: части {first_fold:?} и {second_fold:?}, положительных {counts:?}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_stratified_folds_preserving_class_proportions();
}

// Строим график по результатам урока.
fn visualize_stratified_folds_preserving_class_proportions() {
    // Сравниваем величины, вычисленные в примере.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Положительные в каждой части",
        // Указываем подпись вертикальной оси.
        "число",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("подряд: fold 1", 4.0),
            // Добавляем пару значений для сравнения или построения графика.
            ("подряд: fold 2", 0.0),
            // Добавляем пару значений для сравнения или построения графика.
            ("страты: fold 1", 2.0),
            // Добавляем пару значений для сравнения или построения графика.
            ("страты: fold 2", 2.0),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
