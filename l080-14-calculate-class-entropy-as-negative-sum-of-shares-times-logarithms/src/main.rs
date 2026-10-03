// Урок 14.1. Неопределённость класса (энтропия): отрицательная сумма долей, умноженных на их логарифмы.
// Связь с принятой терминологией: Энтропия долей классов в узле дерева решений.
// Зачем здесь эта тема: Дерево выбирает разбиение по смешанности классов в узле; энтропия даёт одну
//   меру смешанности.
// Почему код устроен так: Считаем вклад каждой доли класса и сравниваем чистый и смешанный узлы.
// Представь: Узел только с классом A чистый; смесь A и B даёт большую энтропию.
//
// У чистого узла энтропия равна нулю, при долях 50/50 она максимальна.
// Нулевую долю пропускаем: предел p·log(p) при p→0 равен нулю.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

fn main() {
    for (_description, positive_class_share) in [
        ("только отрицательный класс", 0.0),
        ("четверть положительных", 0.25),
        ("классы поровну", 0.5),
        ("только положительный класс", 1.0),
    ] {
        assert!((0.0..=1.0).contains(&positive_class_share));
        let negative_class_share: f64 = 1.0 - positive_class_share;
        let mut binary_class_entropy_in_bits_where_0_means_one_class_and_1_means_equal_class_shares: f64 = 0.0;
        for probability in [positive_class_share, negative_class_share] {
            if probability > 0.0 {
                let ratio: f64 = (probability - 1.0) / (probability + 1.0);
                let mut term: f64 = ratio;
                let mut logarithm: f64 = 0.0;
                for odd_divisor in (1..=99).step_by(2) {
                    logarithm += term / odd_divisor as f64;
                    term *= ratio * ratio;
                }
                binary_class_entropy_in_bits_where_0_means_one_class_and_1_means_equal_class_shares -= probability * (2.0 * logarithm) / std::f64::consts::LN_2;
            }
        }
        assert!(binary_class_entropy_in_bits_where_0_means_one_class_and_1_means_equal_class_shares >= -1e-10 && binary_class_entropy_in_bits_where_0_means_one_class_and_1_means_equal_class_shares <= 1.0 + 1e-10);
        if positive_class_share == 0.0 || positive_class_share == 1.0 {
            assert!(check_f64_eq_1e_minus_10(
                binary_class_entropy_in_bits_where_0_means_one_class_and_1_means_equal_class_shares,
                0.0
            ));
        }
        if positive_class_share == 0.5 {
            assert!(check_f64_eq_1e_minus_10(
                binary_class_entropy_in_bits_where_0_means_one_class_and_1_means_equal_class_shares,
                1.0
            ));
        }
    }

    plot_class_entropy_as_negative_sum_of_class_shares_times_their_logarithms();
}

// Строим график по результатам урока.
fn plot_class_entropy_as_negative_sum_of_class_shares_times_their_logarithms() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Энтропия бинарного класса",
        "доля положительных",
        "энтропия",
        &[lesson_visualization::Series {
            name: "H(p)",

            points: &(1..100)
                .map(|plot_step_index| {
                    let probability: f64 = plot_step_index as f64 / 100.0;
                    (
                        probability,
                        -probability * probability.log2()
                            - (1.0 - probability) * (1.0 - probability).log2(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
