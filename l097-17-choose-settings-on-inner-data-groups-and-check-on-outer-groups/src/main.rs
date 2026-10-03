// Урок 17.3. Выбор настроек на внутренних группах данных и проверка на внешних.
// Зачем здесь эта тема: Выбор параметров по тем же проверочным данным делает оценку оптимистичной.
// Почему код устроен так: Внутренние блоки выбирают параметры, а внешние оценивают уже выбранный
//   алгоритм.
// Представь: Если выбрали лучший параметр по validation, тот же validation уже не даёт независимой
//   оценки качества.
//
// Что изучаем: Вложенная оценка.
// Зачем это нужно: Внутреннее разбиение выбирает гиперпараметр, а внешнее измеряет качество выбранной
// процедуры на новых данных.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let inner_scores: [(usize, f64); 3] = [(1, 0.70), (3, 0.85), (5, 0.80)];
    assert!(
        !inner_scores.is_empty(),
        "для выбора k нужна хотя бы одна оценка"
    );
    let outer_truth: [bool; 4] = [true, false, true, false];
    let outer_predictions: [bool; 4] = [true, false, false, false];
    assert_eq!(
        outer_truth.len(),
        outer_predictions.len(),
        "число прогнозов должно совпадать с числом ответов"
    );
    assert!(
        !outer_truth.is_empty(),
        "для внешней оценки нужен хотя бы один пример"
    );

    let _: f64 = (0..outer_truth.len())
        .filter(|&index| outer_truth[index] == outer_predictions[index])
        .count() as f64
        / outer_truth.len() as f64;
    let _ = inner_scores
        .iter()
        .max_by(|first_candidate, second_candidate| {
            first_candidate.1.total_cmp(&second_candidate.1)
        })
        .unwrap()
        .0;

    // Выполняем вычисления из примера.
    let _ = inner_scores;
}
