// Урок 17.4. Расчёт подготовки признаков только по текущей обучающей группе.
// Связь с принятой терминологией: Утечка при подготовке признаков между блоками кросс-валидации.
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
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `training_data` для следующего шага примера.");
    let training_data: [f64; 3] = [1.0, 2.0, 3.0];
    trace_step!(training_data);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    trace_note!("Создаём набор значений `validation` для следующего шага примера.");
    let validation: [f64; 1] = [100.0];
    trace_step!(validation);
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `training_mean`.");
    let training_mean: f64 = training_data.iter().sum::<f64>() / training_data.len() as f64;
    trace_step!(training_mean);
    trace_note!("Комбинируем исходные величины и сохраняем результат в `validation_centered`.");
    let validation_centered: f64 = validation[0] - training_mean;
    trace_step!(validation_centered);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("среднее train={training_mean}, validation после центрирования={validation_centered}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_training_mean_and_centered_validation_value(training_mean, validation_centered);
}

// Строим график по результатам урока.
fn plot_training_mean_and_centered_validation_value(training_mean: f64, validation_centered: f64) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Центрирование по train",
        "значение",
        &[
            ("train mean", training_mean),
            ("validation centered", validation_centered),
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
