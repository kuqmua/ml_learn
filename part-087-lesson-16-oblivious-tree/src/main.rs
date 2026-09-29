// Урок 16.2. Симметричное дерево.
// Один порог на каждом уровне ведёт к 2^depth листьям.

fn main() {
    let tree = part_087_lesson_16_oblivious_tree::ObliviousTree {
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
