// Урок 05.2. Проверка независимости событий: сравнение совместной вероятности с результатом умножения вероятностей.
// Зачем здесь эта тема: Формула произведения упрощается лишь при независимости; это допущение надо
//   проверять.
// Почему код устроен так: Сравниваем совместную и произведение отдельных вероятностей на понятных
//   событиях.
// Представь: Если вероятность дождя не меняет вероятность выбранного исхода, эти события можно
//   считать независимыми.
//
// События независимы, когда вероятность их совместного появления равна результату
// умножения отдельных вероятностей. Одинаковые отдельные вероятности этого не гарантируют.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

fn main() {
    let cases: [(&str, f64, f64, f64, bool); 3] = [
        ("две независимые монеты", 0.5, 0.5, 0.25, true),
        ("зависимые события", 0.5, 0.5, 0.5, false),
        ("несовместимые события", 0.5, 0.5, 0.0, false),
    ];
    for (
        _description,
        first_event_probability,
        second_event_probability,
        joint_probability,
        expected,
    ) in cases
    {
        assert!(
            (0.0..=1.0).contains(&first_event_probability)
                && (0.0..=1.0).contains(&second_event_probability)
        );
        assert!((0.0..=1.0).contains(&joint_probability));
        let independent: bool = check_f64_eq_1e_minus_10(
            joint_probability,
            first_event_probability * second_event_probability,
        );
        assert_eq!(independent, expected);
    }

    plot_joint_event_probabilities(cases);
}

// Строим график по результатам урока.
fn plot_joint_event_probabilities(cases: [(&str, f64, f64, f64, bool); 3]) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Зависимость событий",
        "P(A∩B)",
        &[
            ("независимые", cases[0].3),
            ("зависимые", cases[1].3),
            ("несовместимые", cases[2].3),
        ],
    )
    .expect("не удалось сохранить график");
}
