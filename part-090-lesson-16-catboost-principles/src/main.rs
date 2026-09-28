// Урок 16.5. Миниатюрный бустинг с категориями.
// Соединяем упорядоченную статистику и симметричное дерево; это учебная схема, не полная реализация CatBoost.

use part_087_lesson_16_oblivious_tree::ObliviousTree;
use part_088_lesson_16_ordered_category_statistics::ordered_target_mean;

fn main() {
    let categories = ["A", "B", "A", "B", "A", "B"];
    let targets: [f64; 6] = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
    // Замену категорий числами, рассчитанными по целям, называют target encoding.
    let category_target_mean_values = ordered_target_mean(&categories, &targets, 0.5, 1.0).unwrap();
    // Порог фиксирован для прозрачности примера; настоящий алгоритм выбирает split по данным.
    let tree = ObliviousTree {
        splits: vec![(0, 0.5)],
        leaves: vec![-0.25, 0.25],
    };
    let base = 0.5;
    let learning_rate = 0.5;
    let mut before = 0.0;
    let mut after = 0.0;
    for (index, (&feature, &target)) in category_target_mean_values.iter().zip(&targets).enumerate()
    {
        let prediction = base + learning_rate * tree.predict(&[feature]).unwrap();
        before += (base - target).powi(2);
        after += (prediction - target).powi(2);
        println!("строка {index}: target={target}, код={feature:.3}, prediction={prediction:.3}");
    }
    println!("MSE до: {:.3}; после: {:.3}", before / 6.0, after / 6.0);
    visualize(before / 6.0, after / 6.0);
}

fn visualize(before: f64, after: f64) {
    let path = lesson_visualization::bars(
        env!("CARGO_MANIFEST_DIR"),
        "mse",
        "Ошибка учебной схемы",
        "MSE",
        &[("до", before), ("после", after)],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", path.display());
}
