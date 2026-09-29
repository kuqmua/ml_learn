// Сводная практика 23. Устойчивое обучение с SGD, momentum, Adam и ранней остановкой.
// Почему этот урок сейчас: Устойчивое обучение требует совместить масштабирование, оптимизатор и критерий остановки.
// Почему пример устроен так: Сравниваем SGD, momentum и Adam на одной задаче при одинаковой проверке.
//
// Что повторяем вместе: SGD, momentum, Adam, нормализация, clipping, early stopping.
// Зачем это нужно: Momentum, ограничение градиента и ранняя остановка влияют на устойчивость и итоговую
//   ошибку обучения.
// Что показывает программа: Запускаем один и тот же эксперимент без импульса и с импульсом. Для каждого
//   запуска сохраняем лучшую ошибку на validation и эпоху.
// Что проверить при изменении примера: Сравни на одном split и одинаковой инициализации; выбери эпоху по
//   validation loss.
// Дополнительная практика: Добавь два оптимизатора и контроль нормы градиента к MLP.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Учебные реализации математических операций для этого урока.

    // Шаг: Запускаем один и тот же эксперимент без импульса и с импульсом.
    let mut results: Vec<(f64, f64)> = Vec::new();
    lesson_trace::trace_step!(results);
    // Повторяем расчёт для каждого элемента последовательности.
    for momentum in [0., 0.8] {
        lesson_trace::trace_step!(momentum);
        // Шаг: Для каждого запуска сохраняем лучшую ошибку на validation и эпоху.
        let (loss, epoch): (f64, usize) = (|| -> (f64, usize) {
            // Используем подготовленное значение в следующем шаге примера.
            /* Один параметр позволяет видеть действие momentum, clipping и early stopping. Сравниваем обновление параметра с momentum, ограничением градиента и ранней остановкой. */
            // Сохраняем результат этого шага в `momentum`.
            let momentum: f64 = momentum;
            lesson_trace::trace_step!(momentum);
            // Сохраняем рассчитанное значение `(mut weight, mut velocity)` для следующих операций.
            let (mut weight, mut velocity): (f64, f64) = (8., 0.);
            lesson_trace::trace_step!(weight);
            lesson_trace::trace_step!(velocity);
            // Сохраняем рассчитанное значение `(mut best, mut best_epoch)` для следующих операций.
            let (mut best, mut best_epoch): (f64, usize) = (f64::INFINITY, 0);
            lesson_trace::trace_step!(best);
            lesson_trace::trace_step!(best_epoch);
            // 100 — верхний предел эпох; лучшую модель ниже выбираем по validation loss.
            for epoch in 0..100 {
                lesson_trace::trace_step!(epoch);
                // Выполняем встроенный расчёт один раз и сохраняем результат в `rate_of_change`.
                // Производную функции по параметру или вектор таких производных называют gradient.
                let rate_of_change: f64 = (|| -> f64 {
                    // Используем подготовленное значение в следующем шаге примера.
                    /* Ограничиваем число замкнутым интервалом без готового метода clamp. */
                    // Сохраняем результат этого шага в `value`.
                    let value: f64 = (|| -> f64 {
                        // Используем подготовленное значение в следующем шаге примера.
                        /* Производная квадратичной ошибки по весу равна удвоенному отклонению от минимума. */
                        // Сохраняем результат этого шага в `weight`.
                        let weight: f64 = weight;
                        lesson_trace::trace_step!(weight);
                        // Умножаем величины согласно используемой формуле.
                        2.0 * (weight - 3.0)
                    })();
                    lesson_trace::trace_step!(value);
                    // Комбинируем исходные величины и сохраняем результат в `minimum`.
                    let minimum: f64 = -1.;
                    lesson_trace::trace_step!(minimum);
                    // Сохраняем рассчитанное значение `choose_larger_number` для следующих операций.
                    let choose_larger_number: f64 = 1.;
                    lesson_trace::trace_step!(choose_larger_number);
                    // Проверяем условие и выбираем соответствующую ветку алгоритма.
                    if value < minimum {
                        // Используем ранее рассчитанное значение `minimum` в текущем выражении.
                        minimum
                    // Используем подготовленное значение в следующем шаге примера.
                    } else if value > choose_larger_number {
                        // Используем ранее рассчитанное значение `choose_larger_number` в текущем выражении.
                        choose_larger_number
                    // Обрабатываем случай, когда предыдущее условие не выполнено.
                    } else {
                        // Используем ранее рассчитанное значение `value` в текущем выражении.
                        value
                    }
                })();
                lesson_trace::trace_step!(rate_of_change);
                // Обновляем `velocity` результатом текущего шага.
                velocity = momentum * velocity + rate_of_change;
                lesson_trace::trace_step!(velocity);
                // Вычитаем очередной вклад из текущего значения параметра.
                weight -= 0.1 * velocity;
                lesson_trace::trace_step!(weight);
                // Выполняем встроенный расчёт один раз и сохраняем результат в `validation`.
                let validation: f64 = (|| -> f64 {
                    // Используем подготовленное значение в следующем шаге примера.
                    /* Возводим число в квадрат обычным умножением. */
                    // Сохраняем результат этого шага в `value`.
                    let value: f64 = weight - 3.;
                    lesson_trace::trace_step!(value);
                    // Умножаем величины согласно используемой формуле.
                    value * value
                })();
                lesson_trace::trace_step!(validation);
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if validation < best {
                    // Обновляем `best` результатом текущего шага.
                    best = validation;
                    lesson_trace::trace_step!(best);
                    // Обновляем `best_epoch` результатом текущего шага.
                    best_epoch = epoch;
                    lesson_trace::trace_step!(best_epoch);
                }
                // Проверяем условие и выбираем соответствующую ветку алгоритма.
                if epoch - best_epoch > 12 {
                    // Останавливаем цикл после достижения условия завершения.
                    break;
                }
            }
            // Составляем результат из вычисленных значений в указанном порядке.
            (best, best_epoch)
        })();
        lesson_trace::trace_step!(loss);
        lesson_trace::trace_step!(epoch);
        // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
        println!("momentum={momentum}: best validation loss={loss:.6} at epoch {epoch}");
        // Используем подготовленное значение в следующем шаге примера.
        results.push((momentum, loss));
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_practice_robust_training_with_sgd_momentum_adam_and_early_stopping(results);
}

// Строим график по результатам урока.
fn visualize_practice_robust_training_with_sgd_momentum_adam_and_early_stopping(
    results: std::vec::Vec<(f64, f64)>,
) {
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Momentum и лучшая validation error",
        // Указываем подпись вертикальной оси.
        "ошибка",
        // Передаём ряды или значения для отрисовки графика.
        &[("без momentum", results[0].1), ("с momentum", results[1].1)],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
