// Урок 06.2. Медиана: поиск среднего по положению значения после сортировки.
// Связь с принятой терминологией: Медиана числовых значений.
// Зачем здесь эта тема: Среднее чувствительно к выбросу; медиана показывает центральное значение
//   после упорядочивания.
// Почему код устроен так: Сортируем малый набор и рассматриваем нечётное и чётное число элементов.
// Представь: Для 2, 4, 100 медиана равна 4: крайнее число не сдвигает центральное место.
//
// У нечётного набора берём средний элемент, у чётного — среднее двух центральных.
// Выброс меняет среднее арифметическое гораздо сильнее, чем медиану.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, &[f64], f64); 4] = [
        ("нечётное число значений", &[2.0, 4.0, 6.0], 4.0),
        ("чётное число значений", &[2.0, 4.0, 6.0, 8.0], 5.0),
        ("сильный выброс", &[2.0, 4.0, 6.0, 100.0], 5.0),
        ("одно значение", &[7.0], 7.0),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, source, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(source);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(!source.is_empty(), "медиана пустого набора не определена");
        lesson_trace::trace_note!("Сохраняем результат этого шага в `values`.");
        let mut values: Vec<f64> = source.to_vec();
        lesson_trace::trace_step!(values);
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        values.sort_by(f64::total_cmp);
        lesson_trace::trace_note!("Определяем размер данных и сохраняем его в `middle`.");
        let middle: usize = values.len() / 2;
        lesson_trace::trace_step!(middle);
        lesson_trace::trace_note!("Определяем размер данных и сохраняем его в `median`.");
        let median: f64 = if values.len() % 2 == 0 {
            lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
            (values[middle - 1] + values[middle]) / 2.0
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            values[middle]
        };
        lesson_trace::trace_step!(median);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(median, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {values:?} → медиана {median}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_values_and_median_as_middle_of_sorted_values();
}

// Строим график по результатам урока.
fn plot_values_and_median_as_middle_of_sorted_values() {
    lesson_trace::trace_note!("Наглядное представление величин из этого урока.");
    let observation_points: Vec<(f64, f64)> = [(1.0, 1.0), (2.0, 3.0), (3.0, 7.0)].to_vec();
    lesson_trace::trace_note!("Собираем значения для `median_points` в коллекцию.");
    let median_points: Vec<(f64, f64)> = [(1.0, 3.0), (3.0, 3.0)].to_vec();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Медиана и отдельные значения",
        "индекс",
        "значение",
        &[
            lesson_visualization::Series {
                name: "наблюдения",

                points: &observation_points,
            },
            lesson_visualization::Series {
                name: "медиана",

                points: &median_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
