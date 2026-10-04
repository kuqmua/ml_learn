// Урок 083. Измерять ошибки реально построенных деревьев разной глубины на обучении и проверке.
// Проверяем, что запоминание испорченной обучающей метки улучшает обучение, но ухудшает ответы на
// новых точках.

fn main() {
    #[derive(Debug)]
    enum Tree {
        Leaf(bool),
        Split(f64, Box<Tree>, Box<Tree>),
    }
    fn fit(data: &[(f64, bool)], depth: usize) -> Tree {
        let positives = data.iter().filter(|v| v.1).count();
        let majority = positives * 2 >= data.len();
        if depth == 0 || positives == 0 || positives == data.len() {
            return Tree::Leaf(majority);
        }
        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mixing = |rows: &[(f64, bool)]| {
            let p = rows.iter().filter(|v| v.1).count() as f64 / rows.len() as f64;
            rows.len() as f64 * 2.0 * p * (1.0 - p)
        };
        let cut = (1..sorted.len())
            .min_by(|&a, &b| {
                (mixing(&sorted[..a]) + mixing(&sorted[a..]))
                    .total_cmp(&(mixing(&sorted[..b]) + mixing(&sorted[b..])))
            })
            .unwrap();
        Tree::Split(
            (sorted[cut - 1].0 + sorted[cut].0) / 2.0,
            Box::new(fit(&sorted[..cut], depth - 1)),
            Box::new(fit(&sorted[cut..], depth - 1)),
        )
    }
    fn predict(tree: &Tree, x: f64) -> bool {
        match tree {
            Tree::Leaf(v) => *v,
            Tree::Split(t, left, right) => predict(if x < *t { left } else { right }, x),
        }
    }
    // Основное правило: x >= 2. Но у точки 3 намеренно испорчена обучающая метка.
    let training = [
        (0.0, false),
        (1.0, false),
        (2.0, true),
        (3.0, false),
        (4.0, true),
        (5.0, true),
    ];
    let validation = [
        (0.2, false),
        (1.2, false),
        (2.2, true),
        (2.8, true),
        (3.2, true),
        (4.2, true),
    ];
    let mut errors = Vec::new();
    for depth in [0, 1, 4] {
        let tree = fit(&training, depth);
        let error = |rows: &[(f64, bool)]| {
            rows.iter()
                .filter(|&&(x, y)| predict(&tree, x) != y)
                .count() as f64
                / rows.len() as f64
        };
        let train_error = error(&training);
        let validation_error = error(&validation);
        println!(
            "Глубина={depth}: ошибка обучения={train_error:.3}, проверки={validation_error:.3}"
        );
        errors.push((train_error, validation_error));
    }
    assert_eq!(errors[2].0, 0.0);
    assert!(errors[2].1 > errors[1].1);
    println!(
        "Глубокое дерево запомнило испорченную метку: обучение улучшилось, новые ответы ухудшились."
    );
}

// Чему учит этот урок:
// Учимся измерять ошибки реально построенных деревьев разной глубины на обучении и проверке.
// Проверяем, что запоминание испорченной обучающей метки улучшает обучение, но ухудшает ответы на
// новых точках.
