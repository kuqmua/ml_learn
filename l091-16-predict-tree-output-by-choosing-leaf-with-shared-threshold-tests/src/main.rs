// Урок 091. Получать ответ дерева, используя общую пороговую проверку на каждом уровне.
// Комбинация результатов проверок выбирает конечное значение из списка листьев.

use l091_16_predict_tree_output_by_choosing_leaf_with_shared_threshold_tests::ObliviousTree;

fn main() {
    let tree: ObliviousTree = ObliviousTree {
        splits: vec![(0, 0.5), (1, 0.5)],
        leaves: vec![0.0, 1.0, 2.0, 3.0],
    };
    for input in [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]] {
        let _ = &(tree
            .predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&input)
            .unwrap());
    }

    let inputs = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    let outputs = inputs.map(|input| {
        tree.predict_tree_output_by_choosing_leaf_with_shared_threshold_tests(&input)
            .unwrap()
    });
    println!("Входы={inputs:?}; значения листьев={outputs:?}");
    let mut sorted = outputs;
    sorted.sort_by(f64::total_cmp);
    assert_eq!(sorted, [0.0, 1.0, 2.0, 3.0]);
}

// Чему учит этот урок:
// Учимся получать ответ дерева, используя общую пороговую проверку на каждом уровне.
// Комбинация результатов проверок выбирает конечное значение из списка листьев.
