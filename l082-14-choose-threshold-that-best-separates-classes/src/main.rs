// Урок 14.3. Выбор порога, который лучше всего разделяет классы.
// Связь с принятой терминологией: Жадный выбор порога для разбиения дерева решений.
// Зачем здесь эта тема: Мера нечистоты сама не строит дерево; нужно найти порог, который лучше
//   разделяет метки.
// Почему код устроен так: Перебираем кандидаты и считаем взвешенную нечистоту дочерних групп.
// Представь: Порог «признак < 5» полезен, если слева и справа после него классы стали однороднее.
//
// Что изучаем: Жадный выбор разбиения.
// Зачем это нужно: На каждом шаге дерево выбирает порог с наименьшей взвешенной нечистотой, не перебирая
// всё дерево целиком.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `data` для следующего шага примера.");
    let data: [(f64, bool); 4] = [(1.0, false), (2.0, false), (3.0, true), (4.0, true)];
    lesson_trace::trace_step!(data);
    lesson_trace::trace_note!("Создаём изменяемое значение `best` для следующих операций.");
    let mut best: (f64, f64) = (f64::INFINITY, 0.0);
    lesson_trace::trace_step!(best);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    for threshold in [1.5, 2.5, 3.5] {
        lesson_trace::trace_step!(threshold);
        lesson_trace::trace_note!(
            "Преобразуем входные данные и сохраняем полученную коллекцию в `left`."
        );
        let left: Vec<&(f64, bool)> = data.iter().filter(|sample| sample.0 < threshold).collect();
        lesson_trace::trace_step!(left);
        lesson_trace::trace_note!(
            "Преобразуем входные данные и сохраняем полученную коллекцию в `right`."
        );
        let right: Vec<&(f64, bool)> = data.iter().filter(|sample| sample.0 >= threshold).collect();
        lesson_trace::trace_step!(right);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            !left.is_empty() && !right.is_empty(),
            "порог должен оставлять примеры с обеих сторон"
        );
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `left_positive` для следующих операций."
        );
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        let left_positive: f64 =
            left.iter().filter(|sample| sample.1).count() as f64 / left.len() as f64;
        lesson_trace::trace_step!(left_positive);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `right_positive` для следующих операций."
        );
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        let right_positive: f64 =
            right.iter().filter(|sample| sample.1).count() as f64 / right.len() as f64;
        lesson_trace::trace_step!(right_positive);
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `left_gini`.");
        let left_gini: f64 = 2.0 * left_positive * (1.0 - left_positive);
        lesson_trace::trace_step!(left_gini);
        lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `right_gini`.");
        let right_gini: f64 = 2.0 * right_positive * (1.0 - right_positive);
        lesson_trace::trace_step!(right_gini);
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `score` для следующих операций."
        );
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        let score: f64 =
            (left.len() as f64 * left_gini + right.len() as f64 * right_gini) / data.len() as f64;
        lesson_trace::trace_step!(score);
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        if score < best.0 {
            lesson_trace::trace_note!(
                "Присваиваем вычисленное значение соответствующей переменной или полю."
            );
            best = (score, threshold);
            lesson_trace::trace_step!(best);
        }
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("лучший порог={}, Gini={}", best.1, best.0);

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_weighted_class_mixing_for_different_thresholds();
}

// Строим график по результатам урока.
fn plot_weighted_class_mixing_for_different_thresholds() {
    lesson_trace::trace_note!("Значения из этого урока на графике.");
    let greedy_split_points: Vec<(f64, f64)> =
        [(1.5, 1.0 / 3.0), (2.5, 0.0), (3.5, 1.0 / 3.0)].to_vec();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Чистота разбиения",
        "порог",
        "взвешенный Gini",
        &[lesson_visualization::Series {
            name: "данные −−++",

            points: &greedy_split_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
