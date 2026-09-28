// Урок 30.2. Действие агента.
//
// В линейном мире агент может шагнуть вправо или влево. На левой границе
// движение влево оставляет его в нулевой клетке.

fn main() {
    let last_state = 4;
    for (description, state, action, expected) in [
        ("шаг вправо", 2, 1, 3),
        ("шаг влево", 2, -1, 1),
        ("левая граница", 0, -1, 0),
        ("правая граница", 4, 1, 4),
    ] {
        assert!((0..=last_state).contains(&state));
        assert!(action == -1 || action == 1);
        let next_state = (state + action).clamp(0, last_state);
        assert_eq!(next_state, expected);
        println!("{description}: {state} + {action} → {next_state}");
    }
    // Значения из этого урока на графике.
    let chart_points_0: Vec<(f64, f64)> = (0..=5).map(|i| (i as f64, (i + 1) as f64)).collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Переход состояния",
        "текущее состояние",
        "следующее состояние",
        &[lesson_visualization::Series {
            name: "действие +1",
            points: &chart_points_0,
        }],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
