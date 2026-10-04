// Урок 058. Последовательно превращать вход в оценку прямой, затем в вероятность через сигмоиду.
// Проверяем взаимодополняемость вероятностей двух классов и значение 0.5 при нулевой оценке.

fn main() {
    let (weight, bias) = (2.0, 1.0);
    for input in [-2.0_f64, -0.5, 1.0] {
        let score = weight * input + bias;
        let pos_probability = 1.0 / (1.0 + (-score).exp());
        let neg_probability = 1.0 - pos_probability;
        println!(
            "Вход={input} -> оценка={score} -> P(1)={pos_probability:.4}, P(0)={neg_probability:.4}"
        );
        assert!((pos_probability + neg_probability - 1.0).abs() < 1e-12);
        if score == 0.0 {
            assert_eq!(pos_probability, 0.5);
        }
    }
}

// Чему учит этот урок:
// Учимся последовательно превращать вход в оценку прямой, затем в вероятность через сигмоиду.
// Проверяем взаимодополняемость вероятностей двух классов и значение 0.5 при нулевой оценке.
