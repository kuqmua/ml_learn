// Урок 10.2. Логарифмическая ошибка по правильному ответу и прогнозной вероятности.
//
// Уверенный правильный прогноз имеет малую ошибку; уверенный неверный — большую.
// Вероятности 0 и 1 дают бесконечную ошибку для неверного класса, поэтому пример
// считает только строго внутренние вероятности.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `cases`.
    let cases: [(&str, f64, f64); 4] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("верный уверенный прогноз", 1.0, 0.9),
        // Добавляем пару значений для сравнения или построения графика.
        ("неуверенный прогноз", 1.0, 0.5),
        // Добавляем пару значений для сравнения или построения графика.
        ("неверный уверенный прогноз", 1.0, 0.1),
        // Добавляем пару значений для сравнения или построения графика.
        ("отрицательный класс предсказан верно", 0.0, 0.1),
    ];
    lesson_trace::trace_step!(cases);
    // Задаём учебные значения для `losses`.
    let mut losses: [f64; 4] = [0.0; 4];
    lesson_trace::trace_step!(losses);
    // Повторяем расчёт для каждого элемента последовательности.
    for (index, (description, target, probability)) in cases.into_iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(target);
        lesson_trace::trace_step!(probability);
        // Проверяем ожидаемое свойство учебного примера.
        assert!(target == 0.0 || target == 1.0);
        // Проверяем ожидаемое свойство учебного примера.
        assert!(probability > 0.0 && probability < 1.0);
        // Сохраняем результат этого шага в `chosen_probability`.
        let chosen_probability: f64 = if target == 1.0 {
            // Используем подготовленное значение в следующем шаге примера.
            probability
        // Обрабатываем случай, когда предыдущее условие не выполнено.
        } else {
            // Вычисляем значение по указанной формуле.
            1.0 - probability
        };
        lesson_trace::trace_step!(chosen_probability);
        // ln(x) ≈ 2·(t+t³/3+t⁵/5+...), t=(x−1)/(x+1).
        let ratio: f64 = (chosen_probability - 1.0) / (chosen_probability + 1.0);
        lesson_trace::trace_step!(ratio);
        // Сохраняем результат этого шага в `term`.
        let mut term: f64 = ratio;
        lesson_trace::trace_step!(term);
        // Сохраняем результат этого шага в `logarithm`.
        let mut logarithm: f64 = 0.0;
        lesson_trace::trace_step!(logarithm);
        // Повторяем расчёт для каждого элемента последовательности.
        for odd_divisor in (1..=99).step_by(2) {
            lesson_trace::trace_step!(odd_divisor);
            // Обновляем значение результатом текущего вычисления.
            logarithm += term / odd_divisor as f64;
            lesson_trace::trace_step!(logarithm);
            // Обновляем значение результатом текущего вычисления.
            term *= ratio * ratio;
            lesson_trace::trace_step!(term);
        }
        // Обновляем значение результатом текущего вычисления.
        losses[index] = -2.0 * logarithm;
        lesson_trace::trace_step!(losses);
        // Печатаем рассчитанные значения для проверки примера.
        println!(
            // Передаём подпись или текстовое значение для следующего шага.
            "{description}: target={target}, вероятность={probability}, loss={:.3}",
            // Используем подготовленное значение в следующем шаге примера.
            losses[index]
        );
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_binary_logarithmic_loss_from_target_and_predicted_probability(losses);
}

// Строим график по результатам урока.
fn visualize_binary_logarithmic_loss_from_target_and_predicted_probability(losses: [f64; 4]) {
    // Проверяем ожидаемое свойство учебного примера.
    assert!(losses[0] < losses[1] && losses[1] < losses[2]);
    // Проверяем ожидаемое свойство учебного примера.
    assert!((losses[0] - losses[3]).abs() < 1e-10);
    // График величин и зависимостей, изученных в этом уроке.
    let positive_target_loss_points: Vec<(f64, f64)> = (1..100)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `probability`.
            let probability: f64 = plot_step_index as f64 / 100.0;
            // Добавляем пару значений для сравнения или построения графика.
            (probability, -probability.ln())
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Собираем значения для `negative_target_loss_points` в коллекцию.
    let negative_target_loss_points: Vec<(f64, f64)> = (1..100)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `probability`.
            let probability: f64 = plot_step_index as f64 / 100.0;
            // Добавляем пару значений для сравнения или построения графика.
            (probability, -(1.0 - probability).ln())
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
        "Логарифмическая ошибка",
        // Указываем подпись горизонтальной оси.
        "вероятность положительного класса",
        // Указываем подпись вертикальной оси.
        "ошибка",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "y=1",
                // Передаём рассчитанные координаты точек.
                points: &positive_target_loss_points,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "y=0",
                // Передаём рассчитанные координаты точек.
                points: &negative_target_loss_points,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
