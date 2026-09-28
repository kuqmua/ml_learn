// Урок 26.5. Миниатюрная авторегрессионная модель звука.
// Сочетаем причинные дилатированные свёртки, gate и вероятность следующего дискретного отсчёта.

use part_138_lesson_26_causal_convolution::causal_conv;
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// Текущий вход содержит только уже известные отсчёты.
fn next_probability(history: &[u8]) -> f64 {
    let input: Vec<f64> = history
        .iter()
        .map(|&sample| f64::from(sample) * 2.0 - 1.0)
        .collect();
    let filter_one = causal_conv(&input, 0.8, 0.4, 1).unwrap();
    let gate_one = causal_conv(&input, 0.2, -0.3, 1).unwrap();
    let layer_one: Vec<f64> = filter_one
        .iter()
        .zip(&gate_one)
        .map(|(&f, &g)| f.tanh() * sigmoid(g))
        .collect();
    let filter_two = causal_conv(&layer_one, 1.0, 0.5, 2).unwrap();
    let gate_two = causal_conv(&layer_one, 0.1, 0.6, 2).unwrap();
    let last = history.len() - 1;
    let output = filter_two[last].tanh() * sigmoid(gate_two[last]);
    sigmoid(2.0 * output)
}
fn main() {
    let mut samples = vec![1, 0, 1, 1];
    for _ in 0..4 {
        let probability = next_probability(&samples);
        let next = u8::from(probability >= 0.5);
        samples.push(next);
        println!("P(следующий отсчёт=1)={probability:.3}; выбор={next}");
    }
    assert_eq!(samples.len(), 8);
    // Фиксированные веса здесь показывают только прямой проход; обучение остаётся отдельной задачей.
    println!("дискретный звук: {samples:?}");
    visualize(&samples);
}

fn visualize(samples: &[u8]) {
    let points: Vec<_> = samples
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as f64, f64::from(x)))
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "samples",
        "Дискретные отсчёты",
        "t",
        "значение",
        &[lesson_visualization::Series {
            name: "отсчёт",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
