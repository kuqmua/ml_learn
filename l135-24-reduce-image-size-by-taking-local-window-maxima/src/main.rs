// Урок 24.4. Уменьшение изображения: выбор максимума в каждом локальном окне.
// Связь с принятой терминологией: Выбор максимума в локальных окнах изображения.
// Зачем здесь эта тема: После фильтра соседние сильные отклики можно свести к одному признаку.
// Почему код устроен так: Берём максимум в каждом локальном окне и показываем потерю точного
//   положения.
// Представь: Из окна значений [1, 3, 2, 0] max pooling оставит 3 и забудет точное место этого пика.
//
// Что изучаем: Max pooling.
// Зачем это нужно: Pooling оставляет наиболее сильный отклик в локальном окне и уменьшает пространственный
// размер.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `feature_map` для следующего шага примера.");
    let feature_map: [[f64; 2]; 2] = [[1.0, 4.0], [3.0, 2.0]];
    lesson_trace::trace_step!(feature_map);
    lesson_trace::trace_note!("Создаём изменяемое значение `maximum` для следующих операций.");
    lesson_trace::trace_note!("Для выбора максимума карта должна содержать хотя бы одно значение.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !feature_map.is_empty() && !feature_map[0].is_empty(),
        "карта признаков не должна быть пустой"
    );
    lesson_trace::trace_note!("Сохраняем результат этого шага в `maximum`.");
    let mut maximum: f64 = feature_map[0][0];
    lesson_trace::trace_step!(maximum);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for row in feature_map {
        lesson_trace::trace_step!(row);
        lesson_trace::trace_note!(
            "Повторяем следующий блок для каждого элемента указанной последовательности."
        );
        for value in row {
            lesson_trace::trace_step!(value);
            lesson_trace::trace_note!(
                "Проверяем условие и выбираем соответствующую ветку алгоритма."
            );
            if value > maximum {
                lesson_trace::trace_note!(
                    "Присваиваем вычисленное значение соответствующей переменной или полю."
                );
                maximum = value;
                lesson_trace::trace_step!(maximum);
            }
        }
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("max pooling = {maximum}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_largest_values_in_local_image_windows(feature_map, maximum);
}

// Строим график по результатам урока.
fn plot_largest_values_in_local_image_windows(feature_map: [[f64; 2]; 2], maximum: f64) {
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
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Max pooling",
        "значение",
        &[
            ("1", feature_map[0][0]),
            ("2", feature_map[0][1]),
            ("3", feature_map[1][0]),
            ("4", feature_map[1][1]),
            ("максимум", maximum),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
