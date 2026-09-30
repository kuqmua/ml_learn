// Урок 23.7. Практика: устойчивое обновление весов и остановка по ошибке на проверочных данных.
// Связь с принятой терминологией: Устойчивое обучение с SGD, momentum, Adam и ранней остановкой.
// Зачем здесь эта тема: Устойчивое обучение требует совместить масштабирование, оптимизатор и
//   критерий остановки.
// Почему код устроен так: Сравниваем SGD, momentum и Adam на одной задаче при одинаковой проверке.
// Представь: Сравниваем несколько правил обновления на одинаковой ошибке, чтобы выбрать устойчивый
//   ход обучения.
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
    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    lesson_trace::trace_note!(
        "Шаг: Запускаем один и тот же эксперимент без импульса и с импульсом."
    );
    let mut results: Vec<(f64, f64)> = Vec::new();
    lesson_trace::trace_step!(results);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for momentum in [0., 0.8] {
        lesson_trace::trace_step!(momentum);
        lesson_trace::trace_note!(
            "Шаг: Для каждого запуска сохраняем лучшую ошибку на validation и эпоху."
        );
        let (loss, epoch): (f64, usize) = (|| -> (f64, usize) {
            lesson_trace::trace_note!(
                "Используем подготовленное значение в следующем шаге примера."
            );
            lesson_trace::trace_note!(
                "Один параметр позволяет видеть действие momentum, clipping и early stopping. Сравниваем обновление параметра с momentum, ограничением градиента и ранней остановкой."
            );
            lesson_trace::trace_note!("Сохраняем результат этого шага в `momentum`.");
            let momentum: f64 = momentum;
            lesson_trace::trace_step!(momentum);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `(mut weight, mut velocity)` для следующих операций."
            );
            let (mut weight, mut velocity): (f64, f64) = (8., 0.);
            lesson_trace::trace_step!(weight);
            lesson_trace::trace_step!(velocity);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `(mut best, mut best_epoch)` для следующих операций."
            );
            let (mut best, mut best_epoch): (f64, usize) = (f64::INFINITY, 0);
            lesson_trace::trace_step!(best);
            lesson_trace::trace_step!(best_epoch);
            lesson_trace::trace_note!(
                "100 — верхний предел эпох; лучшую модель ниже выбираем по validation loss."
            );
            for epoch in 0..100 {
                lesson_trace::trace_step!(epoch);
                lesson_trace::trace_note!(
                    "Выполняем встроенный расчёт один раз и сохраняем результат в `rate_of_change`."
                );
                lesson_trace::trace_note!(
                    "Производную функции по параметру или вектор таких производных называют gradient."
                );
                let rate_of_change: f64 = (|| -> f64 {
                    lesson_trace::trace_note!(
                        "Используем подготовленное значение в следующем шаге примера."
                    );
                    lesson_trace::trace_note!(
                        "Ограничиваем число замкнутым интервалом без готового метода clamp."
                    );
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
                    let value: f64 = (|| -> f64 {
                        lesson_trace::trace_note!(
                            "Используем подготовленное значение в следующем шаге примера."
                        );
                        lesson_trace::trace_note!(
                            "Производная квадратичной ошибки по весу равна удвоенному отклонению от минимума."
                        );
                        lesson_trace::trace_note!("Сохраняем результат этого шага в `weight`.");
                        let weight: f64 = weight;
                        lesson_trace::trace_step!(weight);
                        lesson_trace::trace_note!(
                            "Умножаем величины согласно используемой формуле."
                        );
                        2.0 * (weight - 3.0)
                    })();
                    lesson_trace::trace_step!(value);
                    lesson_trace::trace_note!(
                        "Комбинируем исходные величины и сохраняем результат в `minimum`."
                    );
                    let minimum: f64 = -1.;
                    lesson_trace::trace_step!(minimum);
                    lesson_trace::trace_note!(
                        "Сохраняем рассчитанное значение `choose_larger_number` для следующих операций."
                    );
                    let choose_larger_number: f64 = 1.;
                    lesson_trace::trace_step!(choose_larger_number);
                    lesson_trace::trace_note!(
                        "Проверяем условие и выбираем соответствующую ветку алгоритма."
                    );
                    if value < minimum {
                        lesson_trace::trace_note!(
                            "Используем ранее рассчитанное значение `minimum` в текущем выражении."
                        );
                        lesson_trace::trace_note!(
                            "Используем подготовленное значение в следующем шаге примера."
                        );
                        minimum
                    } else if value > choose_larger_number {
                        lesson_trace::trace_note!(
                            "Используем ранее рассчитанное значение `choose_larger_number` в текущем выражении."
                        );
                        choose_larger_number
                    } else {
                        lesson_trace::trace_note!(
                            "Обрабатываем случай, когда предыдущее условие не выполнено."
                        );
                        lesson_trace::trace_note!(
                            "Используем ранее рассчитанное значение `value` в текущем выражении."
                        );
                        value
                    }
                })();
                lesson_trace::trace_step!(rate_of_change);
                lesson_trace::trace_note!("Обновляем `velocity` результатом текущего шага.");
                velocity = momentum * velocity + rate_of_change;
                lesson_trace::trace_step!(velocity);
                lesson_trace::trace_note!(
                    "Вычитаем очередной вклад из текущего значения параметра."
                );
                weight -= 0.1 * velocity;
                lesson_trace::trace_step!(weight);
                lesson_trace::trace_note!(
                    "Выполняем встроенный расчёт один раз и сохраняем результат в `validation`."
                );
                let validation: f64 = (|| -> f64 {
                    lesson_trace::trace_note!(
                        "Используем подготовленное значение в следующем шаге примера."
                    );
                    lesson_trace::trace_note!("Возводим число в квадрат обычным умножением.");
                    lesson_trace::trace_note!("Сохраняем результат этого шага в `value`.");
                    let value: f64 = weight - 3.;
                    lesson_trace::trace_step!(value);
                    lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
                    value * value
                })();
                lesson_trace::trace_step!(validation);
                lesson_trace::trace_note!(
                    "Проверяем условие и выбираем соответствующую ветку алгоритма."
                );
                if validation < best {
                    lesson_trace::trace_note!("Обновляем `best` результатом текущего шага.");
                    best = validation;
                    lesson_trace::trace_step!(best);
                    lesson_trace::trace_note!("Обновляем `best_epoch` результатом текущего шага.");
                    best_epoch = epoch;
                    lesson_trace::trace_step!(best_epoch);
                }
                lesson_trace::trace_note!(
                    "Проверяем условие и выбираем соответствующую ветку алгоритма."
                );
                if epoch - best_epoch > 12 {
                    lesson_trace::trace_note!(
                        "Останавливаем цикл после достижения условия завершения."
                    );
                    break;
                }
            }
            lesson_trace::trace_note!(
                "Составляем результат из вычисленных значений в указанном порядке."
            );
            (best, best_epoch)
        })();
        lesson_trace::trace_step!(loss);
        lesson_trace::trace_step!(epoch);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("momentum={momentum}: best validation loss={loss:.6} at epoch {epoch}");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        results.push((momentum, loss));
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_best_validation_error_during_training(results);
}

// Строим график по результатам урока.
fn plot_best_validation_error_during_training(results: std::vec::Vec<(f64, f64)>) {
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Momentum и лучшая validation error",
        "ошибка",
        &[("без momentum", results[0].1), ("с momentum", results[1].1)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
