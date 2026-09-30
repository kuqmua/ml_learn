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
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Инициализируем значение `prior_positive` начальным состоянием.");
    let prior_positive: f64 = 0.5;
    lesson_trace::trace_step!(prior_positive);
    lesson_trace::trace_note!(
        "Инициализируем значение `word_one_given_positive` начальным состоянием."
    );
    let word_one_given_positive: f64 = 0.8;
    lesson_trace::trace_step!(word_one_given_positive);
    lesson_trace::trace_note!(
        "Инициализируем значение `word_two_given_positive` начальным состоянием."
    );
    let word_two_given_positive: f64 = 0.6;
    lesson_trace::trace_step!(word_two_given_positive);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `joint_score`.");
    let joint_score: f64 = prior_positive * word_one_given_positive * word_two_given_positive;
    lesson_trace::trace_step!(joint_score);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("оценка положительного класса = {joint_score}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
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
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
