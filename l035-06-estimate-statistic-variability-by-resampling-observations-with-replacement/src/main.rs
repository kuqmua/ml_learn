// Урок 06.6. Изменчивость оценки: повторный набор выборок с возвращением наблюдений.
// Связь с принятой терминологией: Повторные выборки с возвращением из наблюдений.
// Зачем здесь эта тема: Неопределённость можно оценить повторными выборками без новой ручной
//   формулы для каждой статистики.
// Почему код устроен так: Выбираем наблюдения с возвращением, чтобы каждая повторная выборка имела
//   исходный размер.
// Представь: Из [2, 4, 6] повторная выборка может быть [2, 2, 6]: повторение разрешено.
//
// Что изучаем: Bootstrap.
// Зачем это нужно: Повторная выборка с возвращением показывает, как меняется оценка при перестановке
// наблюдений.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let values: [f64; 3] = [2.0, 4.0, 6.0];
    let resamples: [[usize; 3]; 4] = [[0, 1, 2], [0, 0, 2], [1, 2, 2], [0, 1, 1]];
    assert!(!values.is_empty(), "исходная выборка не должна быть пустой");
    for indices in resamples {
        assert!(
            !indices.is_empty(),
            "повторная выборка не должна быть пустой"
        );
        assert!(
            indices.iter().all(|&index| index < values.len()),
            "индекс выходит за границы исходной выборки"
        );
        let mut sum_of_resampled_values: f64 = 0.0;
        for index in indices {
            sum_of_resampled_values += values[index];
        }
        let _mean: f64 = sum_of_resampled_values / indices.len() as f64;
    }

    plot_means_of_samples_drawn_with_replacement(values, resamples);
}

// Строим график по результатам урока.
fn plot_means_of_samples_drawn_with_replacement(values: [f64; 3], resamples: [[usize; 3]; 4]) {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Bootstrap: средние повторных выборок",
        "номер выборки",
        "среднее",
        &[lesson_visualization::Series {
            name: "среднее",

            points: &resamples
                .iter()
                .enumerate()
                .map(|(item_index, indices)| {
                    (
                        (item_index + 1) as f64,
                        indices
                            .iter()
                            .map(|&sample_index| values[sample_index])
                            .sum::<f64>()
                            / indices.len() as f64,
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
