// Урок 04.4. Пакетное и стохастическое обновление по градиенту.
// Зачем здесь эта тема: Градиент можно оценить по всем примерам или по одному; это меняет шум и
//   стоимость шага.
// Почему код устроен так: Сравниваем оба режима на одних данных, фиксируя, какой набор дал
//   очередное обновление.
// Представь: Среднее по всем строкам даёт ровный шаг, а отдельная случайная строка может временно
//   толкнуть параметр в другую сторону.
//
// Что изучаем: Batch и stochastic обновления.
// Зачем это нужно: Batch использует средний градиент всех примеров, stochastic — градиент одного. На одном
// шаге их направления могут отличаться.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `data` для следующего шага примера.
    let data: [(f64, f64); 2] = [(1.0, 2.0), (2.0, 4.0)];
    lesson_trace::trace_step!(data);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(!data.is_empty(), "для градиента нужны обучающие примеры");
    // Инициализируем значение `weight` начальным состоянием.
    let weight: f64 = 0.0;
    lesson_trace::trace_step!(weight);
    // Инициализируем изменяемый накопитель `summed_rates_of_change` начальным состоянием.
    // Производную функции по параметру или вектор таких производных называют gradient.
    let mut summed_rates_of_change: f64 = 0.0;
    lesson_trace::trace_step!(summed_rates_of_change);
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for (feature, target) in data {
        lesson_trace::trace_step!(feature);
        lesson_trace::trace_step!(target);
        // Прибавляем очередной вклад к ранее накопленному результату.
        summed_rates_of_change += 2.0 * (weight * feature - target) * feature;
        lesson_trace::trace_step!(summed_rates_of_change);
    }
    // Считаем количество элементов и сохраняем его в `batch_loss_rate_of_change`.
    let batch_loss_rate_of_change: f64 = summed_rates_of_change / data.len() as f64;
    lesson_trace::trace_step!(batch_loss_rate_of_change);
    // Сохраняем рассчитанное значение `(first_feature, first_target)` для следующих операций.
    let (first_feature, first_target): (f64, f64) = data[0];
    lesson_trace::trace_step!(first_feature);
    lesson_trace::trace_step!(first_target);
    // Умножаем значения и сохраняем результат в `single_example_loss_rate_of_change`.
    let single_example_loss_rate_of_change: f64 =
        2.0 * (weight * first_feature - first_target) * first_feature;
    lesson_trace::trace_step!(single_example_loss_rate_of_change);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("batch={batch_loss_rate_of_change}, stochastic={single_example_loss_rate_of_change}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_batch_versus_stochastic_gradient_updates(
        batch_loss_rate_of_change,
        single_example_loss_rate_of_change,
    );
}

// Строим график по результатам урока.
fn visualize_batch_versus_stochastic_gradient_updates(
    batch_loss_rate_of_change: f64,
    single_example_loss_rate_of_change: f64,
) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Batch и stochastic градиенты",
        // Указываем подпись вертикальной оси.
        "градиент",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("batch", batch_loss_rate_of_change),
            // Добавляем пару значений для сравнения или построения графика.
            ("stochastic", single_example_loss_rate_of_change),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
