// Урок 31.6. Запрет использования будущих позиций текста при сборе контекста.
// Связь с принятой терминологией: Запрет внимания к будущим токенам причинной маской.
// Зачем здесь эта тема: При генерации токена модель не должна видеть продолжение, которое ещё
//   предстоит предсказать.
// Почему код устроен так: Закрываем будущие позиции до softmax, иначе их значения утекут в текущий
//   выход.
// Представь: Предсказывая третье слово, нельзя смотреть на четвёртое: иначе ответ уже подсказан.
//
// На позиции 0 виден только первый токен, на позиции 1 — первые два,
// на последней позиции — все. Вес будущих позиций всегда равен нулю.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `raw_weights`.");
    let raw_weights: [f64; 3] = [0.2, 0.3, 0.5];
    lesson_trace::trace_step!(raw_weights);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for current_position in 0..raw_weights.len() {
        lesson_trace::trace_step!(current_position);
        lesson_trace::trace_note!(
            "Задаём учебные значения для `future_position_filtered_weights`."
        );
        lesson_trace::trace_note!("Запрет внимания к будущим позициям называют causal mask.");
        let mut future_position_filtered_weights: [f64; 3] = [0.0; 3];
        lesson_trace::trace_step!(future_position_filtered_weights);
        lesson_trace::trace_note!("Вычисляем `allowed_sum` по элементам исходной коллекции.");
        let allowed_sum: f64 = raw_weights[..=current_position].iter().sum();
        lesson_trace::trace_step!(allowed_sum);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            allowed_sum > 0.0,
            "доступные позиции должны иметь положительную сумму весов"
        );
        lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for index in 0..=current_position {
            lesson_trace::trace_step!(index);
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            future_position_filtered_weights[index] = raw_weights[index] / allowed_sum;
            lesson_trace::trace_step!(future_position_filtered_weights);
        }
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((future_position_filtered_weights.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
        lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
        lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
        assert!(
            future_position_filtered_weights[current_position + 1..]
                .iter()
                .all(|&weight| weight == 0.0)
        );
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("позиция {current_position}: веса {future_position_filtered_weights:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_allowed_current_and_past_position_pairs(raw_weights);
}

// Строим график по результатам урока.
fn plot_allowed_current_and_past_position_pairs(raw_weights: [f64; 3]) {
    lesson_trace::trace_note!("Каждая строка показывает допустимые ключи для текущей позиции.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let weights: Vec<Vec<f64>> = (0..raw_weights.len())
        .map(|position| {
            lesson_trace::trace_note!("Вычисляем `total` по элементам исходной коллекции.");
            let total: f64 = raw_weights[..=position].iter().sum();
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
            lesson_trace::trace_note!("Собираем результаты в коллекцию.");
            (0..raw_weights.len())
                .map(|key| {
                    lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
                    if key <= position {
                        lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
                        raw_weights[key] / total
                    } else {
                        lesson_trace::trace_note!(
                            "Обрабатываем случай, когда предыдущее условие не выполнено."
                        );
                        lesson_trace::trace_note!(
                            "Используем подготовленное значение в следующем шаге примера."
                        );
                        0.0
                    }
                })
                .collect()
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок тепловой карты.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::heatmap(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Причинная маска внимания",
        &weights,
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
