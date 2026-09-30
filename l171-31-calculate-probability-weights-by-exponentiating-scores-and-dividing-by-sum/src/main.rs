// Урок 31.5. Веса вероятностей (softmax): вычисление экспонент оценок и деление каждой на их сумму.
// Связь с принятой терминологией: Преобразование оценок внимания в веса с помощью softmax.
// Зачем здесь эта тема: Сырые оценки внимания могут быть отрицательными и не суммируются в единицу.
// Почему код устроен так: Применяем softmax, чтобы получить неотрицательные веса с суммой один.
// Представь: Для оценок [2, 1] softmax даёт два положительных веса, которые вместе составляют 1.
//
// Большая оценка получает больший вес. Равные оценки дают равные веса.
// Прибавление одной константы к обеим оценкам не меняет веса, а сумма весов равна 1.

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2]); 4] = [
        ("равные оценки", [0.0, 0.0]),
        ("вторая оценка выше", [0.0, 1.0]),
        ("первая оценка выше", [1.0, 0.0]),
        ("к обеим прибавили 1", [1.0, 2.0]),
    ];
    trace_step!(cases);
    trace_note!("Задаём учебные значения для `reference_weights`.");
    let mut reference_weights: [f64; 2] = [0.0; 2];
    trace_step!(reference_weights);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    trace_note!("Оценку модели до преобразования в вероятность называют logit.");
    for (description, raw_model_scores) in cases {
        trace_step!(description);
        trace_step!(raw_model_scores);
        trace_note!("Задаём учебные значения для `exponentials`.");
        let mut exponentials: [f64; 2] = [0.0; 2];
        trace_step!(exponentials);
        trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for index in 0..2 {
            trace_step!(index);
            trace_note!("Считаем exp(x) первыми 30 членами ряда Тейлора для малых учебных оценок.");
            let mut term: f64 = 1.0;
            trace_step!(term);
            trace_note!("Сохраняем результат этого шага в `sum`.");
            let mut sum: f64 = 1.0;
            trace_step!(sum);
            trace_note!("Повторяем расчёт для каждого элемента последовательности.");
            trace_note!("30 членов ряда Σx^k/k! приближают exp(score−max_score) для softmax.");
            for order in 1..=30 {
                trace_step!(order);
                trace_note!("Обновляем значение результатом текущего вычисления.");
                term *= raw_model_scores[index] / order as f64;
                trace_step!(term);
                trace_note!("Обновляем значение результатом текущего вычисления.");
                sum += term;
                trace_step!(sum);
            }
            trace_note!("Обновляем значение результатом текущего вычисления.");
            exponentials[index] = sum;
            trace_step!(exponentials);
        }
        trace_note!("Сохраняем результат этого шага в `denominator`.");
        let denominator: f64 = exponentials[0] + exponentials[1];
        trace_step!(denominator);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            denominator > 0.0,
            "сумма экспонент должна быть положительной"
        );
        trace_note!("Задаём учебные значения для `weights`.");
        let weights: [f64; 2] = [exponentials[0] / denominator, exponentials[1] / denominator];
        trace_step!(weights);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((weights[0] + weights[1] - 1.0).abs() < 1e-10);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!(weights.iter().all(|&weight| (0.0..=1.0).contains(&weight)));
        trace_note!("Разбираем результат по его возможным вариантам.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        trace_note!("Выполняем действие для этого варианта данных.");
        match description {
            "равные оценки" => assert!((weights[0] - weights[1]).abs() < 1e-10),

            "вторая оценка выше" => {
                trace_note!("Проверяем ожидаемое свойство учебного примера.");
                assert!(weights[1] > weights[0]);
                trace_note!("Обновляем значение результатом текущего вычисления.");
                reference_weights = weights;
                trace_step!(reference_weights);
            }

            "первая оценка выше" => assert!(weights[0] > weights[1]),

            "к обеим прибавили 1" => {
                trace_note!("Проверяем ожидаемое свойство учебного примера.");
                assert!((weights[0] - reference_weights[0]).abs() < 1e-10);
                trace_note!("Проверяем ожидаемое свойство учебного примера.");
                assert!((weights[1] - reference_weights[1]).abs() < 1e-10);
            }

            _ => unreachable!(),
        }
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: {raw_model_scores:?} → {weights:?}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_second_probability_weight_as_its_exponential_divided_by_sum_of_two_exponentials();
}

// Строим график по результатам урока.
fn plot_second_probability_weight_as_its_exponential_divided_by_sum_of_two_exponentials() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразование оценок в вероятности с суммой 1 называют softmax.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let normalized_probability_points: Vec<(f64, f64)> = (-60..=60)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `distance_value`.");
            let distance_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (distance_value, 1.0 / (1.0 + (-distance_value).exp()))
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
