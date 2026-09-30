// Урок 29.3. Неопределённость языковой модели (перплексия): e в степени среднего отрицательного логарифма вероятности.
// Связь с принятой терминологией: Perplexity из средней кросс энтропии языковой модели.
// Зачем здесь эта тема: Среднюю кросс энтропию трудно читать как число вариантов продолжения.
// Почему код устроен так: Возводим e в среднюю ошибку и получаем perplexity на той же
//   последовательности.
// Представь: Одинаковая средняя ошибка может читаться как эффективное число возможных продолжений
//   через exp.
//
// Что изучаем: Perplexity.
// Зачем это нужно: Perplexity — экспонента средней cross-entropy; меньшая величина означает лучшее
// вероятностное предсказание.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Инициализируем значение `predicted_probability_error` начальным состоянием."
    );
    lesson_trace::trace_note!(
        "Ошибку предсказанного распределения вероятностей называют cross-entropy."
    );
    let predicted_probability_error: f64 = 0.7;
    lesson_trace::trace_step!(predicted_probability_error);
    lesson_trace::trace_note!("Создаём изменяемое значение `term` для следующих операций.");
    let mut term: f64 = 1.0;
    lesson_trace::trace_step!(term);
    lesson_trace::trace_note!(
        "Создаём изменяемое значение `effective_choice_count` для следующих операций."
    );
    lesson_trace::trace_note!(
        "Эффективное число вариантов, соответствующее ошибке языковой модели, называют perplexity."
    );
    let mut effective_choice_count: f64 = 1.0;
    lesson_trace::trace_step!(effective_choice_count);
    lesson_trace::trace_note!(
        "Perplexity = exp(cross-entropy); 30 членов ряда Σx^k/k! приближают exp(0.7)."
    );
    for order in 1..=30 {
        lesson_trace::trace_step!(order);
        lesson_trace::trace_note!("Умножаем накопленное значение на очередной множитель.");
        term *= predicted_probability_error / order as f64;
        lesson_trace::trace_step!(term);
        lesson_trace::trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        effective_choice_count += term;
        lesson_trace::trace_step!(effective_choice_count);
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("perplexity={effective_choice_count:.3}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_perplexity_as_e_to_average_negative_log_probability();
}

// Строим график по результатам урока.
fn plot_perplexity_as_e_to_average_negative_log_probability() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let effective_choice_count_points: Vec<(f64, f64)> = (0..=40)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `loss_value`.");
            let loss_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (loss_value, loss_value.exp())
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
        "Perplexity",
        "cross-entropy",
        "perplexity",
        &[lesson_visualization::Series {
            name: "exp(loss)",

            points: &effective_choice_count_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
