// Урок 05.2. Независимость событий.
//
// События независимы, когда вероятность их совместного появления равна результату
// умножения отдельных вероятностей. Одинаковые отдельные вероятности этого не гарантируют.

fn main() {
    let cases: [(&str, f64, f64, f64, bool); 3] = [
        ("две независимые монеты", 0.5, 0.5, 0.25, true),
        ("зависимые события", 0.5, 0.5, 0.5, false),
        ("несовместимые события", 0.5, 0.5, 0.0, false),
    ];
    for (description, first, second, both, expected) in cases {
        assert!((0.0..=1.0).contains(&first) && (0.0..=1.0).contains(&second));
        assert!((0.0..=1.0).contains(&both));
        let independent = (both - first * second).abs() < 1e-10;
        assert_eq!(independent, expected);
        println!(
            "{description}: P(A)={first}, P(B)={second}, P(A и B)={both}, независимы={independent}"
        );
    }
}
