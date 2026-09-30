// Урок 07.6. Проверка влияния тестовых данных на статистики, используемые при обучении.
// Связь с принятой терминологией: Утечка информации из тестовых данных в обучение.
// Зачем здесь эта тема: Даже без передачи меток в обучение статистика test может сделать оценку
//   качества слишком оптимистичной.
// Почему код устроен так: Сравниваем честную подготовку с подготовкой, куда попали тестовые строки.
// Представь: Если в test есть выброс 1000, среднее train не должно измениться из-за этого числа.
//
// Что изучаем: Утечка данных.
// Зачем это нужно: Статистику подготовки признаков нельзя считать по test: иначе тестовые значения влияют
// на обучение.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Создаём набор значений `training_data` для следующего шага примера."
    );
    let training_data: [f64; 3] = [1.0, 2.0, 3.0];
    lesson_trace::trace_step!(training_data);
    lesson_trace::trace_note!("Создаём набор значений `test` для следующего шага примера.");
    let test: [f64; 1] = [100.0];
    lesson_trace::trace_step!(test);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `training_mean`."
    );
    let training_mean: f64 = training_data.iter().sum::<f64>() / training_data.len() as f64;
    lesson_trace::trace_step!(training_mean);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `contaminated_mean` для следующих операций."
    );
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    let contaminated_mean: f64 = (training_data.iter().sum::<f64>() + test.iter().sum::<f64>())
        / (training_data.len() + test.len()) as f64;
    lesson_trace::trace_step!(contaminated_mean);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("только train={training_mean}, с утечкой={contaminated_mean}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_means_computed_with_and_without_test_data(training_mean, contaminated_mean);
}

// Строим график по результатам урока.
fn plot_means_computed_with_and_without_test_data(training_mean: f64, contaminated_mean: f64) {
    lesson_trace::trace_note!("Сравнение величин из этого урока.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Утечка меняет статистику",
        "среднее",
        &[("train", training_mean), ("с утечкой", contaminated_mean)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
