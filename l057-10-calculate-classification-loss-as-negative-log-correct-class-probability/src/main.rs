// Урок 10.2. Ошибка классификации: отрицательный логарифм вероятности правильного класса.
// Связь с принятой терминологией: Логарифмическая ошибка по правильному ответу и прогнозной вероятности.
// Зачем здесь эта тема: Для обучения вероятностного классификатора нужна ошибка, сильно штрафующая
//   уверенный неверный ответ.
// Почему код устроен так: Берём логарифм вероятности правильного класса и явно проверяем поведение
//   у границ 0 и 1.
// Представь: Прогноз 0,99 для неверного класса должен штрафоваться сильнее, чем неуверенный прогноз
//   0,55.
//
// Уверенный правильный прогноз имеет малую ошибку; уверенный неверный — большую.
// Вероятности 0 и 1 дают бесконечную ошибку для неверного класса, поэтому пример
// считает только строго внутренние вероятности.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, f64, f64); 4] = [
        ("верный уверенный прогноз", 1.0, 0.9),
        ("неуверенный прогноз", 1.0, 0.5),
        ("неверный уверенный прогноз", 1.0, 0.1),
        ("отрицательный класс предсказан верно", 0.0, 0.1),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Задаём учебные значения для `losses`.");
    let mut losses: [f64; 4] = [0.0; 4];
    lesson_trace::trace_step!(losses);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (index, (description, target, probability)) in cases.into_iter().enumerate() {
        lesson_trace::trace_step!(index);
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(target);
        lesson_trace::trace_step!(probability);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(target == 0.0 || target == 1.0);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(probability > 0.0 && probability < 1.0);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `chosen_probability`.");
        let chosen_probability: f64 = if target == 1.0 {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            probability
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!("Вычисляем значение по указанной формуле.");
            1.0 - probability
        };
        lesson_trace::trace_step!(chosen_probability);
        lesson_trace::trace_note!(
            "Учебный аналог `f64::ln`: ряд показывает шаги вычисления, но может быть медленнее и менее точным."
        );
        lesson_trace::trace_note!("ln(x) ≈ 2·(t+t³/3+t⁵/5+...), t=(x−1)/(x+1).");
        let ratio: f64 = (chosen_probability - 1.0) / (chosen_probability + 1.0);
        lesson_trace::trace_step!(ratio);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `term`.");
        let mut term: f64 = ratio;
        lesson_trace::trace_step!(term);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `logarithm`.");
        let mut logarithm: f64 = 0.0;
        lesson_trace::trace_step!(logarithm);
        lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for odd_divisor in (1..=99).step_by(2) {
            lesson_trace::trace_step!(odd_divisor);
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            logarithm += term / odd_divisor as f64;
            lesson_trace::trace_step!(logarithm);
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            term *= ratio * ratio;
            lesson_trace::trace_step!(term);
        }
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        losses[index] = -2.0 * logarithm;
        lesson_trace::trace_step!(losses);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        println!(
            "{description}: target={target}, вероятность={probability}, loss={:.3}",
            losses[index]
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_classification_loss_as_negative_log_probability_for_each_correct_class(losses);
}

// Строим график по результатам урока.
fn plot_classification_loss_as_negative_log_probability_for_each_correct_class(losses: [f64; 4]) {
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(losses[0] < losses[1] && losses[1] < losses[2]);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!((losses[0] - losses[3]).abs() < 1e-10);
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let positive_target_loss_points: Vec<(f64, f64)> = (1..100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (probability, -probability.ln())
        })
        .collect();
    lesson_trace::trace_note!("Собираем значения для `negative_target_loss_points` в коллекцию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let negative_target_loss_points: Vec<(f64, f64)> = (1..100)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
            let probability: f64 = plot_step_index as f64 / 100.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (probability, -(1.0 - probability).ln())
        })
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Логарифмическая ошибка",
        "вероятность положительного класса",
        "ошибка",
        &[
            lesson_visualization::Series {
                name: "y=1",

                points: &positive_target_loss_points,
            },
            lesson_visualization::Series {
                name: "y=0",

                points: &negative_target_loss_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
