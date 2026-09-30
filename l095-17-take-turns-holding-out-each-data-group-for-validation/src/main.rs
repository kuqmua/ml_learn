// Урок 17.1. Поочерёдное выделение каждой группы данных для проверки модели.
// Связь с принятой терминологией: Поочерёдный выбор каждого блока данных для валидации.
// Зачем здесь эта тема: Один train/validation split может дать случайно удачную оценку.
// Почему код устроен так: По очереди делаем каждый блок проверочным и усредняем результаты.
// Представь: При трёх блоках каждый один раз становится проверочным, а два других служат обучением.
//
// Что изучаем: K-fold кросс-валидация.
// Зачем это нужно: Каждый блок данных один раз становится проверочным, пока остальные используются для
// обучения.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `rows` для следующего шага примера.");
    let rows: [i32; 6] = [0, 1, 2, 3, 4, 5];
    lesson_trace::trace_step!(rows);
    lesson_trace::trace_note!("Сохраняем рассчитанное значение `folds` для следующих операций.");
    let folds: i32 = 3;
    lesson_trace::trace_step!(folds);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for fold in 0..folds {
        lesson_trace::trace_step!(fold);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `validation` для следующих операций."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!(
            "Копируем значения из ссылок, чтобы получить самостоятельные элементы."
        );
        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        let validation: Vec<i32> = rows
            .iter()
            .copied()
            .filter(|&row| row % folds == fold)
            .collect();
        lesson_trace::trace_step!(validation);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `training_data` для следующих операций."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!(
            "Копируем значения из ссылок, чтобы получить самостоятельные элементы."
        );
        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        let training_data: Vec<i32> = rows
            .iter()
            .copied()
            .filter(|&row| row % folds != fold)
            .collect();
        lesson_trace::trace_step!(training_data);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("fold={fold}: train={training_data:?}, validation={validation:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_validation_group_assigned_to_each_row();
}

// Строим график по результатам урока.
fn plot_validation_group_assigned_to_each_row() {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    let cross_validation_fold_points: Vec<(f64, f64)> = (0..9)
        .map(|plot_step_index| (plot_step_index as f64, (plot_step_index % 3) as f64))
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "K-fold: номер fold для строки",
        "номер строки",
        "fold",
        &[lesson_visualization::Series {
            name: "3 части",

            points: &cross_validation_fold_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
