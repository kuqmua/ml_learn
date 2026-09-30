// Урок 05.3. Пересчёт вероятности события с учётом частоты полученного наблюдения.
// Связь с принятой терминологией: Пересчёт вероятности события после наблюдения по формуле Байеса.
// Зачем здесь эта тема: Диагностический сигнал меняет исходную вероятность события; формула Байеса
//   соединяет prior и качество сигнала.
// Почему код устроен так: Разделяем истинные и ложные сигналы, чтобы увидеть влияние редкости
//   события.
// Представь: Даже хороший тест даёт много ложных тревог, когда проверяемое событие очень редкое.
//
// Что изучаем: Формула Байеса.
// Зачем это нужно: Формула пересчитывает вероятность причины после наблюдения результата. Редкая болезнь
// остаётся редкой даже после несовершенного теста.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Инициализируем значение `prevalence` начальным состоянием.");
    let prevalence: f64 = 0.01;
    trace_step!(prevalence);
    trace_note!("Инициализируем значение `sensitivity` начальным состоянием.");
    let sensitivity: f64 = 0.90;
    trace_step!(sensitivity);
    trace_note!("Инициализируем значение `specificity` начальным состоянием.");
    let specificity: f64 = 0.95;
    trace_step!(specificity);
    trace_note!("Умножаем значения и сохраняем результат в `true_positive`.");
    let true_positive: f64 = prevalence * sensitivity;
    trace_step!(true_positive);
    trace_note!("Умножаем значения и сохраняем результат в `false_positive`.");
    let false_positive: f64 = (1.0 - prevalence) * (1.0 - specificity);
    trace_step!(false_positive);
    trace_note!("Нормируем или усредняем величину делением и сохраняем её в `posterior`.");
    let posterior: f64 = true_positive / (true_positive + false_positive);
    trace_step!(posterior);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("P(болен | положительный тест) = {posterior:.3}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_disease_probability_before_and_after_positive_test(posterior);
}

// Строим график по результатам урока.
fn plot_disease_probability_before_and_after_positive_test(posterior: f64) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Байес: до и после теста",
        "вероятность",
        &[("до теста", 0.01), ("после теста", posterior)],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
