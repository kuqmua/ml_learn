// Урок 30.5. Исследование и использование.
//
// Случайное число ниже epsilon ведёт к исследованию, иначе выбираем лучшее известное действие.
// На границе random=epsilon выбираем использование.

fn main() {
    let exploration_probability = 0.1;
    let best_known_action = "вправо";
    for (description, random_fraction, expected_action) in [
        ("исследование", 0.05, "влево"),
        ("граница", 0.1, "вправо"),
        ("использование", 0.8, "вправо"),
    ] {
        assert!((0.0..1.0).contains(&random_fraction));
        let action = if random_fraction < exploration_probability {
            "влево"
        } else {
            best_known_action
        };
        assert_eq!(action, expected_action);
        println!("{description}: случайное число={random_fraction}, действие={action}");
    }
}
