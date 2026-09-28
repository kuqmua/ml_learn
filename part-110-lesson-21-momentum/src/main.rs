// Урок 21.2. Импульс momentum.
//
// Что изучаем: Импульс momentum.
// Зачем это нужно: Скорость накапливает прежние градиенты и сглаживает последовательность обновлений.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Создаём набор значений `gradients` для следующего шага примера.
    let gradients = [2.0, 1.0, -0.5];
    // Инициализируем изменяемый накопитель `velocity` начальным состоянием.
    let mut velocity = 0.0;
    // Создаём изменяемое значение `weight` для следующих операций.
    let mut weight = 1.0;
    let mut weight_history = vec![(0.0, weight)];
    let mut velocity_history = vec![(0.0, velocity)];
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for (step, gradient) in gradients.into_iter().enumerate() {
        // Присваиваем вычисленное значение соответствующей переменной или полю.
        velocity = 0.8 * velocity + gradient;
        // Вычитаем очередной вклад из текущего значения параметра.
        weight -= 0.1 * velocity;
        // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
        println!("градиент={gradient}, скорость={velocity}, вес={weight}");
        weight_history.push(((step + 1) as f64, weight));
        velocity_history.push(((step + 1) as f64, velocity));
    }
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Momentum: накопление скорости",
        "шаг",
        "значение",
        &[
            lesson_visualization::Series {
                name: "вес",
                points: &weight_history,
            },
            lesson_visualization::Series {
                name: "скорость",
                points: &velocity_history,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
