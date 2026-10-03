// Урок 080. Измеряем, насколько перемешаны два класса в группе примеров.
// Если все примеры одного класса, результат равен 0: выбирать между классами не приходится.
// Если классы встречаются поровну, результат равен 1 — наибольшее значение для двух классов.
// В формуле используются доли классов и их логарифмы по основанию 2.
// Нулевую долю пропускаем, потому что логарифм нуля посчитать нельзя.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

fn main() {
    for (_description, pos_class_share) in [
        ("только отрицательный класс", 0.0),
        ("четверть положительных", 0.25),
        ("классы поровну", 0.5),
        ("только положительный класс", 1.0),
    ] {
        assert!((0.0..=1.0).contains(&pos_class_share));
        let neg_class_share: f64 = 1.0 - pos_class_share;
        let mut binary_class_entropy_in_bits: f64 = 0.0;
        for probability in [pos_class_share, neg_class_share] {
            if probability > 0.0 {
                let ratio: f64 = (probability - 1.0) / (probability + 1.0);
                let mut term: f64 = ratio;
                let mut logarithm: f64 = 0.0;
                for odd_divisor in (1..=99).step_by(2) {
                    logarithm += term / odd_divisor as f64;
                    term *= ratio * ratio;
                }
                binary_class_entropy_in_bits -=
                    probability * (2.0 * logarithm) / std::f64::consts::LN_2;
            }
        }
        assert!(
            binary_class_entropy_in_bits >= -1e-10 && binary_class_entropy_in_bits <= 1.0 + 1e-10
        );
        if pos_class_share == 0.0 || pos_class_share == 1.0 {
            assert!(check_f64_eq_1e_minus_10(binary_class_entropy_in_bits, 0.0));
        }
        if pos_class_share == 0.5 {
            assert!(check_f64_eq_1e_minus_10(binary_class_entropy_in_bits, 1.0));
        }
    }
}
