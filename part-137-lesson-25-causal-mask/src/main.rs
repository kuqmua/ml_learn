// Урок 25.6. Причинная маска внимания.
//
// На позиции 0 виден только первый токен, на позиции 1 — первые два,
// на последней позиции — все. Вес будущих позиций всегда равен нулю.

fn main() {
    let raw_weights = [0.2, 0.3, 0.5];
    for current_position in 0..raw_weights.len() {
        let mut masked = [0.0; 3];
        let allowed_sum: f64 = raw_weights[..=current_position].iter().sum();
        assert!(
            allowed_sum > 0.0,
            "доступные позиции должны иметь положительную сумму весов"
        );
        for index in 0..=current_position {
            masked[index] = raw_weights[index] / allowed_sum;
        }
        assert!((masked.iter().sum::<f64>() - 1.0).abs() < 1e-10);
        assert!(
            masked[current_position + 1..]
                .iter()
                .all(|&weight| weight == 0.0)
        );
        println!("позиция {current_position}: веса {masked:?}");
    }
}
