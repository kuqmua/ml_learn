// Урок 05.2. Проверка независимости событий: сравнение совместной вероятности с произведением вероятностей.
// Связь с принятой терминологией: Независимость двух событий.
// Зачем здесь эта тема: Формула произведения упрощается лишь при независимости; это допущение надо
//   проверять.
// Почему код устроен так: Сравниваем совместную и произведение отдельных вероятностей на понятных
//   событиях.
// Представь: Если вероятность дождя не меняет вероятность выбранного исхода, эти события можно
//   считать независимыми.
//
// События независимы, когда вероятность их совместного появления равна результату
// умножения отдельных вероятностей. Одинаковые отдельные вероятности этого не гарантируют.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, f64, f64, f64, bool); 3] = [
        ("две независимые монеты", 0.5, 0.5, 0.25, true),
        ("зависимые события", 0.5, 0.5, 0.5, false),
        ("несовместимые события", 0.5, 0.5, 0.0, false),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, first, second, both, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(first);
        lesson_trace::trace_step!(second);
        lesson_trace::trace_step!(both);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((0.0..=1.0).contains(&first) && (0.0..=1.0).contains(&second));
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert!((0.0..=1.0).contains(&both));
        lesson_trace::trace_note!("Сохраняем результат этого шага в `independent`.");
        lesson_trace::trace_note!(
            "10⁻¹⁰ допускает округление f64 при проверке равенства P(A∩B)=P(A)·P(B)."
        );
        let independent: bool = (both - first * second).abs() < 1e-10;
        lesson_trace::trace_step!(independent);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(independent, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        println!(
            "{description}: P(A)={first}, P(B)={second}, P(A и B)={both}, независимы={independent}"
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_joint_event_probabilities(cases);
}

// Строим график по результатам урока.
fn plot_joint_event_probabilities(cases: [(&str, f64, f64, f64, bool); 3]) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Зависимость событий",
        "P(A∩B)",
        &[
            ("независимые", cases[0].3),
            ("зависимые", cases[1].3),
            ("несовместимые", cases[2].3),
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
