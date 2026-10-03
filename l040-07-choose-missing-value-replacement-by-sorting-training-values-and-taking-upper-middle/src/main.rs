// Урок 07.4. Заполнение пропусков серединой отсортированных обучающих значений.
// Связь с принятой терминологией: Заполнение пропусков признака медианой обучающих данных.
// Зачем здесь эта тема: После разбиения пропуски нужно заполнить; медиана validation или test
//   внесла бы в обучение чужую информацию.
// Почему код устроен так: Считаем медиану только на train и применяем её к остальным частям без
//   пересчёта.
// Представь: Если в validation появился пропуск, подставляем медиану train; изменение validation не
//   должно менять эту медиану.
// Медиану вычисляем только по train, затем применяем к validation.

/// Медиана для нечётного числа значений: сортируем и берём середину. При чётном числе эта реализация берёт верхний средний элемент.

fn choose_missing_value_replacement_by_sorting_training_values_and_taking_upper_middle(
    values: &[f64],
) -> f64 {
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[sorted.len() / 2]
}
fn main() {
    let training_data: [Option<f64>; 4] = [Some(1.0), None, Some(3.0), Some(5.0)];
    let validation: [Option<f64>; 2] = [None, Some(100.0)];

    let replacement: f64 =
        choose_missing_value_replacement_by_sorting_training_values_and_taking_upper_middle(
            &training_data.iter().flatten().copied().collect::<Vec<_>>(),
        );
    let _: [f64; 4] = training_data.map(|value| value.unwrap_or(replacement));
    assert_eq!(replacement, 3.0);
    assert_eq!(
        validation.map(|value| value.unwrap_or(replacement)),
        [3.0, 100.0]
    );
}
