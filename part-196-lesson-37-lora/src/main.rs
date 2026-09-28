// Урок 37.1. Низкоранговая адаптация LoRA.
// Замороженную матрицу дополняют произведением маленьких обучаемых матриц.

fn main() {
    let frozen = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    // Rank 1: A имеет форму 4x1, B — 1x4.
    let a = [0.1, 0.2, 0.3, 0.4];
    let b = [1.0, 0.0, -1.0, 0.0];
    let input = [1.0, 2.0, 3.0, 4.0];
    let bx: f64 = b.iter().zip(input).map(|(x, y)| x * y).sum();
    let output: [f64; 4] = std::array::from_fn(|row| {
        frozen[row]
            .iter()
            .zip(input)
            .map(|(w, x)| w * x)
            .sum::<f64>()
            + a[row] * bx
    });
    assert_eq!(output[0], 0.8);
    println!("выход с LoRA: {output:?}; trainable=8 вместо 16 параметров");
}
