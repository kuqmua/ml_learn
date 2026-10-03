// Урок 23.6. Остановка обучения, когда ошибка на проверочных данных перестаёт улучшаться.
// Зачем здесь эта тема: Продолжение обучения после ухудшения validation может усилить переобучение.
// Почему код устроен так: Отслеживаем лучший результат и останавливаемся после нескольких эпох без
//   улучшения.
// Представь: Если validation перестала улучшаться несколько эпох, продолжать обучение уже
//   необязательно.
//
// Останавливаем обучение после двух подряд эпох без улучшения validation loss.
// Улучшение сбрасывает счётчик; при постоянном улучшении остановки нет.

fn main() {
    let cases: [(&str, &[f64], Option<usize>); 3] = [
        (
            "два ухудшения подряд",
            &[0.8, 0.6, 0.5, 0.52, 0.55, 0.58],
            Some(4),
        ),
        (
            "улучшение сбрасывает счётчик",
            &[0.8, 0.9, 0.7, 0.8, 0.6],
            None,
        ),
        ("каждая эпоха лучше", &[0.8, 0.7, 0.6, 0.5], None),
    ];
    let allowed_non_improving_epochs_before_stopping: i32 = 2;
    for (_description, validation_losses, expected_stop) in cases {
        let mut best: f64 = f64::INFINITY;
        let mut consecutive_epochs_without_lower_validation_error: i32 = 0;
        let mut stopped_at: Option<usize> = None;
        for (epoch, &loss) in validation_losses.iter().enumerate() {
            if loss < best {
                best = loss;
                consecutive_epochs_without_lower_validation_error = 0;
            } else {
                consecutive_epochs_without_lower_validation_error += 1;
            }
            if consecutive_epochs_without_lower_validation_error
                >= allowed_non_improving_epochs_before_stopping
            {
                stopped_at = Some(epoch);
                break;
            }
        }
        assert_eq!(stopped_at, expected_stop);
    }

    // Выполняем вычисления из примера.
    let _ = cases;
}
