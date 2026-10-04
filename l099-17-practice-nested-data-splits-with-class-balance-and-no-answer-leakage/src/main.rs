// Урок 17.5. Практика: вложенное разделение данных с сохранением долей классов и без подсказок из проверочных ответов.
// Зачем здесь эта тема: Стратификация, подбор параметров и подготовка признаков должны соблюдаться
//   на обоих уровнях проверки.
// Почему код устроен так: Вкладываем весь конвейер во внутренний цикл и оставляем внешний блок
//   нетронутым.
// Представь: Для каждого внешнего проверочного блока заново выбираем настройки и учим
//   преобразования на оставшихся данных.
//
// Что повторяем вместе: k-fold, стратификация, nested evaluation, утечка в preprocessing.
// Зачем это нужно: Кросс-валидация использует несколько разбиений для выбора параметра, сохраняя test для
//   итоговой оценки.
// Что показывает программа: Задаём данные для выбора числа соседей. Сравниваем значения гиперпараметра на
//   кросс-валидации. После выбора параметра один раз оцениваем качество на test.
// Что проверить при изменении примера: Каждый объект ровно один раз попадает в validation; test
//   используется один раз после выбора модели.
// Дополнительная практика: Реализуй k-fold подбор одного гиперпараметра для модели из предыдущих уроков.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    /// Метод ближайших соседей: сортируем обучающие значения по расстоянию и выбираем большинство среди заданного числа ближайших.
    fn choose_majority_class_among_nearest_training_values(
        training_examples: &[(f64, bool)],

        feature_value: f64,

        neighbor_count: usize,
    ) -> bool {
        assert!(
            neighbor_count > 0 && neighbor_count <= training_examples.len(),
            "число соседей должно быть от 1 до числа обучающих примеров"
        );
        let mut sorted_neighbors: Vec<(f64, bool)> = training_examples
            .iter()
            .map(|&(training_feature, target)| {
                (
                    (|| -> f64 {
                        let value: f64 = training_feature - feature_value;
                        if value < 0.0 { -value } else { value }
                    })(),
                    target,
                )
            })
            .collect();
        sorted_neighbors
            .sort_by(|left_result, right_result| left_result.0.total_cmp(&right_result.0));
        sorted_neighbors
            .iter()
            .take(neighbor_count)
            .filter(|(_, target)| *target)
            .count()
            * 2
            > neighbor_count
    }

    let training_examples: [(f64, bool); 9] = [
        (0.0, false),
        (1.0, false),
        (2.0, false),
        (3.0, true),
        (4.0, true),
        (5.0, true),
        (6.0, true),
        (7.0, false),
        (8.0, false),
    ];
    let best: (usize, f64) = [1, 3, 5]
        .into_iter()
        .map(|neighbor_count| {
            (
                neighbor_count,
                (|| -> f64 {
                    let data: &[(f64, bool)] = &training_examples;
                    let neighbor_count: usize = neighbor_count;
                    let fold_count: usize = 3;
                    assert!(
                        data.len() >= fold_count,
                        "для каждого блока нужен хотя бы один пример"
                    );
                    let mut correct_predictions: usize = 0;
                    for fold in 0..fold_count {
                        let training_examples: Vec<(f64, bool)> = data
                            .iter()
                            .enumerate()
                            .filter(|(sample_index, _)| sample_index % fold_count != fold)
                            .map(|(_, record)| *record)
                            .collect();
                        for (_, record) in data
                            .iter()
                            .enumerate()
                            .filter(|(sample_index, _)| sample_index % fold_count == fold)
                        {
                            correct_predictions += usize::from(
                                choose_majority_class_among_nearest_training_values(
                                    &training_examples,
                                    record.0,
                                    neighbor_count,
                                ) == record.1,
                            );
                        }
                    }
                    correct_predictions as f64 / data.len() as f64
                })(),
            )
        })
        .max_by(|left_result, right_result| left_result.1.total_cmp(&right_result.1))
        .unwrap();
    let test: [(f64, bool); 2] = [(2.5, false), (5.5, true)];
    let correct_prediction_share: f64 = test
        .iter()
        .filter(|&&(feature_value, target)| {
            choose_majority_class_among_nearest_training_values(
                &training_examples,
                feature_value,
                best.0,
            ) == target
        })
        .count() as f64
        / test.len() as f64;
    let _ = (&(best.0), &(best.1));

    // Выполняем вычисления из примера.
    let _ = (best, correct_prediction_share);
}
