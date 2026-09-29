// Урок 07.6. Утечка информации из тестовых данных в обучение.
//
// Что изучаем: Утечка данных.
// Зачем это нужно: Статистику подготовки признаков нельзя считать по test: иначе тестовые значения влияют
// на обучение.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `training_data` для следующего шага примера.
    let training_data: [f64; 3] = [1.0, 2.0, 3.0];
    lesson_trace::trace_step!(training_data);
    // Создаём набор значений `test` для следующего шага примера.
    let test: [f64; 1] = [100.0];
    lesson_trace::trace_step!(test);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    // Преобразуем входные данные и сохраняем полученную коллекцию в `training_mean`.
    let training_mean: f64 = training_data.iter().sum::<f64>() / training_data.len() as f64;
    lesson_trace::trace_step!(training_mean);
    // Сохраняем рассчитанное значение `contaminated_mean` для следующих операций.
    let contaminated_mean: f64 =
        // Составляем результат из вычисленных значений в указанном порядке.
        (training_data.iter().sum::<f64>() + test.iter().sum::<f64>()) / (training_data.len() + test.len()) as f64;
    lesson_trace::trace_step!(contaminated_mean);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("только train={training_mean}, с утечкой={contaminated_mean}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_data_leakage_from_test_set_into_training(training_mean, contaminated_mean);
}

// Строим график по результатам урока.
fn visualize_data_leakage_from_test_set_into_training(training_mean: f64, contaminated_mean: f64) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Утечка меняет статистику",
        // Указываем подпись вертикальной оси.
        "среднее",
        // Передаём ряды или значения для отрисовки графика.
        &[("train", training_mean), ("с утечкой", contaminated_mean)],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
