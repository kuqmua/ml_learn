// Урок 14.3. Выбор порога, который лучше всего разделяет классы.
// Связь с принятой терминологией: Жадный выбор порога для разбиения дерева решений.
// Зачем здесь эта тема: Мера нечистоты сама не строит дерево; нужно найти порог, который лучше
//   разделяет метки.
// Почему код устроен так: Перебираем кандидаты и считаем взвешенную нечистоту дочерних групп.
// Представь: Порог «признак < 5» полезен, если слева и справа после него классы стали однороднее.
//
// Что изучаем: Жадный выбор разбиения.
// Зачем это нужно: На каждом шаге дерево выбирает порог с наименьшей взвешенной нечистотой, не перебирая
// всё дерево целиком.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let data: [(f64, bool); 4] = [(1.0, false), (2.0, false), (3.0, true), (4.0, true)];
    let mut best: (f64, f64) = (f64::INFINITY, 0.0);
    for threshold in [1.5, 2.5, 3.5] {
        let samples_below_threshold: Vec<&(f64, bool)> =
            data.iter().filter(|sample| sample.0 < threshold).collect();
        let samples_at_or_above_threshold: Vec<&(f64, bool)> =
            data.iter().filter(|sample| sample.0 >= threshold).collect();
        assert!(
            !samples_below_threshold.is_empty() && !samples_at_or_above_threshold.is_empty(),
            "порог должен оставлять примеры с обеих сторон"
        );
        let positive_share_below_threshold: f64 = samples_below_threshold
            .iter()
            .filter(|sample| sample.1)
            .count() as f64
            / samples_below_threshold.len() as f64;
        let positive_share_at_or_above_threshold: f64 = samples_at_or_above_threshold
            .iter()
            .filter(|sample| sample.1)
            .count() as f64
            / samples_at_or_above_threshold.len() as f64;
        let class_mixing_below_threshold: f64 =
            2.0 * positive_share_below_threshold * (1.0 - positive_share_below_threshold);
        let class_mixing_at_or_above_threshold: f64 = 2.0
            * positive_share_at_or_above_threshold
            * (1.0 - positive_share_at_or_above_threshold);
        let score: f64 = (samples_below_threshold.len() as f64 * class_mixing_below_threshold
            + samples_at_or_above_threshold.len() as f64 * class_mixing_at_or_above_threshold)
            / data.len() as f64;
        if score < best.0 {
            best = (score, threshold);
        }
    }
    let _ = (&(best.1), &(best.0));

    plot_weighted_class_mixing_for_different_thresholds();
}

// Строим график по результатам урока.
fn plot_weighted_class_mixing_for_different_thresholds() {
    let greedy_split_points: Vec<(f64, f64)> =
        [(1.5, 1.0 / 3.0), (2.5, 0.0), (3.5, 1.0 / 3.0)].to_vec();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Чистота разбиения",
        "порог",
        "взвешенный Gini",
        &[lesson_visualization::Series {
            name: "данные −−++",

            points: &greedy_split_points,
        }],
    )
    .expect("не удалось сохранить график");
}
