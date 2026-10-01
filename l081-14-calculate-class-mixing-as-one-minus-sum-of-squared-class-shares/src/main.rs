// Урок 14.2. Смешанность классов (мера Джини): единица минус сумма квадратов долей классов.
// Связь с принятой терминологией: Нечистота Джини по долям классов в узле дерева решений.
// Зачем здесь эта тема: Нечистота Джини даёт другой простой критерий выбора разбиения дерева.
// Почему код устроен так: Сравниваем суммы квадратов долей с энтропией на одинаковых составах узла.
// Представь: Чистый узел имеет Джини 0; для равных долей двух классов нечистота выше.
//
// Что изучаем: Нечистота Gini.
// Зачем это нужно: Gini равна единице минус сумма квадратов долей классов; чистому узлу соответствует
// ноль.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    for positive_class_share in [0.0, 0.5, 1.0] {
        let negative_class_share: f64 = 1.0 - positive_class_share;
        let _: f64 = 1.0
            - positive_class_share * positive_class_share
            - negative_class_share * negative_class_share;
    }

    plot_class_mixing_as_twice_positive_share_times_negative_share();
}

// Строим график по результатам урока.
fn plot_class_mixing_as_twice_positive_share_times_negative_share() {
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
