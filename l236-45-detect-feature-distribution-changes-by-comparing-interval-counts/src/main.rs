// Урок 45.1. Обнаружение изменений данных: сравнение числа значений в интервалах признака.
// Связь с принятой терминологией: Сравнение распределений признака до и после выпуска модели.
// Зачем здесь эта тема: После выпуска реальные признаки могут уйти от обучающего распределения.
// Почему код устроен так: Сравниваем доли одинаковых диапазонов признака в опорном и текущем
//   наборе.
// Представь: Если раньше большинство значений было ниже 0,5, а теперь выше, доли двух корзин
//   поменяются.
//
// Считаем значения ниже 0.5 и значения от 0.5. Сравнение старой и новой выборки
// показывает, переместилась ли масса распределения между интервалами.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 4], [i32; 2]); 3] = [
        ("эталон", [0.1, 0.2, 0.8, 0.9], [2, 2]),
        ("без сдвига", [0.2, 0.3, 0.7, 0.8], [2, 2]),
        ("сдвиг к большим значениям", [0.6, 0.7, 0.8, 0.9], [0, 4]),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, values, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(values);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Задаём учебные значения для `bins`.");
        let mut bins: [i32; 2] = [0; 2];
        lesson_trace::trace_step!(bins);
        lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for value in values {
            lesson_trace::trace_step!(value);
            lesson_trace::trace_note!(
                "0.5 — выбранная граница двух интервалов: [0, 0.5) и [0.5, 1]."
            );
            let index: usize = if value < 0.5 { 0 } else { 1 };
            lesson_trace::trace_step!(index);
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            bins[index] += 1;
            lesson_trace::trace_step!(bins);
        }
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(bins, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {values:?} → частоты {bins:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_observation_counts_in_feature_intervals();
}

// Строим график по результатам урока.
fn plot_observation_counts_in_feature_intervals() {
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
        "Распределение признака",
        "число наблюдений",
        &[
            ("до: <0.5", 2.0),
            ("до: ≥0.5", 2.0),
            ("после: <0.5", 0.0),
            ("после: ≥0.5", 4.0),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
