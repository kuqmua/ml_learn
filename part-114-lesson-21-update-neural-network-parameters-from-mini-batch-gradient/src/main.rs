// Урок 21.5. Обновление параметров сети по среднему градиенту мини пакета.
//
// Что изучаем: Mini-batch обучение.
// Зачем это нужно: Шаг обновления может использовать средний градиент небольшой группы примеров вместо
// одного объекта или всего набора.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `rates_of_change` для следующего шага примера.
    // Производную функции по параметру или вектор таких производных называют gradient.
    let rates_of_change: [f64; 2] = [2.0, 4.0];
    lesson_trace::trace_step!(rates_of_change);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        // Используем подготовленное значение в следующем шаге примера.
        !rates_of_change.is_empty(),
        // Передаём подпись или текстовое значение для следующего шага.
        "мини-пакет градиентов не должен быть пустым"
    );
    // Преобразуем входные данные и сохраняем полученную коллекцию в `small_batch_loss_rate_of_change`.
    let small_batch_loss_rate_of_change: f64 =
        rates_of_change.iter().sum::<f64>() / rates_of_change.len() as f64;
    lesson_trace::trace_step!(small_batch_loss_rate_of_change);
    // Сохраняем рассчитанное значение `old_weight` для следующих операций.
    let old_weight: f64 = 1.0;
    lesson_trace::trace_step!(old_weight);
    // Скорость 0.1 означает, что из веса вычитается десятая часть среднего градиента мини-батча.
    let learning_rate: f64 = 0.1;
    lesson_trace::trace_step!(learning_rate);
    // Умножаем значения и сохраняем результат в `new_weight`.
    let new_weight: f64 = old_weight - learning_rate * small_batch_loss_rate_of_change;
    lesson_trace::trace_step!(new_weight);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("средний градиент={small_batch_loss_rate_of_change}, новый вес={new_weight}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_update_neural_network_parameters_from_mini_batch_gradient(old_weight, new_weight);
}

// Строим график по результатам урока.
fn visualize_update_neural_network_parameters_from_mini_batch_gradient(
    old_weight: f64,
    new_weight: f64,
) {
    // Сравниваем величины, вычисленные в примере.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Mini-batch обновление",
        // Указываем подпись вертикальной оси.
        "вес",
        // Передаём ряды или значения для отрисовки графика.
        &[("до", old_weight), ("после", new_weight)],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
