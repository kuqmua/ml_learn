// Урок 10.3. Вероятность класса.
//
// Что изучаем: Вероятность класса.
// Зачем это нужно: Вероятность класса описывает уверенность модели и позволяет менять решение без
// повторного обучения.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Инициализируем значение `probability_positive` начальным состоянием.
    let probability_positive = 0.7;
    // Комбинируем исходные величины и сохраняем результат в `probability_negative`.
    let probability_negative = 1.0 - probability_positive;
    // Проверяем обязательное условие до дальнейшего вычисления.
    assert!(probability_positive >= 0.0 && probability_positive <= 1.0);
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("P(y=1)={probability_positive}, P(y=0)={probability_negative}");
    // График величин и зависимостей, изученных в этом уроке.
    let chart_points_0: Vec<(f64, f64)> = (0..=100)
        .map(|i| {
            let p = i as f64 / 100.0;
            (p, p)
        })
        .collect();
    let chart_points_1: Vec<(f64, f64)> = (0..=100)
        .map(|i| {
            let p = i as f64 / 100.0;
            (p, 1.0 - p)
        })
        .collect();
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Вероятности двух классов",
        "P(y=1)",
        "вероятность",
        &[
            lesson_visualization::Series {
                name: "положительный",
                points: &chart_points_0,
            },
            lesson_visualization::Series {
                name: "отрицательный",
                points: &chart_points_1,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
