// Урок 16.2. Симметричное дерево решений с общим разбиением на каждом уровне.
// Один порог на каждом уровне ведёт к 2^depth листьям.

fn main() {
    let tree: part_087_lesson_16_symmetric_decision_tree_with_shared_level_splits::ObliviousTree =
        part_087_lesson_16_symmetric_decision_tree_with_shared_level_splits::ObliviousTree {
            splits: vec![(0, 0.5), (1, 0.5)],
            leaves: vec![0.0, 1.0, 2.0, 3.0],
        };
    for input in [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]] {
        println!(
            "{input:?} -> {}",
            tree.predict_with_oblivious_decision_tree(&input).unwrap()
        );
    }
}
