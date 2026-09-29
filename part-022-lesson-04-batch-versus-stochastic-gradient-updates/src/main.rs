// Урок 04.4. Пакетное и стохастическое обновление по градиенту.
//
// Что изучаем: Batch и stochastic обновления.
// Зачем это нужно: Batch использует средний градиент всех примеров, stochastic — градиент одного. На одном
// шаге их направления могут отличаться.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Создаём набор значений `data` для следующего шага примера.
    let data = [(1.0, 2.0), (2.0, 4.0)];
    // Проверяем ожидаемое свойство учебного примера.
    assert!(!data.is_empty(), "для градиента нужны обучающие примеры");
    // Инициализируем значение `weight` начальным состоянием.
    let weight = 0.0;
    // Инициализируем изменяемый накопитель `summed_rates_of_change` начальным состоянием.
    // Производную функции по параметру или вектор таких производных называют gradient.
    let mut summed_rates_of_change = 0.0;
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for (feature, target) in data {
        // Прибавляем очередной вклад к ранее накопленному результату.
        summed_rates_of_change += 2.0 * (weight * feature - target) * feature;
    }
    // Считаем количество элементов и сохраняем его в `batch_loss_rate_of_change`.
    let batch_loss_rate_of_change = summed_rates_of_change / data.len() as f64;
    // Сохраняем рассчитанное значение `(first_feature, first_target)` для следующих операций.
    let (first_feature, first_target) = data[0];
    // Умножаем значения и сохраняем результат в `single_example_loss_rate_of_change`.
    let single_example_loss_rate_of_change =
        2.0 * (weight * first_feature - first_target) * first_feature;
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("batch={batch_loss_rate_of_change}, stochastic={single_example_loss_rate_of_change}");

    // Построение графика вынесено из основного кода урока.
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
    let chart = lesson_visualization::bar_chart(
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
