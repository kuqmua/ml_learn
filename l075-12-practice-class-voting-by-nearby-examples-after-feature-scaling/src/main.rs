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
    /// Возводим число в квадрат обычным умножением.
    /// Вместо этой учебной обёртки можно написать `value * value` или `value.powi(2)`.
    /// Само умножение не обязательно медленнее библиотечного метода.
    fn calc_square_by_multiplying_number_by_itself(value: f64) -> f64 {
        value * value
    }

    let training_examples: [([f64; 2], bool); 4] = [
        ([0., 0.], false),
        ([0., 1.], false),
        ([2., 2.], true),
        ([2., 3.], true),
    ];
    for neighbor_count in [1, 3] {
        let _ = &((|| -> bool {
            let training_examples: &[([f64; 2], bool)] = &training_examples;

            let neighbor_count: usize = neighbor_count;

            assert!(neighbor_count > 0 && neighbor_count <= training_examples.len());

            let query_point: [f64; 2] = [1.8, 2.1];
            let mut nearest_neighbor_vote_uses_selected_count: Vec<(f64, bool)> = training_examples
                .iter()
                .map(|&(features, target)| {
                    (
                        (calc_square_by_multiplying_number_by_itself(features[0] - query_point[0])
                            + calc_square_by_multiplying_number_by_itself(
                                features[1] - query_point[1],
                            )),
                        target,
                    )
                })
                .collect();

            nearest_neighbor_vote_uses_selected_count.sort_by(|left_neighbor, right_neighbor| {
                left_neighbor.0.total_cmp(&right_neighbor.0)
            });

            nearest_neighbor_vote_uses_selected_count
                .iter()
                .take(neighbor_count)
                .filter(|(_, target)| *target)
                .count()
                * 2
                > neighbor_count
        })());
    }

    plot_training_points_by_class_and_query_point(training_examples);
}

// Строим график по результатам урока.
fn plot_training_points_by_class_and_query_point(training_examples: [([f64; 2], bool); 4]) {
    let query_point: [(f64, f64); 1] = [(1.8, 2.1)];
    lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Ближайшие соседи и запрос",
        "признак 1",
        "признак 2",
        &[
            lesson_visualization::Series {
                name: "класс 0",

                points: &training_examples
                    .iter()
                    .filter(|(_, target)| !*target)
                    .map(|(point, _)| (point[0], point[1]))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "класс 1",

                points: &training_examples
                    .iter()
                    .filter(|(_, target)| *target)
                    .map(|(point, _)| (point[0], point[1]))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "запрос",

                points: &query_point,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
