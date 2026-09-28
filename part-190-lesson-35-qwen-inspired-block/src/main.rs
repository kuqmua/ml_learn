// Урок 35.8. Учебный блок по мотивам Qwen3.
// Собираем pre-RMSNorm, QK-Norm, RoPE, GQA, residual и SwiGLU без реальных весов Qwen.

use part_177_lesson_34_causal_self_attention::softmax;
use part_183_lesson_35_rmsnorm::rms_norm;
use part_184_lesson_35_rope::rotate_pair;
use part_187_lesson_35_swiglu::swiglu;

fn block(states: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let gamma = [1.0, 1.0];
    let norm: Vec<[f64; 2]> = states
        .iter()
        .map(|x| {
            let y = rms_norm(x, &gamma, 1e-6).unwrap();
            [y[0], y[1]]
        })
        .collect();
    // Одна K/V-голова хранит общие ключи и значения для двух Q-голов.
    let keys: Vec<_> = norm
        .iter()
        .enumerate()
        .map(|(position, x)| {
            let key = rms_norm(x, &gamma, 1e-6).unwrap();
            rotate_pair([key[0], key[1]], position, 0.1)
        })
        .collect();
    let mut output = Vec::new();
    for index in 0..states.len() {
        let mut context = [0.0; 2];
        for head in 0..2 {
            // Разные проекции Q обращаются к одним и тем же сохранённым K/V.
            let raw_query = if head == 0 {
                norm[index]
            } else {
                [norm[index][1], -norm[index][0]]
            };
            let query = rms_norm(&raw_query, &gamma, 1e-6).unwrap();
            let query = rotate_pair([query[0], query[1]], index, 0.1);
            let logits: Vec<f64> = (0..=index)
                .map(|past| (query[0] * keys[past][0] + query[1] * keys[past][1]) / 2.0_f64.sqrt())
                .collect();
            let weights = softmax(&logits);
            for (past, &weight) in weights.iter().enumerate() {
                context[0] += 0.5 * weight * norm[past][0];
                context[1] += 0.5 * weight * norm[past][1];
            }
        }
        let residual = [states[index][0] + context[0], states[index][1] + context[1]];
        let ff_input = rms_norm(&residual, &gamma, 1e-6).unwrap();
        output.push([
            residual[0] + 0.1 * swiglu(ff_input[0], ff_input[1]),
            residual[1] + 0.1 * swiglu(ff_input[1], ff_input[0]),
        ]);
    }
    output
}
fn main() {
    let states = [[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
    let output = block(&states);
    assert_eq!(block(&states[..1])[0], output[0]);
    println!("выход учебного блока: {output:?}");
    // Реальный Qwen3 имеет многомерные проекции, обученные веса и масштабные данные.
}
