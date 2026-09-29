// Урок 17.4. Утечка при подготовке признаков между блоками кросс-валидации.
// Зачем здесь эта тема: Разделить строки недостаточно, если нормализация обучена сразу на всех
//   блоках.
// Почему код устроен так: Переобучаем преобразование внутри каждого train-блока, не используя
//   соответствующий validation-блок.
// Представь: Среднее для нормализации первого fold нельзя считать с участием его проверочных строк.
//
// Что изучаем: Утечка при подготовке признаков.
// Зачем это нужно: В каждом fold среднее и другие статистики вычисляем только по обучающей части, а затем
// применяем к validation.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    // Создаём набор значений `training_data` для следующего шага примера.
    let training_data: [f64; 3] = [1.0, 2.0, 3.0];
    lesson_trace::trace_step!(training_data);
    // Проверяем ожидаемое свойство учебного примера.
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    // Создаём набор значений `validation` для следующего шага примера.
    let validation: [f64; 1] = [100.0];
    lesson_trace::trace_step!(validation);
    // Преобразуем входные данные и сохраняем полученную коллекцию в `training_mean`.
    let training_mean: f64 = training_data.iter().sum::<f64>() / training_data.len() as f64;
    lesson_trace::trace_step!(training_mean);
    // Комбинируем исходные величины и сохраняем результат в `validation_centered`.
    let validation_centered: f64 = validation[0] - training_mean;
    lesson_trace::trace_step!(validation_centered);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("среднее train={training_mean}, validation после центрирования={validation_centered}");

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    visualize_preprocessing_leakage_across_cross_validation_folds(
        training_mean,
        validation_centered,
    );
}

// Строим график по результатам урока.
fn visualize_preprocessing_leakage_across_cross_validation_folds(
    training_mean: f64,
    validation_centered: f64,
) {
    // Сравнение величин из этого урока.
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок диаграммы.
        "Центрирование по train",
        // Указываем подпись вертикальной оси.
        "значение",
        // Передаём ряды или значения для отрисовки графика.
        &[
            // Добавляем пару значений для сравнения или построения графика.
            ("train mean", training_mean),
            // Добавляем пару значений для сравнения или построения графика.
            ("validation centered", validation_centered),
        ],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
