// Урок 12.3. Выбор числа ближайших примеров, голосующих за класс.
// Связь с принятой терминологией: Выбор числа соседей для классификации kNN.
// Зачем здесь эта тема: Один сосед чувствителен к шуму, а слишком большое k размывает локальную
//   структуру.
// Почему код устроен так: Сравниваем голосование нескольких ближайших точек при разных k.
// Представь: Один сосед может ошибаться из-за шума; голоса трёх соседей могут изменить решение.
//
// Что изучаем: Выбор числа соседей k.
// Зачем это нужно: Малое k делает решение чувствительным к одной точке, большое сглаживает локальные
// различия.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let neighbor_targets: [bool; 5] = [true, false, false, true, true];
    for neighbor_count in [1, 3, 5] {
        let _: bool = neighbor_targets[..neighbor_count]
            .iter()
            .filter(|&&target| target)
            .count()
            * 2
            > neighbor_count;
    }

    plot_positive_class_share_among_nearest_examples(neighbor_targets);
}

// Строим график по результатам урока.
fn plot_positive_class_share_among_nearest_examples(neighbor_targets: [bool; 5]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Соседи и доля положительных",
        "k",
        "доля",
        &[
            lesson_visualization::Series {
                name: "положительные среди k",

                points: &[1usize, 3, 5]
                    .iter()
                    .map(|&neighbor_count| {
                        (
                            neighbor_count as f64,
                            neighbor_targets[..neighbor_count]
                                .iter()
                                .filter(|&&element_value| element_value)
                                .count() as f64
                                / neighbor_count as f64,
                        )
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "граница решения",

                points: &[(1.0, 0.5), (5.0, 0.5)].to_vec(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
