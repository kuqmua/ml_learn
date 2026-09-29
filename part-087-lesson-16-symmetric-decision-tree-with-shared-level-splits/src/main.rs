// Урок 16.2. Симметричное дерево решений с общим разбиением на каждом уровне.
// Зачем здесь эта тема: Обычное дерево может иметь разные условия в каждом узле; симметричное
//   применяет одно условие на всём уровне.
// Почему код устроен так: Проверяем общие разбиения и индексацию листьев на малой глубине.
// Представь: На одном уровне симметричного дерева все узлы задают один вопрос, например «x < 5?».
// Один порог на каждом уровне ведёт к 2^depth листьям.

fn main() {
    lesson_trace::enable();
    let tree: part_087_lesson_16_symmetric_decision_tree_with_shared_level_splits::ObliviousTree =
        part_087_lesson_16_symmetric_decision_tree_with_shared_level_splits::ObliviousTree {
            splits: vec![(0, 0.5), (1, 0.5)],
            leaves: vec![0.0, 1.0, 2.0, 3.0],
        };
    lesson_trace::trace_step!(tree);
    for input in [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]] {
        lesson_trace::trace_step!(input);
        println!(
            "{input:?} -> {}",
            tree.predict_with_oblivious_decision_tree(&input).unwrap()
        );
    }
}
