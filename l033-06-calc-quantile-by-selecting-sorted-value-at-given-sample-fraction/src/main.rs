// Урок 06.4. Квантиль: значение на заданной доле упорядоченной выборки.
// Зачем здесь эта тема: Один центр и один разброс не показывают хвосты; квантиль задаёт порог для
//   выбранной доли наблюдений.
// Почему код устроен так: Работаем с упорядоченными числами и явно выбираем индекс, чтобы правило
//   квантиля было проверяемым.
// Представь: Медиана — частный случай квантиля: половина упорядоченных значений находится не выше
//   неё.
//
// Используем ближайший порядковый элемент с индексом floor((n−1)·доля).
// Доля 0 даёт минимум, 1 — максимум; между ними выбирается элемент внутри ряда.

fn main() {
    let mut values: [i32; 5] = [9, 1, 7, 3, 5];
    assert!(!values.is_empty(), "для квантиля нужна непустая выборка");
    values.sort();
    for (_description, quantile_fraction, expected) in [
        ("минимум", 0.0, 1),
        ("середина", 0.5, 5),
        ("три четверти", 0.75, 7),
        ("максимум", 1.0, 9),
    ] {
        assert!(
            (0.0..=1.0).contains(&quantile_fraction),
            "доля должна быть от 0 до 1"
        );
        let index: usize = ((values.len() - 1) as f64 * quantile_fraction) as usize;
        let quantile_value_at_floored_fraction_of_last_sorted_index: i32 = values[index];
        assert_eq!(
            quantile_value_at_floored_fraction_of_last_sorted_index,
            expected
        );
    }

    plot_quantiles_as_sorted_values_at_each_fraction_of_sample();
}

// Строим график по результатам урока.
fn plot_quantiles_as_sorted_values_at_each_fraction_of_sample() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Квантили выборки",
        "доля",
        "квантиль",
        &[lesson_visualization::Series {
            name: "значения 1, 2, 3, 4, 5",

            points: &(0..=100)
                .map(|plot_step_index| {
                    let probability: f64 = plot_step_index as f64 / 100.0;
                    (
                        probability,
                        [1.0, 2.0, 3.0, 4.0, 5.0][((probability * 4.0) as usize).min(4)],
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
