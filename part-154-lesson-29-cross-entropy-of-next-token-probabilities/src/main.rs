// Урок 29.2. Кросс энтропия вероятностей правильных следующих токенов.
// Зачем здесь эта тема: Чтобы обучать вероятности следующего токена, нужна ошибка по правильному
//   продолжению.
// Почему код устроен так: Берём отрицательный логарифм назначенной ему вероятности и усредняем по
//   позициям.
// Представь: Если правильному слову дали вероятность 0,9, ошибка меньше, чем при вероятности 0,1.
//
// Чем выше вероятность правильных токенов, тем меньше ошибка. Для вероятности 1
// вклад равен нулю; вероятность 0 запрещена, потому что её логарифм не определён.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `cases`.
    let cases: [(&str, &[f64]); 3] = [
        // Добавляем пару значений для сравнения или построения графика.
        ("идеальная уверенность", &[1.0, 1.0]),
        // Добавляем пару значений для сравнения или построения графика.
        ("правильные токены вероятны", &[0.8, 0.5]),
        // Добавляем пару значений для сравнения или построения графика.
        ("правильные токены маловероятны", &[0.2, 0.1]),
    ];
    lesson_trace::trace_step!(cases);
    // Сохраняем результат этого шага в `previous_error`.
    let mut previous_error: f64 = 0.0;
    lesson_trace::trace_step!(previous_error);
    // Повторяем расчёт для каждого элемента последовательности.
    for (index, (description, probabilities)) in cases.into_iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(probabilities);
        // Проверяем ожидаемое свойство учебного примера.
        assert!(!probabilities.is_empty(), "нужна хотя бы одна вероятность");
        // Проверяем ожидаемое свойство учебного примера.
        assert!(
            // Обновляем значение результатом текущего вычисления.
            probabilities.iter().all(|&p| p > 0.0 && p <= 1.0),
            // Передаём подпись или текстовое значение для следующего шага.
            "вероятность должна быть больше 0 и не больше 1"
        );
        // Сохраняем результат этого шага в `negative_log_sum`.
        let mut negative_log_sum: f64 = 0.0;
        lesson_trace::trace_step!(negative_log_sum);
        // Повторяем расчёт для каждого элемента последовательности.
        for &probability in probabilities {
            lesson_trace::trace_step!(probability);
            // Учебный аналог `f64::ln`: ряд показывает шаги вычисления, но может быть медленнее и менее точным.
            // ln(x) ≈ 2·(t+t³/3+t⁵/5+...), где t=(x−1)/(x+1).
            let ratio: f64 = (probability - 1.0) / (probability + 1.0);
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
            negative_log_sum -= 2.0 * logarithm;
            lesson_trace::trace_step!(negative_log_sum);
        }
        // Определяем размер данных и сохраняем его в `error`.
        let error: f64 = negative_log_sum / probabilities.len() as f64;
        lesson_trace::trace_step!(error);
        // Выбираем дальнейший шаг по выполнению условия.
        if index > 0 {
            // Проверяем ожидаемое свойство учебного примера.
            assert!(error > previous_error);
        }
        // Обновляем значение результатом текущего вычисления.
        previous_error = error;
        lesson_trace::trace_step!(previous_error);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: {probabilities:?} → cross-entropy {error:.3}");
    }
    // Задаём учебные значения для `invalid`.
    let invalid: [f64; 2] = [0.0, 0.5];
    lesson_trace::trace_step!(invalid);
    // Выбираем дальнейший шаг по выполнению условия.
    if invalid.iter().any(|&probability| probability <= 0.0) {
        // Печатаем рассчитанные значения для проверки примера.
        println!("нулевая вероятность правильного токена: конечную ошибку вычислить нельзя");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_cross_entropy_of_next_token_probabilities();
}

// Строим график по результатам урока.
fn visualize_cross_entropy_of_next_token_probabilities() {
    // График величин и зависимостей, изученных в этом уроке.
    // Ошибку предсказанного распределения вероятностей называют cross-entropy.
    let predicted_probability_error_points: Vec<(f64, f64)> = (1..=100)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `probability`.
            let probability: f64 = plot_step_index as f64 / 100.0;
            // Добавляем пару значений для сравнения или построения графика.
            (probability, -probability.ln())
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
        "Cross-entropy правильного токена",
        // Указываем подпись горизонтальной оси.
        "вероятность",
        // Указываем подпись вертикальной оси.
        "ошибка",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "-ln(p)",
            // Передаём рассчитанные координаты точек.
            points: &predicted_probability_error_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
