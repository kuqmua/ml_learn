// Урок 21.4. Различие нейронов при обучении: назначение разных начальных весов.
// Зачем здесь эта тема: Одинаковые стартовые веса нейронов сохраняют симметрию и мешают учить
//   разные признаки.
// Почему код устроен так: Задаём разные начальные веса и сравниваем их дальнейшие выходы.
// Представь: Если два нейрона начали с одинаковых весов и получают одинаковые градиенты, они
//   останутся одинаковыми.
//
// Нейроны с одинаковыми весами дают одинаковый ответ на один вход.
// Разные веса позволяют нейронам начать обучение с разных ответов.

fn main() {
    let input: [f64; 2] = [1.0, 2.0];
    let cases: [(&str, [f64; 2], [f64; 2], bool); 2] = [
        ("одинаковые веса", [0.2, -0.3], [0.2, -0.3], true),
        ("разные веса", [0.2, -0.3], [-0.1, 0.4], false),
    ];
    for (_description, first_neuron, second_neuron, should_match) in cases {
        let first_output: f64 = first_neuron[0] * input[0] + first_neuron[1] * input[1];
        let second_output: f64 = second_neuron[0] * input[0] + second_neuron[1] * input[1];
        assert_eq!(first_output == second_output, should_match);
    }

    plot_different_starting_weights();
}

// Строим график по результатам урока.
fn plot_different_starting_weights() {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Разные начальные веса",
        "вес",
        &[("нейрон 1", 0.5), ("нейрон 2", -0.5)],
    )
    .expect("не удалось сохранить график");
}
