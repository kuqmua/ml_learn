// Урок 15.5. Практика: обучение моделей на повторных выборках и голосование за класс.
// Связь с принятой терминологией: Ансамбль с выборками с возвращением и голосованием.
// Зачем здесь эта тема: Bagging соединяет bootstrap, независимые модели и голосование для
//   уменьшения нестабильности.
// Почему код устроен так: На одном малом наборе показываем, как повторные строки меняют модели и
//   общий ответ.
// Представь: Сначала создаём несколько выборок, затем обучаем модель на каждой и объединяем ответы.
//
// Что повторяем вместе: bagging, bootstrap, majority vote, bias/variance.
// Зачем это нужно: Ансамбль объединяет несколько простых моделей, обученных на разных выборках, чтобы
//   снизить зависимость от одной модели.
// Что показывает программа: Берём обучающие точки для нескольких базовых моделей. Каждый stump обучаем на
//   своей bootstrap-выборке. Объединяем прогнозы моделей голосованием большинства.
// Что проверить при изменении примера: Сравни одно дерево с ансамблем на одинаковом split; фиксируй seed
//   каждого дерева.
// Дополнительная практика: Собери несколько деревьев на bootstrap-выборках и усредни прогнозы.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let data: [(f64, bool); 6] = [
        (0., false),
        (1., false),
        (2., true),
        (3., true),
        (4., false),
        (5., true),
    ];

    let models: Vec<(f64, bool)> = (1..=9)
        .map(|seed| {
            (|| -> (f64, bool) {
                let data: &[(f64, bool)] = &(|| -> Vec<(f64, bool)> {
                    let data: &[(f64, bool)] = &data;
                    let seed: u64 = seed;
                    let mut generator_state: u64 = seed;
                    (0..data.len())
                        .map(|_| {
                            generator_state = generator_state
                                .wrapping_mul(6364136223846793005)
                                .wrapping_add(1);
                            data[(generator_state as usize) % data.len()]
                        })
                        .collect()
                })();
                let mut best: (f64, f64, bool) = (f64::INFINITY, 0., false);
                for &(candidate_threshold, _) in data {
                    for reverse in [false, true] {
                        let errors: f64 = data
                            .iter()
                            .filter(|&&(feature_value, label)| {
                                ((feature_value >= candidate_threshold) ^ reverse) != label
                            })
                            .count() as f64;
                        if errors < best.0 {
                            best = (errors, candidate_threshold, reverse);
                        }
                    }
                }
                (best.1, best.2)
            })()
        })
        .collect();

    assert!(
        !models.is_empty(),
        "для сравнения нужна хотя бы одна модель"
    );
    for feature_value in [0.5, 2.5, 4.5] {
        let _first_model: bool = (feature_value >= models[0].0) ^ models[0].1;
        let votes: usize = models
            .iter()
            .filter(|&&model| {
                (|| -> bool {
                    let model: (f64, bool) = model;
                    let feature_value: f64 = feature_value;
                    (feature_value >= model.0) ^ model.1
                })()
            })
            .count();
        let _ = (&(models.len()), &(votes * 2 > models.len()));
    }

    plot_thresholds_learned_by_resampled_models(models);
}

// Строим график по результатам урока.
fn plot_thresholds_learned_by_resampled_models(models: std::vec::Vec<(f64, bool)>) {
    let ensembles_points: Vec<(f64, f64)> = models
        .iter()
        .enumerate()
        .map(|(item_index, (threshold, _))| (item_index as f64, *threshold))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Пороги моделей ансамбля",
        "номер модели",
        "порог",
        &[lesson_visualization::Series {
            name: "пороги",

            points: &ensembles_points,
        }],
    )
    .expect("не удалось сохранить график");
}
