// Урок 41.2. Перемещение агента влево или вправо в границах среды.
// Зачем здесь эта тема: Действие имеет смысл только вместе с правилом перехода между состояниями.
// Почему код устроен так: Ограничиваем движение границами среды и показываем результат каждого
//   шага.
// Представь: Из крайней левой клетки действие «влево» не должно выводить агента за границу мира.
//
// В линейном мире агент может шагнуть вправо или влево. На левой границе
// движение влево оставляет его в нулевой клетке.

fn main() {
    let last_state: i32 = 4;
    for (_description, state, action, expected) in [
        ("шаг вправо", 2, 1, 3),
        ("шаг влево", 2, -1, 1),
        ("левая граница", 0, -1, 0),
        ("правая граница", 4, 1, 4),
    ] {
        assert!((0..=last_state).contains(&state));
        assert!(action == -1 || action == 1);

        assert_eq!((state + action).clamp(0, last_state), expected);
    }

    plot_next_position_after_moving_right();
}

// Строим график по результатам урока.
fn plot_next_position_after_moving_right() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Переход состояния",
        "текущее состояние",
        "следующее состояние",
        &[lesson_visualization::Series {
            name: "действие +1",

            points: &(0..=5)
                .map(|plot_step_index| (plot_step_index as f64, (plot_step_index + 1) as f64))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
