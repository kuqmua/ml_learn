// Урок 23.7. Практика: устойчивое обновление весов и остановка по ошибке на проверочных данных.
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
    let mut results: Vec<(f64, f64)> = Vec::new();
    for past_update_share_kept_in_next_step in [0.0, 0.8] {
        let (loss, _epoch): (f64, usize) = (|| -> (f64, usize) {
            let past_update_share_kept_in_next_step: f64 = past_update_share_kept_in_next_step;
            let (mut weight, mut velocity): (f64, f64) = (8.0, 0.0);
            let (mut best, mut best_epoch): (f64, usize) = (f64::INFINITY, 0);
            for epoch in 0..100 {
                velocity = past_update_share_kept_in_next_step * velocity
                    + (|| -> f64 {
                        let value: f64 = (|| -> f64 {
                            let weight: f64 = weight;
                            2.0 * (weight - 3.0)
                        })();
                        let minimum: f64 = -1.0;
                        let choose_larger_number: f64 = 1.0;
                        if value < minimum {
                            minimum
                        } else if value > choose_larger_number {
                            choose_larger_number
                        } else {
                            value
                        }
                    })();
                weight -= 0.1 * velocity;
                let validation: f64 = (|| -> f64 {
                    let value: f64 = weight - 3.0;
                    value * value
                })();
                if validation < best {
                    best = validation;
                    best_epoch = epoch;
                }
                if epoch - best_epoch > 12 {
                    break;
                }
            }
            (best, best_epoch)
        })();

        results.push((past_update_share_kept_in_next_step, loss));
    }

    // Выполняем вычисления из примера.
    let _ = results;
}
