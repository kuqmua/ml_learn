// Урок 38.2. Симметричное дерево.
// Один порог на каждом уровне ведёт к 2^depth листьям.

use part_193_lesson_38_oblivious_tree::ObliviousTree;
fn main() {
    let tree = ObliviousTree {
        splits: vec![(0, 0.5), (1, 0.5)],
        leaves: vec![0.0, 1.0, 2.0, 3.0],
    };
    for input in [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]] {
        println!("{input:?} -> {}", tree.predict(&input).unwrap());
    }
}
