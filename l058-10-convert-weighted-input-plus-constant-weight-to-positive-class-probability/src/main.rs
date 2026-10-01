// Урок 10.3. Преобразование взвешенного входа со смещением в вероятность положительного класса.
// Связь с принятой терминологией: Вероятность положительного класса из логистической модели.
// Зачем здесь эта тема: Логистическая модель связывает признаки с вероятностью через линейный логит
//   и сигмоиду.
// Почему код устроен так: Показываем оба шага отдельно, чтобы вес признака не смешивался с порогом
//   решения.
// Представь: Вес превращает признак в логит, сигмоида — логит в вероятность; это два разных шага.
//
// Что изучаем: Вероятность класса.
// Зачем это нужно: Вероятность класса описывает уверенность модели и позволяет менять решение без
// повторного обучения.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let probability_positive: f64 = 0.7;
    let _: f64 = 1.0 - probability_positive;
    assert!(probability_positive >= 0.0 && probability_positive <= 1.0);

    plot_positive_and_negative_class_probabilities();
}

// Строим график по результатам урока.
fn plot_positive_and_negative_class_probabilities() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Вероятности двух классов",
        "P(y=1)",
        "вероятность",
        &[
            lesson_visualization::Series {
                name: "положительный",

                points: &(0..=100)
                    .map(|plot_step_index| {
                        let probability: f64 = plot_step_index as f64 / 100.0;
                        (probability, probability)
                    })
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "отрицательный",

                points: &(0..=100)
                    .map(|plot_step_index| {
                        let probability: f64 = plot_step_index as f64 / 100.0;
                        (probability, 1.0 - probability)
                    })
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
