// Урок 13.1. Оценка совместного появления признаков и класса: произведение вероятностей.
// Связь с принятой терминологией: Условная независимость признаков при известном классе.
// Зачем здесь эта тема: Наивный Байес упрощает совместную вероятность признаков предположением
//   независимости при известном классе.
// Почему код устроен так: Разбираем именно условие «при известном классе», чтобы не принять
//   признаки за независимые вообще.
// Представь: Два слова могут быть зависимыми вообще, но модель упрощает задачу, считая их
//   независимыми внутри класса.
//
// Что изучаем: Условная независимость признаков.
// Зачем это нужно: Наивный Байес предполагает независимость признаков при известном классе и перемножает
// их условные вероятности.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Инициализируем значение `prior_positive` начальным состоянием.");
    let prior_positive: f64 = 0.5;
    trace_step!(prior_positive);
    trace_note!("Инициализируем значение `word_one_given_positive` начальным состоянием.");
    let word_one_given_positive: f64 = 0.8;
    trace_step!(word_one_given_positive);
    trace_note!("Инициализируем значение `word_two_given_positive` начальным состоянием.");
    let word_two_given_positive: f64 = 0.6;
    trace_step!(word_two_given_positive);
    trace_note!("Умножаем значения и сохраняем результат в `joint_score`.");
    let joint_score: f64 = prior_positive * word_one_given_positive * word_two_given_positive;
    trace_step!(joint_score);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("оценка положительного класса = {joint_score}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_product_of_feature_probabilities_within_class(
        prior_positive,
        word_one_given_positive,
        word_two_given_positive,
        joint_score,
    );
}

// Строим график по результатам урока.
fn plot_product_of_feature_probabilities_within_class(
    prior_positive: f64,
    word_one_given_positive: f64,
    word_two_given_positive: f64,
    joint_score: f64,
) {
    trace_note!("Сравниваем величины, вычисленные в примере.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Условно независимые признаки",
        "вероятность",
        &[
            ("prior", prior_positive),
            ("слово 1", word_one_given_positive),
            ("слово 2", word_two_given_positive),
            ("совместно", joint_score),
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
