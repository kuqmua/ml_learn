// Урок 29.3. Perplexity из средней кросс энтропии языковой модели.
//
// Что изучаем: Perplexity.
// Зачем это нужно: Perplexity — экспонента средней cross-entropy; меньшая величина означает лучшее
// вероятностное предсказание.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Инициализируем значение `predicted_probability_error` начальным состоянием.
    // Ошибку предсказанного распределения вероятностей называют cross-entropy.
    let predicted_probability_error: f64 = 0.7;
    lesson_trace::trace_step!(predicted_probability_error);
    // Создаём изменяемое значение `term` для следующих операций.
    let mut term: f64 = 1.0;
    lesson_trace::trace_step!(term);
    // Создаём изменяемое значение `effective_choice_count` для следующих операций.
    // Эффективное число вариантов, соответствующее ошибке языковой модели, называют perplexity.
    let mut effective_choice_count: f64 = 1.0;
    lesson_trace::trace_step!(effective_choice_count);
    // Perplexity = exp(cross-entropy); 30 членов ряда Σx^k/k! приближают exp(0.7).
    for order in 1..=30 {
        lesson_trace::trace_step!(order);
        // Умножаем накопленное значение на очередной множитель.
        term *= predicted_probability_error / order as f64;
        lesson_trace::trace_step!(term);
        // Прибавляем очередной вклад к ранее накопленному результату.
        effective_choice_count += term;
        lesson_trace::trace_step!(effective_choice_count);
    }
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("perplexity={effective_choice_count:.3}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_perplexity_from_average_language_model_cross_entropy();
}

// Строим график по результатам урока.
fn visualize_perplexity_from_average_language_model_cross_entropy() {
    // График величин и зависимостей, изученных в этом уроке.
    let effective_choice_count_points: Vec<(f64, f64)> = (0..=40)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `loss_value`.
            let loss_value: f64 = plot_step_index as f64 / 10.0;
            // Добавляем пару значений для сравнения или построения графика.
            (loss_value, loss_value.exp())
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Perplexity",
        // Указываем подпись горизонтальной оси.
        "cross-entropy",
        // Указываем подпись вертикальной оси.
        "perplexity",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "exp(loss)",
            // Передаём рассчитанные координаты точек.
            points: &effective_choice_count_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
