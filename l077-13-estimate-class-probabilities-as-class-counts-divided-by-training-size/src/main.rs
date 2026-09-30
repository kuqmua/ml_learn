// Урок 13.2. Начальные вероятности классов: доли их примеров в обучающем наборе.
// Связь с принятой терминологией: Априорные вероятности классов по обучающим данным.
// Зачем здесь эта тема: Чтобы сравнивать классы по Байесу, нужна исходная частота каждого класса до
//   чтения признаков.
// Почему код устроен так: Считаем prior по обучающим меткам и отделяем его от вероятности
//   признаков.
// Представь: Если 9 из 10 учебных текстов относятся к классу A, он изначально вероятнее до чтения
//   нового текста.
//
// Что изучаем: Априорные вероятности классов.
// Зачем это нужно: До чтения признаков модель учитывает, как часто встречается каждый класс в обучающем
// наборе.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Создаём набор значений `labels` для следующего шага примера.");
    let labels: [&str; 4] = ["code", "code", "code", "ml"];
    trace_step!(labels);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !labels.is_empty(),
        "для частоты класса нужна хотя бы одна метка"
    );
    trace_note!("Преобразуем входные данные и сохраняем полученную коллекцию в `code_count`.");
    let code_count: usize = labels.iter().filter(|&&label| label == "code").count();
    trace_step!(code_count);
    trace_note!("Считаем количество элементов и сохраняем его в `code_prior`.");
    let code_prior: f64 = code_count as f64 / labels.len() as f64;
    trace_step!(code_prior);
    trace_note!("Комбинируем исходные величины и сохраняем результат в `machine_learning_prior`.");
    let machine_learning_prior: f64 = 1.0 - code_prior;
    trace_step!(machine_learning_prior);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("P(code)={code_prior}, P(ml)={machine_learning_prior}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_training_class_shares(code_prior, machine_learning_prior);
}

// Строим график по результатам урока.
fn plot_training_class_shares(code_prior: f64, machine_learning_prior: f64) {
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
        "Априорные вероятности",
        "вероятность",
        &[("code", code_prior), ("ml", machine_learning_prior)],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
