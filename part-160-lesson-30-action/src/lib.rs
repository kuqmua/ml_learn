//! Вычисления и примеры урока part-160-lesson-30-action.

// Урок 30.2. Действие агента.
//
// В линейном мире агент может шагнуть вправо или влево. На левой границе
// движение влево оставляет его в нулевой клетке.

pub fn run() {
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
}
