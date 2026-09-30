// Урок 16.2. Симметричное дерево решений: общая пороговая проверка для всех узлов одного уровня.
// Связь с принятой терминологией: Симметричное дерево решений с общим разбиением на каждом уровне.
// Зачем здесь эта тема: Обычное дерево может иметь разные условия в каждом узле; симметричное
//   применяет одно условие на всём уровне.
// Почему код устроен так: Проверяем общие разбиения и индексацию листьев на малой глубине.
// Представь: На одном уровне симметричного дерева все узлы задают один вопрос, например «x < 5?».
// Один порог на каждом уровне ведёт к 2^depth листьям.

use l091_16_build_symmetric_tree_using_shared_threshold_test_at_each_level::ObliviousTree;

use lesson_trace::{enable_tracing, trace_step};

fn main() {
    enable_tracing();
    let tree: ObliviousTree = ObliviousTree {
        splits: vec![(0, 0.5), (1, 0.5)],
        leaves: vec![0.0, 1.0, 2.0, 3.0],
    };
    trace_step!(tree);
    for input in [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]] {
        trace_step!(input);
        println!(
            "{input:?} -> {}",
            tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&input)
                .unwrap()
        );
    }
}
