// Урок 23.6. Остановка обучения после отсутствия улучшения ошибки валидации.
//
// Останавливаем обучение после двух подряд эпох без улучшения validation loss.
// Улучшение сбрасывает счётчик; при постоянном улучшении остановки нет.

fn main() {
    lesson_trace::enable();
    // Задаём учебные значения для `cases`.
    let cases: [(&str, &[f64], Option<usize>); 3] = [
        (
            // Передаём подпись или текстовое значение для следующего шага.
            "два ухудшения подряд",
            // Передаём ряды или значения для отрисовки графика.
            &[0.8, 0.6, 0.5, 0.52, 0.55, 0.58],
            // Возвращаем присутствующее значение.
            Some(4),
        ),
        (
            // Передаём подпись или текстовое значение для следующего шага.
            "улучшение сбрасывает счётчик",
            // Передаём ряды или значения для отрисовки графика.
            &[0.8, 0.9, 0.7, 0.8, 0.6],
            // Отмечаем отсутствие подходящего значения.
            None,
        ),
        // Добавляем пару значений для сравнения или построения графика.
        ("каждая эпоха лучше", &[0.8, 0.7, 0.6, 0.5], None),
    ];
    lesson_trace::trace_step!(cases);
    // Сохраняем результат этого шага в `patience`.
    let patience: i32 = 2;
    lesson_trace::trace_step!(patience);
    // Повторяем расчёт для каждого элемента последовательности.
    for (description, validation_losses, expected_stop) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(validation_losses);
        lesson_trace::trace_step!(expected_stop);
        // Сохраняем результат этого шага в `best`.
        let mut best: f64 = f64::INFINITY;
        lesson_trace::trace_step!(best);
        // Сохраняем результат этого шага в `bad_epochs`.
        let mut bad_epochs: i32 = 0;
        lesson_trace::trace_step!(bad_epochs);
        // Сохраняем результат этого шага в `stopped_at`.
        let mut stopped_at: Option<usize> = None;
        lesson_trace::trace_step!(stopped_at);
        // Повторяем расчёт для каждого элемента последовательности.
        for (epoch, &loss) in validation_losses.iter().enumerate() {
            lesson_trace::trace_step!(epoch);
            lesson_trace::trace_step!(loss);
            // Выбираем дальнейший шаг по выполнению условия.
            if loss < best {
                // Обновляем значение результатом текущего вычисления.
                best = loss;
                lesson_trace::trace_step!(best);
                // Обновляем значение результатом текущего вычисления.
                bad_epochs = 0;
                lesson_trace::trace_step!(bad_epochs);
            // Обрабатываем случай, когда предыдущее условие не выполнено.
            } else {
                // Обновляем значение результатом текущего вычисления.
                bad_epochs += 1;
                lesson_trace::trace_step!(bad_epochs);
            }
            // Выбираем дальнейший шаг по выполнению условия.
            if bad_epochs >= patience {
                // Обновляем значение результатом текущего вычисления.
                stopped_at = Some(epoch);
                lesson_trace::trace_step!(stopped_at);
                // Переходим к следующему шагу цикла или завершаем его.
                break;
            }
        }
        // Проверяем ожидаемое свойство учебного примера.
        assert_eq!(stopped_at, expected_stop);
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: остановка={stopped_at:?}, лучший loss={best}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_stop_training_after_validation_loss_stagnates(cases);
}

// Строим график по результатам урока.
fn visualize_stop_training_after_validation_loss_stagnates(
    cases: [(&str, &[f64], Option<usize>); 3],
) {
    // Собираем значения для `chart_points` в коллекцию.
    let chart_points: Vec<(f64, f64)> = cases[0]
        // Настраиваем или преобразуем результат предыдущего шага.
        .1
        // Просматриваем элементы коллекции по ссылке.
        .iter()
        // Добавляем порядковый номер к каждому элементу.
        .enumerate()
        // Преобразуем каждый элемент в новое значение.
        .map(|(epoch, &loss)| (epoch as f64, loss))
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Ранняя остановка: первая серия",
        // Указываем подпись горизонтальной оси.
        "эпоха",
        // Указываем подпись вертикальной оси.
        "validation loss",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "loss",
            // Передаём рассчитанные координаты точек.
            points: &chart_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
