// Урок 23.6. Остановка обучения, когда ошибка на проверочных данных перестаёт улучшаться.
// Связь с принятой терминологией: Остановка обучения после отсутствия улучшения ошибки валидации.
// Зачем здесь эта тема: Продолжение обучения после ухудшения validation может усилить переобучение.
// Почему код устроен так: Отслеживаем лучший результат и останавливаемся после нескольких эпох без
//   улучшения.
// Представь: Если validation перестала улучшаться несколько эпох, продолжать обучение уже
//   необязательно.
//
// Останавливаем обучение после двух подряд эпох без улучшения validation loss.
// Улучшение сбрасывает счётчик; при постоянном улучшении остановки нет.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Возвращаем присутствующее значение.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Отмечаем отсутствие подходящего значения.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
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
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Сохраняем результат этого шага в `patience`.");
    let patience: i32 = 2;
    lesson_trace::trace_step!(patience);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, validation_losses, expected_stop) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(validation_losses);
        lesson_trace::trace_step!(expected_stop);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `best`.");
        let mut best: f64 = f64::INFINITY;
        lesson_trace::trace_step!(best);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `bad_epochs`.");
        let mut bad_epochs: i32 = 0;
        lesson_trace::trace_step!(bad_epochs);
        lesson_trace::trace_note!("Сохраняем результат этого шага в `stopped_at`.");
        let mut stopped_at: Option<usize> = None;
        lesson_trace::trace_step!(stopped_at);
        lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
        for (epoch, &loss) in validation_losses.iter().enumerate() {
            lesson_trace::trace_step!(epoch);
            lesson_trace::trace_step!(loss);
            lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
            if loss < best {
                lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                best = loss;
                lesson_trace::trace_step!(best);
                lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                bad_epochs = 0;
                lesson_trace::trace_step!(bad_epochs);
            } else {
                lesson_trace::trace_note!(
                    "Обрабатываем случай, когда предыдущее условие не выполнено."
                );
                lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                bad_epochs += 1;
                lesson_trace::trace_step!(bad_epochs);
            }
            lesson_trace::trace_note!("Выбираем дальнейший шаг по выполнению условия.");
            if bad_epochs >= patience {
                lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
                stopped_at = Some(epoch);
                lesson_trace::trace_step!(stopped_at);
                lesson_trace::trace_note!("Переходим к следующему шагу цикла или завершаем его.");
                break;
            }
        }
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(stopped_at, expected_stop);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!("{description}: остановка={stopped_at:?}, лучший loss={best}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_validation_error_used_to_choose_stopping_step(cases);
}

// Строим график по результатам урока.
fn plot_validation_error_used_to_choose_stopping_step(cases: [(&str, &[f64], Option<usize>); 3]) {
    lesson_trace::trace_note!("Собираем значения для `chart_points` в коллекцию.");
    lesson_trace::trace_note!("Настраиваем или преобразуем результат предыдущего шага.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let chart_points: Vec<(f64, f64)> = cases[0]
        .1
        .iter()
        .enumerate()
        .map(|(epoch, &loss)| (epoch as f64, loss))
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ранняя остановка: первая серия",
        "эпоха",
        "validation loss",
        &[lesson_visualization::Series {
            name: "loss",

            points: &chart_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
