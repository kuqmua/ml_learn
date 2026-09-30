// Урок 10.4. Выбор класса сравнением вероятности с порогом.
// Связь с принятой терминологией: Преобразование прогнозной вероятности в класс по порогу.
// Зачем здесь эта тема: Вероятность ещё не является меткой класса; решение зависит от выбранного
//   порога.
// Почему код устроен так: Сравниваем одну вероятность с порогом, чтобы увидеть смену класса без
//   переобучения модели.
// Представь: Вероятность 0,6 станет положительным классом при пороге 0,5, но отрицательным при
//   пороге 0,7.
//
// Что изучаем: Порог классификации.
// Зачем это нужно: Порог превращает вероятность в метку класса. Более низкий порог обычно даёт больше
// положительных прогнозов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Создаём набор значений `probabilities` для следующего шага примера."
    );
    let probabilities: [f64; 3] = [0.2, 0.55, 0.8];
    lesson_trace::trace_step!(probabilities);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for threshold in [0.5, 0.7] {
        lesson_trace::trace_step!(threshold);
        lesson_trace::trace_note!(
            "Преобразуем входные данные и сохраняем полученную коллекцию в `predictions`."
        );
        let predictions: Vec<bool> = probabilities
            .iter()
            .map(|&probability| probability >= threshold)
            .collect();
        lesson_trace::trace_step!(predictions);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("порог {threshold}: {predictions:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_number_of_positive_predictions_for_changing_threshold();
}

// Строим график по результатам урока.
fn plot_number_of_positive_predictions_for_changing_threshold() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let threshold_points: Vec<(f64, f64)> = (0..=100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `threshold_value`.");
            let threshold_value: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!("Задаём значения следующей строки или последовательности.");
            (
                threshold_value,
                [0.2, 0.55, 0.8]
                    .iter()
                    .filter(|&&probability| probability >= threshold_value)
                    .count() as f64,
            )
        })
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
        "Число положительных прогнозов",
        "порог",
        "количество",
        &[lesson_visualization::Series {
            name: "оценки 0.2, 0.55, 0.8",

            points: &threshold_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
