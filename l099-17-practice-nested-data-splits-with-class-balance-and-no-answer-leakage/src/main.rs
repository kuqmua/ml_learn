// Урок 099. Выполнять вложенную проверку: внутри каждой внешней обучающей части заново выбирать
// настройку.
// Сохраняем доли классов, считаем подготовку только по текущему обучению и проверяем, что каждый
// объект стал внешним проверочным ровно один раз.

fn main() {
    fn groups(data: &[(f64, bool)], count: usize) -> Vec<Vec<usize>> {
        let mut result = vec![Vec::new(); count];
        let mut seen = [0; 2];
        for (index, &(_, class)) in data.iter().enumerate() {
            let class = usize::from(class);
            result[seen[class] % count].push(index);
            seen[class] += 1;
        }
        result
    }
    fn accuracy(training: &[(f64, bool)], validation: &[(f64, bool)], k: usize) -> f64 {
        assert!(k > 0 && k <= training.len());
        // Параметр подготовки считаем заново только по текущей обучающей части.
        let mean = training.iter().map(|v| v.0).sum::<f64>() / training.len() as f64;
        validation
            .iter()
            .filter(|&&(x, y)| {
                let query = x - mean;
                let mut neighbors: Vec<_> = training
                    .iter()
                    .map(|&(value, target)| ((value - mean - query).abs(), target))
                    .collect();
                neighbors.sort_by(|a, b| a.0.total_cmp(&b.0));
                (neighbors[..k].iter().filter(|v| v.1).count() * 2 > k) == y
            })
            .count() as f64
            / validation.len() as f64
    }
    let data = [
        (0.0, false),
        (1.0, false),
        (2.0, false),
        (3.0, false),
        (4.0, false),
        (5.0, false),
        (10.0, true),
        (11.0, true),
        (12.0, true),
        (13.0, true),
        (14.0, true),
        (15.0, true),
    ];
    let mut checked = vec![0; data.len()];
    let mut outer_scores = Vec::new();
    for (fold, outer_indices) in groups(&data, 3).iter().enumerate() {
        let outer_training: Vec<_> = data
            .iter()
            .enumerate()
            .filter(|(i, _)| !outer_indices.contains(i))
            .map(|(_, v)| *v)
            .collect();
        let outer_validation: Vec<_> = outer_indices
            .iter()
            .map(|&i| {
                checked[i] += 1;
                data[i]
            })
            .collect();
        assert_eq!(outer_validation.iter().filter(|v| v.1).count(), 2);
        let inner_groups = groups(&outer_training, 2);
        let mut best = (1, -1.0);
        for k in [1, 3] {
            let mut sum = 0.0;
            for inner_indices in &inner_groups {
                let training: Vec<_> = outer_training
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !inner_indices.contains(i))
                    .map(|(_, v)| *v)
                    .collect();
                let validation: Vec<_> = inner_indices.iter().map(|&i| outer_training[i]).collect();
                sum += accuracy(&training, &validation, k);
            }
            let score = sum / inner_groups.len() as f64;
            println!("Внешняя группа {fold}: внутренняя оценка k={k}: {score}");
            if score > best.1 {
                best = (k, score);
            }
        }
        let score = accuracy(&outer_training, &outer_validation, best.0);
        outer_scores.push(score);
        println!(
            "Группа {fold}: выбрали k={}, проверили на неиспользованных при выборе данных: {score}",
            best.0
        );
    }
    assert!(checked.iter().all(|&n| n == 1));
    assert!(outer_scores.iter().all(|&score| score == 1.0));
    println!(
        "Средняя внешняя точность={}",
        outer_scores.iter().sum::<f64>() / outer_scores.len() as f64
    );
}

// Чему учит этот урок:
// Учимся выполнять вложенную проверку: внутри каждой внешней обучающей части заново выбирать
// настройку.
// Сохраняем доли классов, считаем подготовку только по текущему обучению и проверяем, что каждый
// объект стал внешним проверочным ровно один раз.
