// Урок 12.5. Практика: выбор класса по ближайшим примерам после изменения масштаба признаков.
// Связь с принятой терминологией: Классификация kNN с масштабированием признаков и выбором k.
// Зачем здесь эта тема: Работа kNN зависит сразу от масштаба признаков, метрики расстояния и числа
//   соседей.
// Почему код устроен так: Собираем эти решения в один прогноз и сравниваем его с ожидаемой меткой.
// Представь: Сначала приводим признаки к масштабу, затем ищем ближайших и только после этого
//   голосуем.
//
// Что повторяем вместе: расстояния, масштабирование признаков, выбор k, стоимость предсказания.
// Зачем это нужно: Метод ближайших соседей делает прогноз по похожим размеченным объектам; число соседей
//   меняет границу решения.
// Что показывает программа: Задаём размеченные точки двух классов. Меняем число соседей и сравниваем
//   прогноз для одной точки.
// Что проверить при изменении примера: Сравни k=1 и большие k; продемонстрируй эффект признака с большой
//   шкалой.
// Дополнительная практика: Напиши k-NN классификатор с явным правилом разрешения ничьей.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Шаг: Задаём размеченные точки двух классов.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    lesson_trace::trace_note!("Составляем результат из вычисленных значений в указанном порядке.");
    let training_examples: [([f64; 2], bool); 4] = [
        ([0., 0.], false),
        ([0., 1.], false),
        ([2., 2.], true),
        ([2., 3.], true),
    ];
    lesson_trace::trace_step!(training_examples);

    lesson_trace::trace_note!("Учебные реализации математических операций для этого урока.");

    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calculate_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        value * value
    }

    lesson_trace::trace_note!("Шаг: Меняем число соседей и сравниваем прогноз для одной точки.");
    for neighbor_count in [1, 3] {
        lesson_trace::trace_step!(neighbor_count);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        lesson_trace::trace_note!(
            "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
        );
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
        lesson_trace::trace_note!(
            "Сортируем обучающие точки по расстоянию и голосуем среди ближайших."
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `training_examples`.");
        lesson_trace::trace_note!(
            "Создаём набор значений `query_point` для следующего шага примера."
        );
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `neighbor_count` для следующих операций."
        );
        lesson_trace::trace_note!("Проверяем обязательное условие до дальнейшего вычисления.");
        lesson_trace::trace_note!(
            "Создаём изменяемое значение `nearest_neighbor_vote_uses_selected_count` для следующих операций."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!("Преобразуем каждый элемент последовательности.");
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!(
            "Составляем результат из вычисленных значений в указанном порядке."
        );
        lesson_trace::trace_note!(
            "Складываем или вычитаем величины согласно используемой формуле."
        );
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `label` в текущем выражении."
        );
        lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
        lesson_trace::trace_note!("Сортируем значения в порядке, заданном функцией сравнения.");
        lesson_trace::trace_note!("Задаём параметры короткого локального вычисления.");
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `votes` для следующих операций."
        );
        lesson_trace::trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
        lesson_trace::trace_note!("Оставляем только заданное число лучших элементов.");
        lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
        lesson_trace::trace_note!("Подсчитываем число элементов после отбора.");
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        println!(
            "k={neighbor_count}, class={}",
            (|| -> bool {
                let training_examples: &[([f64; 2], bool)] = &training_examples;
                lesson_trace::trace_step!(training_examples);
                lesson_trace::trace_step!(training_examples);

                let query_point: [f64; 2] = [1.8, 2.1];
                lesson_trace::trace_step!(query_point);
                lesson_trace::trace_step!(query_point);

                let neighbor_count: usize = neighbor_count;
                lesson_trace::trace_step!(neighbor_count);
                lesson_trace::trace_step!(neighbor_count);

                assert!(neighbor_count > 0 && neighbor_count <= training_examples.len());

                let mut nearest_neighbor_vote_uses_selected_count: Vec<(f64, bool)> =
                    training_examples
                        .iter()
                        .map(|&(features, label)| {
                            (
                                (calculate_square_by_multiplying_number_by_itself(
                                    features[0] - query_point[0],
                                ) + calculate_square_by_multiplying_number_by_itself(
                                    features[1] - query_point[1],
                                )),
                                label,
                            )
                        })
                        .collect();
                lesson_trace::trace_step!(nearest_neighbor_vote_uses_selected_count);
                lesson_trace::trace_step!(nearest_neighbor_vote_uses_selected_count);

                nearest_neighbor_vote_uses_selected_count.sort_by(
                    |left_neighbor, right_neighbor| left_neighbor.0.total_cmp(&right_neighbor.0),
                );

                let votes: usize = nearest_neighbor_vote_uses_selected_count
                    .iter()
                    .take(neighbor_count)
                    .filter(|(_, label)| *label)
                    .count();
                lesson_trace::trace_step!(votes);
                lesson_trace::trace_step!(votes);

                votes * 2 > neighbor_count
            })()
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_training_points_by_class_and_query_point(training_examples);
}

// Строим график по результатам урока.
fn plot_training_points_by_class_and_query_point(training_examples: [([f64; 2], bool); 4]) {
    lesson_trace::trace_note!("Показываем именно обучающие точки и запрос из примера.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Оставляем элементы, отвечающие условию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let class_zero: Vec<(f64, f64)> = training_examples
        .iter()
        .filter(|(_, label)| !*label)
        .map(|(point, _)| (point[0], point[1]))
        .collect();
    lesson_trace::trace_note!("Собираем значения для `class_one` в коллекцию.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Оставляем элементы, отвечающие условию.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let class_one: Vec<(f64, f64)> = training_examples
        .iter()
        .filter(|(_, label)| *label)
        .map(|(point, _)| (point[0], point[1]))
        .collect();
    lesson_trace::trace_note!("Задаём учебные значения для `query_point`.");
    let query_point: [(f64, f64); 1] = [(1.8, 2.1)];
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Добавляем ряд данных с подписью к графику.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ближайшие соседи и запрос",
        "признак 1",
        "признак 2",
        &[
            lesson_visualization::Series {
                name: "класс 0",

                points: &class_zero,
            },
            lesson_visualization::Series {
                name: "класс 1",

                points: &class_one,
            },
            lesson_visualization::Series {
                name: "запрос",

                points: &query_point,
            },
        ],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
