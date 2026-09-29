// Урок 11.4. Удвоенное произведение точности положительных прогнозов и полноты, делённое на их сумму.
// Связь с принятой терминологией: Гармоническое среднее точности и полноты бинарной классификации.
// Зачем здесь эта тема: Precision и recall могут расходиться; F1 сводит их в число, чувствительное
//   к меньшему из двух.
// Почему код устроен так: Используем гармоническое среднее и рассматриваем нулевые знаменатели.
// Представь: Если precision высокий, а recall низкий, F1 не позволит одному хорошему числу скрыть
//   другое.
//
// Объединяем precision и recall из двух предыдущих уроков.
// Если обе равны нулю, формула даёт 0/0, поэтому возвращаем None.

fn main() {
    lesson_trace::enable();
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, precision, recall, expected) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        // Добавляем пару значений для сравнения или построения графика.
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(precision);
        lesson_trace::trace_step!(recall);
        lesson_trace::trace_step!(expected);
        // Сохраняем результат этого шага в `twice_precision_times_recall_divided_by_their_sum`.
        let harmonic_mean_score: Option<f64> = part_063_lesson_11_twice_precision_times_recall_divided_by_their_sum::twice_precision_times_recall_divided_by_their_sum(
            Some(precision),
            Some(recall),
        );
        lesson_trace::trace_step!(harmonic_mean_score);
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(harmonic_mean_score, expected);
        // Печатаем рассчитанные значения для проверки примера.
        println!(
            "{description}: precision={precision}, recall={recall}, F1={harmonic_mean_score:?}"
        );
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_twice_precision_times_recall_over_their_sum();
}

// Строим график по результатам урока.
fn plot_twice_precision_times_recall_over_their_sum() {
    // График величин и зависимостей, изученных в этом уроке.
    let harmonic_mean_score_points: Vec<(f64, f64)> = (0..=100)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `recall_value`.
            let recall_value: f64 = plot_step_index as f64 / 100.0;
            (
                // Используем подготовленное значение в следующем шаге примера.
                recall_value,
                // Выбираем дальнейший шаг по выполнению условия.
                if recall_value == 0.0 {
                    // Используем подготовленное значение в следующем шаге примера.
                    0.0
                // Обрабатываем случай, когда предыдущее условие не выполнено.
                } else {
                    // Вычисляем значение по указанной формуле.
                    2.0 * 0.8 * recall_value / (0.8 + recall_value)
                },
            )
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
        "F1 при precision=0.8",
        // Указываем подпись горизонтальной оси.
        "recall",
        // Указываем подпись вертикальной оси.
        "F1",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "F1",
            // Передаём рассчитанные координаты точек.
            points: &harmonic_mean_score_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
