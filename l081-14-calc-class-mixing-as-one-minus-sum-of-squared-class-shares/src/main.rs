// Урок 081. Ещё один способ измерить смешанность классов в группе.
// Из единицы вычитаем сумму квадратов долей классов.
// Для двух классов это то же самое, что 2*p*(1-p), где p — доля первого класса.
// Если все примеры одного класса, получаем 0. При равных долях двух классов получаем 0.5.
// При выборе разбиения данных меньшая смешанность обычно предпочтительнее.

fn main() {
    for pos_class_share in [0.0, 0.5, 1.0] {
        let neg_class_share: f64 = 1.0 - pos_class_share;
        let _binary_gini_mixing: f64 =
            1.0 - pos_class_share * pos_class_share - neg_class_share * neg_class_share;
    }

    plot_class_mixing_as_twice_pos_share_times_neg_share();
}

// Строим график по результатам урока.
fn plot_class_mixing_as_twice_pos_share_times_neg_share() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Нечистота Gini",
        "доля положительных",
        "Gini",
        &[lesson_visualization::Series {
            name: "Gini(p)",

            points: &(0..=100)
                .map(|plot_step_index| {
                    let probability: f64 = plot_step_index as f64 / 100.0;
                    (probability, 2.0 * probability * (1.0 - probability))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
