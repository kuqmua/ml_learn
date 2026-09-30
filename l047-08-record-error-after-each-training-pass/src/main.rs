// Урок 08.4. Запись ошибки после каждого прохода по обучающим данным.
// Связь с принятой терминологией: Запись значения метрики по эпохам обучения.
// Зачем здесь эта тема: Итоговая метрика скрывает ухудшение в середине обучения или переобучение.
// Почему код устроен так: Записываем значение после каждой эпохи и сравниваем изменение во времени.
// Представь: Ошибка по эпохам 10→7→5 показывает улучшение, а 10→7→9 — причину проверить обучение.
//
// Что изучаем: Журнал метрик.
// Зачем это нужно: Записи по эпохам показывают ход обучения, а не только итоговую цифру. Сохраняем номер
// эпохи и ошибку рядом.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `losses` для следующего шага примера.");
    let losses: [f64; 3] = [1.0, 0.6, 0.4];
    lesson_trace::trace_step!(losses);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for (epoch, loss) in losses.into_iter().enumerate() {
        lesson_trace::trace_step!(epoch);
        lesson_trace::trace_step!(loss);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("epoch={epoch}, loss={loss:.2}");
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_error_after_each_training_pass(losses);
}

// Строим график по результатам урока.
fn plot_error_after_each_training_pass(losses: [f64; 3]) {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Передаём элементы коллекции в итератор.");
    lesson_trace::trace_note!("Добавляем порядковый номер к каждому элементу.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let metric_log_points: Vec<(f64, f64)> = losses
        .into_iter()
        .enumerate()
        .map(|(item_index, element_value)| (item_index as f64, element_value))
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
        "Журнал обучения",
        "эпоха",
        "ошибка",
        &[lesson_visualization::Series {
            name: "loss",

            points: &metric_log_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
