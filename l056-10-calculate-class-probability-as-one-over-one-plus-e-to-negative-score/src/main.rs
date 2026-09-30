// Урок 10.1. Вероятность класса через сигмоиду: единица, делённая на сумму единицы и e в степени, противоположной оценке модели.
// Связь с принятой терминологией: Преобразование логита в вероятность класса сигмоидой.
// Зачем здесь эта тема: Линейная сумма может быть любым числом, а для двух классов нужна величина
//   от 0 до 1.
// Почему код устроен так: Применяем сигмоиду к логиту и сравниваем значения по обе стороны нуля.
// Представь: Логит 0 даёт вероятность 0,5; положительный логит повышает её, отрицательный понижает.
//
// Отрицательный logit даёт вероятность ниже 0.5, нулевой — 0.5,
// положительный — выше 0.5. Значение всегда находится между 0 и 1.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Оценку модели до преобразования в вероятность называют logit.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    for (description, raw_model_score, expected_side) in [
        ("отрицательный", -2.0, -1),
        ("нулевой", 0.0, 0),
        ("положительный", 2.0, 1),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(raw_model_score);
        lesson_trace::trace_step!(expected_side);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `term`.");
        let mut term: f64 = 1.0;
        lesson_trace::trace_step!(term);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `exponential`.");
        let mut exponential: f64 = 1.0;
        lesson_trace::trace_step!(exponential);
        lesson_trace::trace_note!(
            "Берём 30 членов ряда Тейлора exp(−logit)=Σ(−logit)^k/k! для небольших учебных logit."
        );
        for index in 1..=30 {
            lesson_trace::trace_step!(index);
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            term *= -raw_model_score / index as f64;
            lesson_trace::trace_step!(term);
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            exponential += term;
            lesson_trace::trace_step!(exponential);
        }
        lesson_trace::trace_note!("Сохраняем результат этого шага в `probability`.");
        let probability: f64 = 1.0 / (1.0 + exponential);
        lesson_trace::trace_step!(probability);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(probability > 0.0 && probability < 1.0);
        lesson_trace::trace_note!(
            "0.5 — середина диапазона вероятностей: ниже неё знак отрицательный, выше положительный."
        );
        let side: i32 = if probability < 0.5 {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            -1
        } else if probability > 0.5 {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            1
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            0
        };
        lesson_trace::trace_step!(side);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(side, expected_side);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description} logit {raw_model_score}: вероятность {probability:.4}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_class_probability_as_one_over_one_plus_e_to_negative_score();
}

// Строим график по результатам урока.
fn plot_class_probability_as_one_over_one_plus_e_to_negative_score() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let sigmoid_points: Vec<(f64, f64)> = (-60..=60)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
            let horizontal_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (horizontal_value, 1.0 / (1.0 + (-horizontal_value).exp()))
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
        "Сигмоида",
        "логит",
        "вероятность",
        &[lesson_visualization::Series {
            name: "σ(x)",

            points: &sigmoid_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
