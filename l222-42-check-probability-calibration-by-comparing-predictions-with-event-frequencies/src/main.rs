// Урок 42.2. Проверка вероятностей: сравнение прогнозов с наблюдаемой частотой событий.
// Зачем здесь эта тема: Вероятность 0,8 полезна лишь если среди таких прогнозов событие происходит
//   примерно в 80% случаев.
// Почему код устроен так: Группируем прогнозы по интервалам и сравниваем среднюю вероятность с
//   частотой метки.
// Представь: Среди прогнозов около 0,8 событие должно встречаться примерно в 8 случаях из 10.
//
// Если модель сообщает 0.8 многим объектам, событие должно происходить примерно в 80% случаев.
// Сравниваем совпадение прогноза с наблюдаемой частотой и чрезмерную уверенность.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

fn main() {
    let observed_targets: [bool; 5] = [true, true, true, false, true];
    assert!(
        !observed_targets.is_empty(),
        "для частоты нужна хотя бы одна метка"
    );
    let observed_frequency: f64 = observed_targets.iter().filter(|&&target| target).count() as f64
        / observed_targets.len() as f64;
    for (_description, predicted_probability, expected_gap) in [
        ("калиброванный прогноз", 0.8, 0.0),
        ("слишком уверенный", 1.0, 0.2),
        ("недооценка", 0.6, 0.2),
    ] {
        assert!((0.0..=1.0).contains(&predicted_probability));

        assert!(check_f64_eq_1e_minus_10(
            (predicted_probability - observed_frequency).abs(),
            expected_gap
        ));
    }

    // Выполняем вычисления из примера.
    let _ = observed_frequency;
}
