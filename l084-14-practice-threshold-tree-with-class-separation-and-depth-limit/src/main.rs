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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Шаг: Создаём одномерную задачу с явной границей классов.");
    let data: [(f64, bool); 4] = [(1., false), (2., false), (3., true), (4., true)];
    trace_step!(data);

    trace_note!(
        "Автоматически получаем стандартные реализации перечисленных трейтов для этого типа."
    );
    trace_note!("Описываем тип `Tree`, чтобы явно хранить состояние и допустимые варианты.");
    trace_note!("Лист хранит итоговую метку класса.");
    trace_note!("Разветвление хранит порог и два дочерних поддерева.");
    trace_note!("Поле `threshold` соответствующее значение в составе структуры.");
    trace_note!("Поле `left` соответствующее значение в составе структуры.");
    trace_note!("Поле `right` соответствующее значение в составе структуры.");
    #[derive(Debug)]
    enum Tree {
        Leaf(bool),

        Split {
            threshold: f64,

            left: Box<Tree>,

            right: Box<Tree>,
        },
    }

    trace_note!(
        "Объявляем повторно используемое вычисление `calculate_class_mixing_as_twice_positive_share_times_negative_share`; параметры ниже задают его входы."
    );
    /// Нечистота Джини для двух классов: 2·p·(1−p), где p — доля положительных меток.
    fn calculate_class_mixing_as_twice_positive_share_times_negative_share(
        data: &[(f64, bool)],
    ) -> f64 {
        trace_note!("Отдельно обрабатываем пустой набор, чтобы избежать неверного расчёта.");
        if data.is_empty() {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return 0.;
        }
        trace_note!(
            "Сохраняем рассчитанное значение `positive_class_share` для следующих операций."
        );
        trace_note!("Долю объектов одного класса среди всех объектов называют fraction.");
        trace_note!("Делим значения, получая нормированную величину или среднее.");
        let positive_class_share: f64 =
            data.iter().filter(|(_, label)| *label).count() as f64 / data.len() as f64;
        trace_step!(positive_class_share);
        trace_note!("Умножаем величины согласно используемой формуле.");
        2. * positive_class_share * (1. - positive_class_share)
    }
    trace_note!("Ищем порог с минимальной взвешенной нечистотой и строим дочерние узлы.");
    /// Дерево решений: выбираем порог с наименьшей взвешенной нечистотой Джини и повторяем до ограничения глубины.
    fn build_threshold_tree_by_minimizing_weighted_class_mixing(
        data: &[(f64, bool)],
        remaining_depth: usize,
    ) -> Tree {
        trace_note!("Считаем число положительных меток, чтобы проверить чистоту узла.");
        let positive_count: usize = data.iter().filter(|(_, label)| *label).count();
        trace_step!(positive_count);
        trace_note!("Чистый узел или достигнутый предел глубины превращаем в лист.");
        if remaining_depth == 0 || positive_count == 0 || positive_count == data.len() {
            trace_note!("Завершаем текущий расчёт и возвращаем найденное значение.");
            return Tree::Leaf(positive_count * 2 >= data.len());
        }
        trace_note!("Извлекаем значения признака и сортируем их для поиска возможных порогов.");
        let mut sorted_feature_values: Vec<f64> = data.iter().map(|sample| sample.0).collect();
        trace_step!(sorted_feature_values);
        trace_note!("Сортируем значения в порядке, заданном функцией сравнения.");
        sorted_feature_values.sort_by(f64::total_cmp);
        trace_note!("Инициализируем изменяемый накопитель `best_split` начальным состоянием.");
        let mut best_split: Option<(f64, f64)> = None;
        trace_step!(best_split);
        trace_note!("Кандидатами служат середины между соседними значениями признака.");
        for pair in sorted_feature_values.windows(2) {
            trace_step!(pair);
            trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
            if pair[0] == pair[1] {
                trace_note!("Пропускаем текущий элемент и переходим к следующему.");
                continue;
            }
            trace_note!(
                "Нормируем или усредняем величину делением и сохраняем её в `candidate_threshold`."
            );
            let candidate_threshold: f64 = (pair[0] + pair[1]) / 2.;
            trace_step!(candidate_threshold);
            trace_note!("Сохраняем рассчитанное значение `left_samples` для следующих операций.");
            trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
            trace_note!("Копируем значения из ссылок, чтобы получить самостоятельные элементы.");
            trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let left_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 < candidate_threshold)
                .collect();
            trace_step!(left_samples);
            trace_note!("Сохраняем рассчитанное значение `right_samples` для следующих операций.");
            trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
            trace_note!("Копируем значения из ссылок, чтобы получить самостоятельные элементы.");
            trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let right_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 >= candidate_threshold)
                .collect();
            trace_step!(right_samples);
            trace_note!("Считаем количество элементов и сохраняем его в `score`.");
            trace_note!("Добавляем этот член в составное арифметическое выражение.");
            trace_note!("Умножаем величины согласно используемой формуле.");
            trace_note!("Делим значения, получая нормированную величину или среднее.");
            let score: f64 = (left_samples.len() as f64
                * calculate_class_mixing_as_twice_positive_share_times_negative_share(
                    &left_samples,
                )
                + right_samples.len() as f64
                    * calculate_class_mixing_as_twice_positive_share_times_negative_share(
                        &right_samples,
                    ))
                / data.len() as f64;
            trace_step!(score);
            trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
            if best_split.is_none_or(|(previous_score, _)| score < previous_score) {
                trace_note!("Обновляем `best_split` результатом текущего шага.");
                best_split = Some((score, candidate_threshold));
                trace_step!(best_split);
            }
        }
        trace_note!("По лучшему порогу рекурсивно строим два дочерних поддерева.");
        if let Some((_, threshold)) = best_split {
            trace_note!("Сохраняем рассчитанное значение `left_samples` для следующих операций.");
            trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
            trace_note!("Копируем значения из ссылок, чтобы получить самостоятельные элементы.");
            trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let left_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 < threshold)
                .collect();
            trace_step!(left_samples);
            trace_note!("Сохраняем рассчитанное значение `right_samples` для следующих операций.");
            trace_note!("Перебираем элементы по ссылке, не копируя исходную коллекцию.");
            trace_note!("Копируем значения из ссылок, чтобы получить самостоятельные элементы.");
            trace_note!("Оставляем только элементы, прошедшие указанную проверку.");
            trace_note!("Собираем элементы итератора в итоговую коллекцию.");
            let right_samples: Vec<(f64, bool)> = data
                .iter()
                .copied()
                .filter(|sample| sample.0 >= threshold)
                .collect();
            trace_step!(right_samples);
            trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
            trace_note!("Используем ранее рассчитанное значение `threshold` в текущем выражении.");
            trace_note!("Заполняем поле `left` соответствующим рассчитанным значением.");
            trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
            trace_note!("Заполняем поле `right` соответствующим рассчитанным значением.");
            trace_note!("Передаём данные по ссылке или разыменовываем их для следующей операции.");
            trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
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
            trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
            trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
            Tree::Leaf(positive_count * 2 >= data.len())
        }
    }

    trace_note!("Шаг: Обучаем дерево, выбирая порог по уменьшению неоднородности.");
    let tree: Tree = build_threshold_tree_by_minimizing_weighted_class_mixing(&data, 2);
    trace_step!(tree);
    trace_note!(
        "Объявляем повторно используемое вычисление `predict_class_by_following_threshold_branches_to_leaf`; параметры ниже задают его входы."
    );
    /// Прогноз дерева: сравниваем признак с порогами, идём по ветвям и возвращаем класс листа.
    fn predict_class_by_following_threshold_branches_to_leaf(
        tree: &Tree,
        feature_value: f64,
    ) -> bool {
        trace_note!("Разбираем каждый возможный вариант значения отдельно.");
        trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        trace_note!("Обрабатываем этот вариант структуры данных отдельным правилом.");
        trace_note!("Используем ранее рассчитанное значение `threshold` в текущем выражении.");
        trace_note!("Используем ранее рассчитанное значение `left` в текущем выражении.");
        trace_note!("Используем ранее рассчитанное значение `right` в текущем выражении.");
        trace_note!("Выполняем действие для этого варианта данных.");
        trace_note!("Проверяем условие и выбираем соответствующую ветку алгоритма.");
        trace_note!("Используем ранее рассчитанное значение `feature_value` в текущем выражении.");
        match tree {
            Tree::Leaf(label) => *label,

            Tree::Split {
                threshold,

                left,

                right,
            } => predict_class_by_following_threshold_branches_to_leaf(
                if feature_value < *threshold {
                    trace_note!(
                        "Используем ранее рассчитанное значение `left` в текущем выражении."
                    );
                    left
                } else {
                    trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
                    trace_note!(
                        "Используем ранее рассчитанное значение `right` в текущем выражении."
                    );
                    right
                },
                feature_value,
            ),
        }
    }

    trace_note!("Шаг: Проверяем прогноз на новой точке и печатаем структуру дерева.");
    trace_note!("Задаём шаблон строки: плейсхолдеры ниже заменятся рассчитанными значениями.");
    trace_note!("Вызываем нужное вычисление с подготовленными аргументами.");
    println!(
        "{tree:?}; x=3.5 -> {}",
        predict_class_by_following_threshold_branches_to_leaf(&tree, 3.5)
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_predicted_leaf_class_for_changing_feature(tree);

    trace_note!("Строим график по результатам урока.");
    fn plot_predicted_leaf_class_for_changing_feature(tree: Tree) {
        trace_note!("Наглядное представление вычислений сводной практики.");
        trace_note!("Преобразуем каждый элемент в новое значение.");
        trace_note!("Собираем результаты в коллекцию.");
        let decision_tree_points: Vec<(f64, f64)> = (0..=50)
            .map(|plot_step_index| {
                trace_note!("Сохраняем результат этого шага в `horizontal_value`.");
                let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                trace_note!("Добавляем пару значений для сравнения или построения графика.");
                (
                    horizontal_value,
                    f64::from(predict_class_by_following_threshold_branches_to_leaf(
                        &tree,
                        horizontal_value,
                    )),
                )
            })
            .collect();
        trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
        trace_note!("Передаём путь к каталогу текущего урока.");
        trace_note!("Указываем имя SVG-файла.");
        trace_note!("Указываем заголовок графика.");
        trace_note!("Указываем подпись горизонтальной оси.");
        trace_note!("Указываем подпись вертикальной оси.");
        trace_note!("Передаём ряды или значения для отрисовки графика.");
        trace_note!("Указываем подпись этого ряда в легенде.");
        trace_note!("Передаём рассчитанные координаты точек.");
        trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
        trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
        println!("график: {}", chart.display());
    }
}
