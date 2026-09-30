// Урок 29.4. Случайный выбор следующей части текста согласно вероятностям.
// Связь с принятой терминологией: Выбор следующего токена из распределения вероятностей.
// Зачем здесь эта тема: Вероятности ещё не создают текст; нужен способ выбрать конкретный следующий
//   токен.
// Почему код устроен так: Используем фиксированное число из [0, 1), чтобы вручную проследить
//   накопленные вероятности и выбор токена.
// Представь: При вероятностях [0,5; 0,3; 0,2] число 0,65 попадёт во второй накопленный интервал.
//
// Что изучаем: Выборка следующего токена.
// Зачем это нужно: Сэмплирование использует распределение вероятностей, а не всегда берёт самый вероятный
// токен.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let words: [&str; 3] = ["кот", "пёс", "мир"];
    let probabilities: [f64; 3] = [0.5, 0.3, 0.2];
    let random_number_between_zero_and_one: f64 = 0.65;
    assert_eq!(
        words.len(),
        probabilities.len(),
        "каждому слову нужна вероятность"
    );
    assert!(!words.is_empty(), "для выбора нужно хотя бы одно слово");
    assert!(
        probabilities
            .iter()
            .all(|&value| value >= 0.0 && value.is_finite()),
        "вероятности должны быть конечными и неотрицательными"
    );
    assert!(
        (probabilities.iter().sum::<f64>() - 1.0).abs() < 1e-9,
        "сумма вероятностей должна быть равна 1"
    );
    assert!(
        (0.0..1.0).contains(&random_number_between_zero_and_one),
        "случайное число должно быть от 0 до 1, не включая 1"
    );
    let mut cumulative: f64 = 0.0;
    for index in 0..words.len() {
        cumulative += probabilities[index];
        if random_number_between_zero_and_one < cumulative {
            let _ = &(words[index]);
            break;
        }
    }

    plot_probabilities_of_possible_next_text_units(probabilities);
}

// Строим график по результатам урока.
fn plot_probabilities_of_possible_next_text_units(probabilities: [f64; 3]) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Распределение следующего токена",
        "вероятность",
        &[
            ("токен 1", probabilities[0]),
            ("токен 2", probabilities[1]),
            ("токен 3", probabilities[2]),
        ],
    )
    .expect("не удалось сохранить график");
}
