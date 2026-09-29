// Урок 11.6. Площадь под кривой точности и полноты по оценкам бинарного классификатора.
//
// Что изучаем: Площадь под PR-кривой.
// Зачем это нужно: PR-AUC суммирует precision при увеличении recall и полезна при редком положительном
// классе.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Оценки уже отсортированы от большей к меньшей.
    let ranked_labels: [bool; 4] = [true, false, true, false];
    lesson_trace::trace_step!(ranked_labels);
    // Преобразуем входные данные и сохраняем полученную коллекцию в `positive_count`.
    let positive_count: f64 = ranked_labels.iter().filter(|&&label| label).count() as f64;
    lesson_trace::trace_step!(positive_count);
    // Инициализируем изменяемый накопитель `found_positive` начальным состоянием.
    let mut found_positive: f64 = 0.0;
    lesson_trace::trace_step!(found_positive);
    // Инициализируем изменяемый накопитель `area` начальным состоянием.
    let mut area: f64 = 0.0;
    lesson_trace::trace_step!(area);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for (rank, label) in ranked_labels.into_iter().enumerate() {
        lesson_trace::trace_step!(rank);
        lesson_trace::trace_step!(label);
        // Проверяем условие и выбираем соответствующую ветку алгоритма.
        if label {
            // Прибавляем очередной вклад к ранее накопленному результату.
            found_positive += 1.0;
            lesson_trace::trace_step!(found_positive);
            // Нормируем или усредняем величину делением и сохраняем её в `precision`.
            let precision: f64 = found_positive / (rank + 1) as f64;
            lesson_trace::trace_step!(precision);
            // Прибавляем очередной вклад к ранее накопленному результату.
            area += precision / positive_count;
            lesson_trace::trace_step!(area);
        }
    }
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        "average precision_from_binary_classification_counts (ступенчатая PR-AUC) = {area:.3}"
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_area_under_precision_recall_curve_for_binary_scores();
}

// Строим график по результатам урока.
fn visualize_area_under_precision_recall_curve_for_binary_scores() {
    // Значения из этого урока на графике.
    let precision_recall_area_under_curve_points: Vec<(f64, f64)> =
        [(0.0, 1.0), (0.5, 1.0), (1.0, 2.0 / 3.0)].to_vec();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "PR-кривая по ранжированным меткам",
        // Указываем подпись горизонтальной оси.
        "полнота",
        // Указываем подпись вертикальной оси.
        "precision",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "метки +−+−",
            // Передаём рассчитанные координаты точек.
            points: &precision_recall_area_under_curve_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
