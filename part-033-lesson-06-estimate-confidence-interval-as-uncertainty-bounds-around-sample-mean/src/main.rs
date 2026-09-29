// Урок 06.5. Доверительный интервал среднего: оценка границ неопределённости по выборке.
// Связь с принятой терминологией: Доверительный интервал для среднего генеральной совокупности.
// Зачем здесь эта тема: Выборочное среднее меняется от выборки к выборке; интервал отражает эту
//   неопределённость.
// Почему код устроен так: Соединяем среднее с оценкой стандартной ошибки и множителем 1,96; это
//   учебное нормальное приближение.
// Представь: Среднее из четырёх наблюдений — оценка; другой набор из той же совокупности мог бы
//   дать немного другое число.
//
// Что изучаем: Доверительный интервал среднего.
// Зачем это нужно: Интервал показывает неопределённость оценки среднего. Демонстрируем приближение mean ±
// 1.96·SE для небольшого учебного набора.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `values` для следующего шага примера.
    let values: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    lesson_trace::trace_step!(values);
    // Сохраняем результат этого шага в `mean`.
    let mean: f64 =
        part_029_lesson_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(&values)
            .unwrap();
    lesson_trace::trace_step!(mean);
    // Сохраняем результат этого шага в `sample_variance`.
    let sample_variance: f64 =
        part_031_lesson_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(
            &values,
        )
        .unwrap();
    lesson_trace::trace_step!(sample_variance);
    // Считаем количество элементов и сохраняем его в `standard_error_squared`.
    let standard_error_squared: f64 = sample_variance / values.len() as f64;
    lesson_trace::trace_step!(standard_error_squared);
    // Создаём изменяемое значение `standard_error` для следующих операций.
    let mut standard_error: f64 = standard_error_squared;
    lesson_trace::trace_step!(standard_error);
    // 80 шагов Ньютона дают здесь устойчивую оценку корня из SE² в арифметике f64.
    for _ in 0..80 {
        // Среднее текущей оценки и SE²/оценка приближается к стандартной ошибке SE.
        standard_error = (standard_error + standard_error_squared / standard_error) / 2.0;
        lesson_trace::trace_step!(standard_error);
    }
    // 1.96 — квантиль стандартного нормального распределения для двустороннего 95% интервала.
    // Формула mean ± 1.96·SE здесь является приближением для учебного примера.
    let margin: f64 = 1.96 * standard_error;
    lesson_trace::trace_step!(margin);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        // Подставляем результаты в этот шаблон вывода или текстового значения.
        "приближённый интервал: [{:.2}, {:.2}]",
        // Складываем или вычитаем величины согласно используемой формуле.
        mean - margin,
        // Складываем или вычитаем величины согласно используемой формуле.
        mean + margin
    );

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_observations_mean_and_uncertainty_bounds(values, mean, margin);
}

// Строим график по результатам урока.
fn plot_observations_mean_and_uncertainty_bounds(values: [f64; 4], mean: f64, margin: f64) {
    // Границы интервала показаны рядом с наблюдениями и средним.
    let observations: Vec<(f64, f64)> = values
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        // Собираем результаты в коллекцию.
        .collect();
    // Определяем размер данных и сохраняем его в `mean_line`.
    let mean_line: [(f64, f64); 2] = [(1.0, mean), (values.len() as f64, mean)];
    // Определяем размер данных и сохраняем его в `lower`.
    let lower: [(f64, f64); 2] = [(1.0, mean - margin), (values.len() as f64, mean - margin)];
    // Определяем размер данных и сохраняем его в `upper`.
    let upper: [(f64, f64); 2] = [(1.0, mean + margin), (values.len() as f64, mean + margin)];
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Приближённый доверительный интервал",
        // Указываем подпись горизонтальной оси.
        "номер наблюдения",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "выборка",
                // Передаём рассчитанные координаты точек.
                points: &observations,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "среднее",
                // Передаём рассчитанные координаты точек.
                points: &mean_line,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "нижняя граница",
                // Передаём рассчитанные координаты точек.
                points: &lower,
            },
            // Добавляем ряд данных с подписью к графику.
            lesson_visualization::Series {
                // Указываем подпись этого ряда в легенде.
                name: "верхняя граница",
                // Передаём рассчитанные координаты точек.
                points: &upper,
            },
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
