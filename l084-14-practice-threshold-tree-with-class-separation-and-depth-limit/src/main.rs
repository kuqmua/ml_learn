// Урок 14.5. Практика: дерево пороговых проверок с разделением классов и ограничением глубины.
// Связь с принятой терминологией: Дерево решений, меры нечистоты, разбиения и ограничение глубины.
// Зачем здесь эта тема: После критерия и порога нужен рекурсивный алгоритм с условием остановки.
// Почему код устроен так: Строим маленькое дерево с ограничением глубины и проверяем его прогнозы.
// Представь: В каждом узле выбираем порог, делим строки и прекращаем ветвление при достижении
//   ограничения глубины.
//
// Что повторяем вместе: энтропия, Gini, жадное разбиение, переобучение.
// Зачем это нужно: Дерево последовательно делит пространство признаков на области и хранит решение в
//   листьях.
// Что показывает программа: Создаём одномерную задачу с явной границей классов. Обучаем дерево, выбирая
//   порог по уменьшению неоднородности. Проверяем прогноз на новой точке и печатаем структуру дерева.
// Что проверить при изменении примера: Проверь чистый лист, константный признак и изменение train/test
//   метрик с глубиной.
// Дополнительная практика: Реализуй дерево для числовых признаков: выбор порога, max_depth, min_samples_leaf.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Шаг: Создаём одномерную задачу с явной границей классов.");
    let data: [(f64, bool); 4] = [(1., false), (2., false), (3., true), (4., true)];
    lesson_trace::trace_step!(data);

    lesson_trace::trace_note!(
        "Автоматически получаем стандартные реализации перечисленных трейтов для этого типа."
    );
    lesson_trace::trace_note!(
        "Описываем тип `Tree`, чтобы явно хранить состояние и допустимые варианты."
    );
    lesson_trace::trace_note!("Лист хранит итоговую метку класса.");
    lesson_trace::trace_note!("Разветвление хранит порог и два дочерних поддерева.");
    lesson_trace::trace_note!("Поле `threshold` соответствующее значение в составе структуры.");
    lesson_trace::trace_note!("Поле `left` соответствующее значение в составе структуры.");
    lesson_trace::trace_note!("Поле `right` соответствующее значение в составе структуры.");
    #[derive(Debug)]
    enum Tree {
        Leaf(bool),

        Split {
            threshold: f64,

            left: Box<Tree>,

            right: Box<Tree>,
        },
    }

    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `calculate_class_mixing_as_twice_positive_share_times_negative_share`; параметры ниже задают его входы."
    );
    /// Нечистота Джини для двух классов: 2·p·(1−p), где p — доля положительных меток.
    fn calculate_class_mixing_as_twice_positive_share_times_negative_share(
        data: &[(f64, bool)],
    ) -> f64 {
        lesson_trace::trace_note!(
            "Отдельно обрабатываем пустой набор, чтобы избежать неверного расчёта."
        );
        if data.is_empty() {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.;
        }
        lesson_trace::trace_note!(
            "Сохраняем рассчитанное значение `positive_class_share` для следующих операций."
        );
        lesson_trace::trace_note!(
            "Долю объектов одного класса среди всех объектов называют fraction."
        );
        lesson_trace::trace_note!("Делим значения, получая нормированную величину или среднее.");
        let positive_class_share: f64 =
            data.iter().filter(|(_, label)| *label).count() as f64 / data.len() as f64;
        lesson_trace::trace_step!(positive_class_share);
        lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
        2. * positive_class_share * (1. - positive_class_share)
    }
    lesson_trace::trace_note!(
        "Ищем порог с минимальной взвешенной нечистотой и строим дочерние узлы."
    );
    /// Дерево решений: выбираем порог с наименьшей взвешенной нечистотой Джини и повторяем до ограничения глубины.
    fn build_threshold_tree_by_minimizing_weighted_class_mixing(
        data: &[(f64, bool)],
        remaining_depth: usize,
    ) -> Tree {
        lesson_trace::trace_note!(
            "Считаем число положительных меток, чтобы проверить чистоту узла."
        );
        let positive_count: usize = data.iter().filter(|(_, label)| *label).count();
        lesson_trace::trace_step!(positive_count);
        lesson_trace::trace_note!("Чистый узел или достигнутый предел глубины превращаем в лист.");
        if remaining_depth == 0 || positive_count == 0 || positive_count == data.len() {
            lesson_trace::trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return Tree::Leaf(positive_count * 2 >= data.len());
        }
        lesson_trace::trace_note!(
            "Извлекаем значения признака и сортируем их для поиска возможных порогов."
        );
        let mut sorted_feature_values: Vec<f64> = data.iter().map(|sample| sample.0).collect();
        lesson_trace::trace_step!(sorted_feature_values);
        lesson_trace::trace_note!("Сортируем значения в порядке, заданном функцией сравнения.");
        sorted_feature_values.sort_by(f64::total_cmp);
        lesson_trace::trace_note!(
            "Инициализируем изменяемый накопитель `best_split` начальным состоянием."
        );
        let mut best_split: Option<(f64, f64)> = None;
        lesson_trace::trace_step!(best_split);
        lesson_trace::trace_note!(
            "Кандидатами служат середины между соседними значениями признака."
        );
        for pair in sorted_feature_values.windows(2) {
            lesson_trace::trace_step!(pair);
            lesson_trace::trace_note!(
                "Проверяем условие и выбираем соответствующую ветку алгоритма."
            );
            if pair[0] == pair[1] {
                lesson_trace::trace_note!("Пропускаем текущий элемент и переходим к следующему.");
                continue;
            }
            lesson_trace::trace_note!(
                "Нормируем или усредняем величину делением и сохраняем её в `candidate_threshold`."
            );
            let candidate_threshold: f64 = (pair[0] + pair[1]) / 2.;
            lesson_trace::trace_step!(candidate_threshold);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `left_samples` для следующих операций."
            );
            lesson_trace::trace_note!(
                "Перебираем элементы по ссылке, не копируя исходную коллекцию."
            );
            lesson_trace::trace_note!(
                "Копируем значения из ссылок, чтобы получить самостоятельные элементы."
            );
            lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let left_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 < candidate_threshold)
                .collect();
            lesson_trace::trace_step!(left_samples);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `right_samples` для следующих операций."
            );
            lesson_trace::trace_note!(
                "Перебираем элементы по ссылке, не копируя исходную коллекцию."
            );
            lesson_trace::trace_note!(
                "Копируем значения из ссылок, чтобы получить самостоятельные элементы."
            );
            lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let right_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 >= candidate_threshold)
                .collect();
            lesson_trace::trace_step!(right_samples);
            lesson_trace::trace_note!("Считаем количество элементов и сохраняем его в `score`.");
            lesson_trace::trace_note!("Добавляем этот член в составное арифметическое выражение.");
            lesson_trace::trace_note!("Умножаем величины согласно используемой формуле.");
            lesson_trace::trace_note!(
                "Делим значения, получая нормированную величину или среднее."
            );
            let score: f64 = (left_samples.len() as f64
                * calculate_class_mixing_as_twice_positive_share_times_negative_share(
                    &left_samples,
                )
                + right_samples.len() as f64
                    * calculate_class_mixing_as_twice_positive_share_times_negative_share(
                        &right_samples,
                    ))
                / data.len() as f64;
            lesson_trace::trace_step!(score);
            lesson_trace::trace_note!(
                "Проверяем условие и выбираем соответствующую ветку алгоритма."
            );
            if best_split.is_none_or(|(previous_score, _)| score < previous_score) {
                lesson_trace::trace_note!("Обновляем `best_split` результатом текущего шага.");
                best_split = Some((score, candidate_threshold));
                lesson_trace::trace_step!(best_split);
            }
        }
        lesson_trace::trace_note!("По лучшему порогу рекурсивно строим два дочерних поддерева.");
        if let Some((_, threshold)) = best_split {
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `left_samples` для следующих операций."
            );
            lesson_trace::trace_note!(
                "Перебираем элементы по ссылке, не копируя исходную коллекцию."
            );
            lesson_trace::trace_note!(
                "Копируем значения из ссылок, чтобы получить самостоятельные элементы."
            );
            lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let left_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 < threshold)
                .collect();
            lesson_trace::trace_step!(left_samples);
            lesson_trace::trace_note!(
                "Сохраняем рассчитанное значение `right_samples` для следующих операций."
            );
            lesson_trace::trace_note!(
                "Перебираем элементы по ссылке, не копируя исходную коллекцию."
            );
            lesson_trace::trace_note!(
                "Копируем значения из ссылок, чтобы получить самостоятельные элементы."
            );
            lesson_trace::trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            lesson_trace::trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let right_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 >= threshold)
                .collect();
            lesson_trace::trace_step!(right_samples);
            lesson_trace::trace_note!(
                "Обрабатываем этот вариант структуры данных отдельным правилом."
            );
            lesson_trace::trace_note!(
                "Используем ранее рассчитанное значение `threshold` в текущем выражении."
            );
            lesson_trace::trace_note!(
                "Заполняем поле `left` соответствующим рассчитанным значением."
            );
            lesson_trace::trace_note!(
                "Передаём данные по ссылке или разыменовываем их для следующей операции."
            );
            lesson_trace::trace_note!(
                "Складываем или вычитаем величины согласно используемой формуле."
            );
            lesson_trace::trace_note!(
                "Заполняем поле `right` соответствующим рассчитанным значением."
            );
            lesson_trace::trace_note!(
                "Передаём данные по ссылке или разыменовываем их для следующей операции."
            );
            lesson_trace::trace_note!(
                "Складываем или вычитаем величины согласно используемой формуле."
            );
            Tree::Split {
                threshold,

                left: Box::new(build_threshold_tree_by_minimizing_weighted_class_mixing(
                    &left_samples,
                    remaining_depth - 1,
                )),

                right: Box::new(build_threshold_tree_by_minimizing_weighted_class_mixing(
                    &right_samples,
                    remaining_depth - 1,
                )),
            }
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!(
                "Обрабатываем этот вариант структуры данных отдельным правилом."
            );
            Tree::Leaf(positive_count * 2 >= data.len())
        }
    }

    lesson_trace::trace_note!("Шаг: Обучаем дерево, выбирая порог по уменьшению неоднородности.");
    let tree: Tree = build_threshold_tree_by_minimizing_weighted_class_mixing(&data, 2);
    lesson_trace::trace_step!(tree);
    lesson_trace::trace_note!(
        "Объявляем повторно используемое вычисление `predict_class_by_following_threshold_branches_to_leaf`; параметры ниже задают его входы."
    );
    /// Прогноз дерева: сравниваем признак с порогами, идём по ветвям и возвращаем класс листа.
    fn predict_class_by_following_threshold_branches_to_leaf(
        tree: &Tree,
        feature_value: f64,
    ) -> bool {
        lesson_trace::trace_note!("Разбираем каждый возможный вариант значения отдельно.");
        lesson_trace::trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        lesson_trace::trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `threshold` в текущем выражении."
        );
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `left` в текущем выражении."
        );
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `right` в текущем выражении."
        );
        lesson_trace::trace_note!("Выполняем действие для этого варианта данных.");
        lesson_trace::trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        lesson_trace::trace_note!(
            "Используем ранее рассчитанное значение `feature_value` в текущем выражении."
        );
        match tree {
            Tree::Leaf(label) => *label,

            Tree::Split {
                threshold,

                left,

                right,
            } => predict_class_by_following_threshold_branches_to_leaf(
                if feature_value < *threshold {
                    lesson_trace::trace_note!(
                        "Используем ранее рассчитанное значение `left` в текущем выражении."
                    );
                    left
                } else {
                    lesson_trace::trace_note!(
                        "Обрабатываем случай, когда предыдущее условие не выполнено."
                    );
                    lesson_trace::trace_note!(
                        "Используем ранее рассчитанное значение `right` в текущем выражении."
                    );
                    right
                },
                feature_value,
            ),
        }
    }

    lesson_trace::trace_note!("Шаг: Проверяем прогноз на новой точке и печатаем структуру дерева.");
    lesson_trace::trace_note!(
        "Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями."
    );
    lesson_trace::trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    println!(
        "{tree:?}; x=3.5 -> {}",
        predict_class_by_following_threshold_branches_to_leaf(&tree, 3.5)
    );

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_predicted_leaf_class_for_changing_feature(tree);

    lesson_trace::trace_note!("Строим график по результатам урока.");
    fn plot_predicted_leaf_class_for_changing_feature(tree: Tree) {
        lesson_trace::trace_note!("Наглядное представление вычислений сводной практики.");
        lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
        lesson_trace::trace_note!("Собираем результаты в коллекцию.");
        let decision_tree_points: Vec<(f64, f64)> = (0..=50)
            .map(|plot_step_index| {
                lesson_trace::trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
                let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                lesson_trace::trace_note!(
                    "Добавляем пару значений для сравнения или построения графика."
                );
                (
                    horizontal_value,
                    f64::from(predict_class_by_following_threshold_branches_to_leaf(
                        &tree,
                        horizontal_value,
                    )),
                )
            })
            .collect();
        lesson_trace::trace_note!(
            "Строим график по рассчитанным значениям и сохраняем его как SVG."
        );
        lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
        lesson_trace::trace_note!("Указываем имя SVG-файла.");
        lesson_trace::trace_note!("Указываем заголовок графика.");
        lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
        lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
        lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
        lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
        lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
        lesson_trace::trace_note!(
            "Прерываем пример с понятной ошибкой, если SVG не удалось записать."
        );
        let chart: std::path::PathBuf = lesson_visualization::line_chart(
            env!("CARGO_MANIFEST_DIR"),
            "lesson-chart",
            "Решение дерева по признаку",
            "признак",
            "класс 1",
            &[lesson_visualization::Series {
                name: "прогноз",

                points: &decision_tree_points,
            }],
        )
        .expect("не удалось сохранить график");
        lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
