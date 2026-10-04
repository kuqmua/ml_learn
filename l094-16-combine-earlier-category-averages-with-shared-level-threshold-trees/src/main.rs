// Урок 094. Кодируем категории по предыдущим ответам и обучаем деревья на остаточных ошибках.
// Это небольшая композиция двух идей, а не полная реализация CatBoost или ordered boosting.
use l091_16_predict_tree_output_by_choosing_leaf_with_shared_threshold_tests::ObliviousTree;
use l092_16_encode_categories_as_average_previous_targets_with_prior_weight::encode_categories_as_average_previous_targets_with_prior_weight;

fn main() {
    let categories = ["A", "B", "A", "B", "A", "B"];
    let targets = [1.0_f64, 0.0, 1.0, 0.0, 1.0, 0.0];
    let features = encode_categories_as_average_previous_targets_with_prior_weight(
        &categories,
        &targets,
        0.5,
        1.0,
    )
    .unwrap();
    let mut candidates = features.to_vec();
    candidates.sort_by(f64::total_cmp);
    candidates.dedup();
    let thresholds: Vec<_> = candidates
        .windows(2)
        .map(|pair| (pair[0] + pair[1]) / 2.0)
        .collect();
    let mut predictions = [0.5; 6];
    let mut trees = Vec::new();
    let rate = 0.5;
    let loss = |predictions: &[f64]| {
        predictions
            .iter()
            .zip(targets)
            .map(|(&p, y)| (p - y).powi(2))
            .sum::<f64>()
            / targets.len() as f64
    };
    let before = loss(&predictions);
    for iteration in 0..5 {
        let residuals: Vec<_> = targets
            .iter()
            .zip(predictions)
            .map(|(&y, p)| y - p)
            .collect();
        let mut best: Option<(f64, ObliviousTree)> = None;
        for &threshold in &thresholds {
            let mut sums = [0.0; 2];
            let mut counts = [0_usize; 2];
            for (&x, &residual) in features.iter().zip(&residuals) {
                let leaf = usize::from(x > threshold);
                sums[leaf] += residual;
                counts[leaf] += 1;
            }
            if counts.contains(&0) {
                continue;
            }
            let leaves = vec![sums[0] / counts[0] as f64, sums[1] / counts[1] as f64];
            let error = features
                .iter()
                .zip(&residuals)
                .map(|(&x, &r)| (leaves[usize::from(x > threshold)] - r).powi(2))
                .sum::<f64>();
            if best
                .as_ref()
                .is_none_or(|(best_error, _)| error < *best_error)
            {
                best = Some((
                    error,
                    ObliviousTree {
                        splits: vec![(0, threshold)],
                        leaves,
                    },
                ));
            }
        }
        let (_, tree) = best.unwrap();
        for (prediction, &feature) in predictions.iter_mut().zip(&features) {
            *prediction += rate
                * tree
                    .predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[feature])
                    .unwrap();
        }
        println!(
            "Шаг {iteration}: порог={}, листья={:?}, ошибка={}",
            tree.splits[0].1,
            tree.leaves,
            loss(&predictions)
        );
        trees.push(tree);
    }
    assert!(loss(&predictions) < before);
    // Новые объекты кодируем по всему TRAIN, без использования их правильных ответов.
    let mut test_error = 0.0;
    for (category, expected) in [("A", 1.0), ("B", 0.0)] {
        let mut sum = 0.5;
        let mut count = 1.0;
        for (&train_category, &target) in categories.iter().zip(&targets) {
            if train_category == category {
                sum += target;
                count += 1.0;
            }
        }
        let feature = sum / count;
        let prediction = 0.5
            + rate
                * trees
                    .iter()
                    .map(|tree| {
                        tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&[
                            feature,
                        ])
                        .unwrap()
                    })
                    .sum::<f64>();
        test_error += (prediction - expected).powi(2) / 2.0;
        println!(
            "Новый объект {category}: признак={feature}, прогноз={prediction}, ответ={expected}"
        );
    }
    assert!(test_error < 0.25); // Постоянный прогноз 0.5 имеет MSE=0.25.
    println!("Ошибка новых объектов={test_error}, baseline=0.25");
}

// Чему учит этот урок:
// Используем прежние ответы при кодировании обучающих категорий, затем подбираем пороги и листья по остаткам.
// Последовательно добавляем обученные поправки и проверяем прогноз на новых объектах.
// Кодирование новых объектов использует только статистики обучения, а не их правильные ответы.
