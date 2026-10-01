// Урок 41.5. Поиск выгодных действий: периодический случайный выбор вместо лучшего известного действия.
// Связь с принятой терминологией: Исследование случайного действия и выбор лучшего известного действия.
// Зачем здесь эта тема: Постоянный выбор известного лучшего действия может не обнаружить лучший
//   путь.
// Почему код устроен так: С вероятностью epsilon исследуем, иначе используем текущую лучшую оценку.
// Представь: Даже если «вправо» пока кажется лучшим, иногда пробуем «влево», чтобы получить новые
//   сведения.
//
// Случайное число ниже epsilon ведёт к исследованию, иначе выбираем лучшее известное действие.
// На границе random=epsilon выбираем использование.

fn main() {
    let exploration_probability: f64 = 0.1;
    let best_known_action: &str = "вправо";
    for (_description, random_number_between_zero_and_one, expected_action) in [
        ("исследование", 0.05, "влево"),
        ("граница", 0.1, "вправо"),
        ("использование", 0.8, "вправо"),
    ] {
        assert!((0.0..1.0).contains(&random_number_between_zero_and_one));
        let action: &str = if random_number_between_zero_and_one < exploration_probability {
            "влево"
        } else {
            best_known_action
        };
        assert_eq!(action, expected_action);
    }

    plot_random_action_choice_below_exploration_threshold();
}

// Строим график по результатам урока.
fn plot_random_action_choice_below_exploration_threshold() {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Исследование и использование",
        "случайное число",
        "выбор",
        &[lesson_visualization::Series {
            name: "порог ε=0.2: 1=исследование",

            points: &(0..=100)
                .map(|plot_step_index| {
                    let random_value: f64 = plot_step_index as f64 / 100.0;
                    (random_value, if random_value < 0.2 { 1.0 } else { 0.0 })
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
