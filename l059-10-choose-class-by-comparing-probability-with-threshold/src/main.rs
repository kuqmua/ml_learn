// Урок 10.4. Выбор класса сравнением вероятности с порогом.
// Связь с принятой терминологией: Преобразование прогнозной вероятности в класс по порогу.
// Зачем здесь эта тема: Вероятность ещё не является меткой класса; решение зависит от выбранного
//   порога.
// Почему код устроен так: Сравниваем одну вероятность с порогом, чтобы увидеть смену класса без
//   переобучения модели.
// Представь: Вероятность 0,6 станет положительным классом при пороге 0,5, но отрицательным при
//   пороге 0,7.
//
// Что изучаем: Порог классификации.
// Зачем это нужно: Порог превращает вероятность в метку класса. Более низкий порог обычно даёт больше
// положительных прогнозов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let probabilities: [f64; 3] = [0.2, 0.55, 0.8];
    for threshold in [0.5, 0.7] {
        let _: [bool; 3] = probabilities.map(|probability| probability >= threshold);
    }

    plot_number_of_positive_predictions_for_changing_threshold();
}

// Строим график по результатам урока.
fn plot_number_of_positive_predictions_for_changing_threshold() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Число положительных прогнозов",
        "порог",
        "количество",
        &[lesson_visualization::Series {
            name: "оценки 0.2, 0.55, 0.8",

            points: &(0..=100)
                .map(|plot_step_index| {
                    let threshold_value: f64 = plot_step_index as f64 / 100.0;
                    (
                        threshold_value,
                        [0.2, 0.55, 0.8]
                            .iter()
                            .filter(|&&probability| probability >= threshold_value)
                            .count() as f64,
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
