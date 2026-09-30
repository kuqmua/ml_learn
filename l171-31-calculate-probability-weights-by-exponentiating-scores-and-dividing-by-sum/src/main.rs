// Урок 31.5. Веса вероятностей (softmax): вычисление экспонент оценок и деление каждой на их сумму.
// Связь с принятой терминологией: Преобразование оценок внимания в веса с помощью softmax.
// Зачем здесь эта тема: Сырые оценки внимания могут быть отрицательными и не суммируются в единицу.
// Почему код устроен так: Применяем softmax, чтобы получить неотрицательные веса с суммой один.
// Представь: Для оценок [2, 1] softmax даёт два положительных веса, которые вместе составляют 1.
//
// Большая оценка получает больший вес. Равные оценки дают равные веса.
// Прибавление одной константы к обеим оценкам не меняет веса, а сумма весов равна 1.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2]); 4] = [
        ("равные оценки", [0.0, 0.0]),
        ("вторая оценка выше", [0.0, 1.0]),
        ("первая оценка выше", [1.0, 0.0]),
        ("к обеим прибавили 1", [1.0, 2.0]),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Задаём учебные значения для `reference_weights`.");
    let mut reference_weights: [f64; 2] = [0.0; 2];
    lesson_trace::trace_step!(reference_weights);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    lesson_trace::trace_note!("Оценку модели до преобразования в вероятность называют logit.");
    for (description, raw_model_scores) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(raw_model_scores);
        lesson_trace::trace_note!("Задаём учебные значения для `exponentials`.");
        let mut exponentials: [f64; 2] = [0.0; 2];
        lesson_trace::trace_step!(exponentials);
        lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for index in 0..2 {
            lesson_trace::trace_step!(index);
            lesson_trace::trace_note!(
                "Считаем exp(x) первыми 30 членами ряда Тейлора для малых учебных оценок."
            );
            let mut term: f64 = 1.0;
            lesson_trace::trace_step!(term);
            lesson_trace::trace_note!("Сохраняем результат этого шага в `sum`.");
            let mut sum: f64 = 1.0;
            lesson_trace::trace_step!(sum);
            lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
            lesson_trace::trace_note!(
                "30 членов ряда Σx^k/k! приближают exp(score−max_score) для softmax."
            );
            for order in 1..=30 {
                lesson_trace::trace_step!(order);
                lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                term *= raw_model_scores[index] / order as f64;
                lesson_trace::trace_step!(term);
                lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                sum += term;
                lesson_trace::trace_step!(sum);
            }
            lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
            exponentials[index] = sum;
            lesson_trace::trace_step!(exponentials);
        }
        lesson_trace::trace_note!("Сохраняем результат этого шага в `denominator`.");
        let denominator: f64 = exponentials[0] + exponentials[1];
        lesson_trace::trace_step!(denominator);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            denominator > 0.0,
            "сумма экспонент должна быть положительной"
        );
        lesson_trace::trace_note!("Задаём учебные значения для `weights`.");
        let weights: [f64; 2] = [exponentials[0] / denominator, exponentials[1] / denominator];
        lesson_trace::trace_step!(weights);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((weights[0] + weights[1] - 1.0).abs() < 1e-10);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(weights.iter().all(|&weight| (0.0..=1.0).contains(&weight)));
        lesson_trace::trace_note!("Разбираем результат по его возможным вариантам.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        lesson_trace::trace_note!("Выполняем действие для этого варианта данных.");
        match description {
            "равные оценки" => assert!((weights[0] - weights[1]).abs() < 1e-10),

            "вторая оценка выше" => {
                lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
                assert!(weights[1] > weights[0]);
                lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                reference_weights = weights;
                lesson_trace::trace_step!(reference_weights);
            }

            "первая оценка выше" => assert!(weights[0] > weights[1]),

            "к обеим прибавили 1" => {
                lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
                assert!((weights[0] - reference_weights[0]).abs() < 1e-10);
                lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
                assert!((weights[1] - reference_weights[1]).abs() < 1e-10);
            }

            _ => unreachable!(),
        }
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {raw_model_scores:?} → {weights:?}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_second_probability_weight_as_its_exponential_divided_by_sum_of_two_exponentials();
}

// Строим график по результатам урока.
fn plot_second_probability_weight_as_its_exponential_divided_by_sum_of_two_exponentials() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразование оценок в вероятности с суммой 1 называют softmax.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let normalized_probability_points: Vec<(f64, f64)> = (-60..=60)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `distance_value`.");
            let distance_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (distance_value, 1.0 / (1.0 + (-distance_value).exp()))
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
        "Softmax двух логитов",
        "разность второго и первого",
        "вес второго",
        &[lesson_visualization::Series {
            name: "softmax",

            points: &normalized_probability_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
