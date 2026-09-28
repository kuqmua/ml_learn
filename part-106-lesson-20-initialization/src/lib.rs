//! Вычисления и примеры урока part-106-lesson-20-initialization.

// Урок 20.2. Инициализация весов.
//
// Нейроны с одинаковыми весами дают одинаковый ответ на один вход.
// Разные веса позволяют нейронам начать обучение с разных ответов.

pub fn run() {
    let input = [1.0, 2.0];
    let cases = [
        ("одинаковые веса", [0.2, -0.3], [0.2, -0.3], true),
        ("разные веса", [0.2, -0.3], [-0.1, 0.4], false),
    ];
    for (description, first_neuron, second_neuron, should_match) in cases {
        assert_eq!(first_neuron.len(), input.len());
        assert_eq!(second_neuron.len(), input.len());
        let first_output = first_neuron[0] * input[0] + first_neuron[1] * input[1];
        let second_output = second_neuron[0] * input[0] + second_neuron[1] * input[1];
        assert_eq!(first_output == second_output, should_match);
        println!("{description}: ответы {first_output} и {second_output}");
    }
}
