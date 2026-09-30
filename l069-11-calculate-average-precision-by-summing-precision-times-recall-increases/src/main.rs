// Урок 11.6. Качество поиска положительных примеров: сумма точности, умноженной на прирост полноты.
// Связь с принятой терминологией: Площадь под кривой точности и полноты по оценкам бинарного классификатора.
// Зачем здесь эта тема: При редком положительном классе ложные положительные особенно влияют на
//   precision.
// Почему код устроен так: Меняем порог и смотрим площадь под кривой precision–recall на тех же
//   оценках.
// Представь: Для редкого класса несколько ложных тревог способны сильно снизить долю верных
//   положительных прогнозов.
//
// Что изучаем: Площадь под PR-кривой.
// Зачем это нужно: PR-AUC суммирует precision при увеличении recall и полезна при редком положительном
// классе.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Оценки уже отсортированы от большей к меньшей.");
    let ranked_labels: [bool; 4] = [true, false, true, false];
    lesson_trace::trace_step!(ranked_labels);
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `positive_count`."
    );
    let positive_count: f64 = ranked_labels.iter().filter(|&&label| label).count() as f64;
    lesson_trace::trace_step!(positive_count);
    lesson_trace::trace_note!(
        "Инициализируем изменяемый накопитель `found_positive` начальным состоянием."
    );
    let mut found_positive: f64 = 0.0;
    lesson_trace::trace_step!(found_positive);
    lesson_trace::trace_note!("Инициализируем изменяемый накопитель `area` начальным состоянием.");
    let mut area: f64 = 0.0;
    lesson_trace::trace_step!(area);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for (rank, label) in ranked_labels.into_iter().enumerate() {
        lesson_trace::trace_step!(rank);
        lesson_trace::trace_step!(label);
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if label {
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            found_positive += 1.0;
            lesson_trace::trace_step!(found_positive);
            lesson_trace::trace_note!(
                "Нормируем или усредняем величину делением и сохраняем её в `precision`."
            );
            let precision: f64 = found_positive / (rank + 1) as f64;
            lesson_trace::trace_step!(precision);
            lesson_trace::trace_note!(
                "Прибавляем очередной вклад к ранее накопленному результату."
            );
            area += precision / positive_count;
            lesson_trace::trace_step!(area);
        }
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!(
        "average calculate_positive_prediction_precision_as_true_positives_divided_by_positive_predictions (ступенчатая PR-AUC) = {area:.3}"
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_correct_positive_prediction_share_against_detected_positive_share();
}

// Строим график по результатам урока.
fn plot_correct_positive_prediction_share_against_detected_positive_share() {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    let precision_recall_area_under_curve_points: Vec<(f64, f64)> =
        [(0.0, 1.0), (0.5, 1.0), (1.0, 2.0 / 3.0)].to_vec();
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
        "PR-кривая по ранжированным меткам",
        "полнота",
        "precision",
        &[lesson_visualization::Series {
            name: "метки +−+−",

            points: &precision_recall_area_under_curve_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
