// Урок 19.4. Сохранённая доля разброса: деление разброса вдоль выбранного направления на общий.
// Зачем здесь эта тема: Уменьшение размерности теряет часть разброса; нужно измерять, сколько
//   сохранилось.
// Почему код устроен так: Делим дисперсию выбранной оси на сумму дисперсий всех осей.
// Представь: Если первая ось объясняет 90% разброса, проекция на неё сохраняет большую часть
//   различий между точками.
//
// Доля первой оси равна её дисперсии, делённой на общую дисперсию.
// Она лежит от 0 до 1; при нулевой общей дисперсии долю определить нельзя.

fn main() {
    let cases: [(&str, [f64; 2], Option<f64>); 4] = [
        ("первая ось сохраняет почти всё", [9.0, 1.0], Some(0.9)),
        ("оси равноправны", [5.0, 5.0], Some(0.5)),
        ("первая ось ничего не сохраняет", [0.0, 4.0], Some(0.0)),
        ("изменчивости нет", [0.0, 0.0], None),
    ];
    for (_description, variances_along_principal_axes, expected) in cases {
        assert!(
            variances_along_principal_axes
                .iter()
                .all(|&value| value >= 0.0),
            "дисперсия не может быть отрицательной"
        );
        let total_variance: f64 =
            variances_along_principal_axes[0] + variances_along_principal_axes[1];
        let variance_share_explained_by_first_axis_where_0_means_none_and_1_means_all_spread_preserved: Option<f64> = if total_variance == 0.0 {
            None
        } else {
            Some(variances_along_principal_axes[0] / total_variance)
        };
        assert_eq!(variance_share_explained_by_first_axis_where_0_means_none_and_1_means_all_spread_preserved, expected);
    }

    plot_first_direction_share_for_changing_variance();
}

// Строим график по результатам урока.
fn plot_first_direction_share_for_changing_variance() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Объяснённая дисперсия первой оси",
        "λ₁",
        "доля",
        &[lesson_visualization::Series {
            name: "λ₂=1",

            points: &(0..=50)
                .map(|plot_step_index| {
                    let variance_along_first_principal_axis: f64 = plot_step_index as f64 / 10.0;
                    (
                        variance_along_first_principal_axis,
                        variance_along_first_principal_axis
                            / (variance_along_first_principal_axis + 1.0),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
