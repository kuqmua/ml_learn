// Урок 14.1. Отрицательная сумма долей классов, умноженных на их логарифмы.
// Связь с принятой терминологией: Энтропия долей классов в узле дерева решений.
// Зачем здесь эта тема: Дерево выбирает разбиение по смешанности классов в узле; энтропия даёт одну
//   меру смешанности.
// Почему код устроен так: Считаем вклад каждой доли класса и сравниваем чистый и смешанный узлы.
// Представь: Узел только с классом A чистый; смесь A и B даёт большую энтропию.
//
// У чистого узла энтропия равна нулю, при долях 50/50 она максимальна.
// Нулевую долю пропускаем: предел p·log(p) при p→0 равен нулю.

fn main() {
    lesson_trace::enable();
    // Повторяем расчёт для каждого элемента последовательности.
    // Долю объектов одного класса среди всех объектов называют fraction.
    for (description, positive_class_share) in [
        // Добавляем пару значений для сравнения или построения графика.
        ("только отрицательный класс", 0.0),
        // Добавляем пару значений для сравнения или построения графика.
        ("четверть положительных", 0.25),
        // Добавляем пару значений для сравнения или построения графика.
        ("классы поровну", 0.5),
        // Добавляем пару значений для сравнения или построения графика.
        ("только положительный класс", 1.0),
    ] {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(positive_class_share);
        // Проверяем ожидаемое свойство учебного примера.
        assert!((0.0..=1.0).contains(&positive_class_share));
        // Сохраняем результат этого шага в `negative_class_share`.
        let negative_class_share: f64 = 1.0 - positive_class_share;
        lesson_trace::trace_step!(negative_class_share);
        // Сохраняем результат этого шага в `uncertainty_measure`.
        // Меру неопределённости распределения называют entropy.
        let mut uncertainty_measure: f64 = 0.0;
        lesson_trace::trace_step!(uncertainty_measure);
        // Повторяем расчёт для каждого элемента последовательности.
        for probability in [positive_class_share, negative_class_share] {
            lesson_trace::trace_step!(probability);
            // Выбираем дальнейший шаг по выполнению условия.
            if probability > 0.0 {
                // Сохраняем результат этого шага в `ratio`.
                let ratio: f64 = (probability - 1.0) / (probability + 1.0);
                lesson_trace::trace_step!(ratio);
                // Сохраняем результат этого шага в `term`.
                let mut term: f64 = ratio;
                lesson_trace::trace_step!(term);
                // Сохраняем результат этого шага в `logarithm`.
                let mut logarithm: f64 = 0.0;
                lesson_trace::trace_step!(logarithm);
                // Повторяем расчёт для каждого элемента последовательности.
                for odd_divisor in (1..=99).step_by(2) {
                    lesson_trace::trace_step!(odd_divisor);
                    // Обновляем значение результатом текущего вычисления.
                    logarithm += term / odd_divisor as f64;
                    lesson_trace::trace_step!(logarithm);
                    // Обновляем значение результатом текущего вычисления.
                    term *= ratio * ratio;
                    lesson_trace::trace_step!(term);
                }
                // Переводим натуральный логарифм в логарифм по основанию 2.
                uncertainty_measure -= probability * (2.0 * logarithm) / std::f64::consts::LN_2;
                lesson_trace::trace_step!(uncertainty_measure);
            }
        }
        // Проверяем ожидаемое свойство учебного примера.
        assert!(uncertainty_measure >= -1e-10 && uncertainty_measure <= 1.0 + 1e-10);
        // Выбираем дальнейший шаг по выполнению условия.
        if positive_class_share == 0.0 || positive_class_share == 1.0 {
            // Проверяем ожидаемое свойство учебного примера.
            assert!(uncertainty_measure.abs() < 1e-10);
        }
        // Выбираем дальнейший шаг по выполнению условия.
        if positive_class_share == 0.5 {
            // Проверяем ожидаемое свойство учебного примера.
            assert!((uncertainty_measure - 1.0).abs() < 1e-10);
        }
        // Печатаем рассчитанные значения для проверки примера.
        println!("{description}: доля={positive_class_share}, энтропия={uncertainty_measure:.3}");
    }

    // Построение графика вынесено из основного кода урока.
    lesson_trace::disable();
    plot_class_entropy_as_negative_sum_of_class_shares_times_their_logarithms();
}

// Строим график по результатам урока.
fn plot_class_entropy_as_negative_sum_of_class_shares_times_their_logarithms() {
    // График величин и зависимостей, изученных в этом уроке.
    let uncertainty_measure_points: Vec<(f64, f64)> = (1..100)
        // Преобразуем каждый элемент в новое значение.
        .map(|plot_step_index| {
            // Сохраняем результат этого шага в `probability`.
            let probability: f64 = plot_step_index as f64 / 100.0;
            // Добавляем пару значений для сравнения или построения графика.
            (
                probability,
                -probability * probability.log2()
                    - (1.0 - probability) * (1.0 - probability).log2(),
            )
        })
        // Собираем результаты в коллекцию.
        .collect();
    // Строим график по рассчитанным значениям и сохраняем его как SVG.
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        // Передаём путь к каталогу текущего урока.
        env!("CARGO_MANIFEST_DIR"),
        // Указываем имя SVG-файла.
        "lesson-chart",
        // Указываем заголовок графика.
        "Энтропия бинарного класса",
        // Указываем подпись горизонтальной оси.
        "доля положительных",
        // Указываем подпись вертикальной оси.
        "энтропия",
        // Передаём ряды или значения для отрисовки графика.
        &[lesson_visualization::Series {
            // Указываем подпись этого ряда в легенде.
            name: "H(p)",
            // Передаём рассчитанные координаты точек.
            points: &uncertainty_measure_points,
        }],
    )
    // Прерываем пример с понятной ошибкой, если SVG не удалось записать.
    .expect("не удалось сохранить график");
    // Печатаем путь к созданному SVG, чтобы его можно было открыть.
    println!("график: {}", chart.display());
}
