// Урок 10.1. Вероятность класса через сигмоиду: единица, делённая на сумму единицы и e в степени, противоположной оценке модели.
// Связь с принятой терминологией: Преобразование логита в вероятность класса сигмоидой.
// Зачем здесь эта тема: Линейная сумма может быть любым числом, а для двух классов нужна величина
//   от 0 до 1.
// Почему код устроен так: Применяем сигмоиду к логиту и сравниваем значения по обе стороны нуля.
// Представь: Логит 0 даёт вероятность 0,5; положительный логит повышает её, отрицательный понижает.
//
// Отрицательная оценка модели даёт вероятность ниже 0.5, нулевой — 0.5,
// положительный — выше 0.5. Значение всегда находится между 0 и 1.

fn main() {
    for (_description, raw_model_score, expected_side) in [
        ("отрицательный", -2.0, -1),
        ("нулевой", 0.0, 0),
        ("положительный", 2.0, 1),
    ] {
        let mut term: f64 = 1.0;
        let mut exponential: f64 = 1.0;
        for index in 1..=30 {
            term *= -raw_model_score / index as f64;
            exponential += term;
        }
        let probability: f64 = 1.0 / (1.0 + exponential);
        assert!(probability > 0.0 && probability < 1.0);
        let side: i32 = if probability < 0.5 {
            -1
        } else if probability > 0.5 {
            1
        } else {
            0
        };
        assert_eq!(side, expected_side);
    }

    plot_class_probability_as_one_over_one_plus_e_to_negative_score();
}

// Строим график по результатам урока.
fn plot_class_probability_as_one_over_one_plus_e_to_negative_score() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сигмоида",
        "логит",
        "вероятность",
        &[lesson_visualization::Series {
            name: "σ(x)",

            points: &(-60..=60)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (horizontal_value, 1.0 / (1.0 + (-horizontal_value).exp()))
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
