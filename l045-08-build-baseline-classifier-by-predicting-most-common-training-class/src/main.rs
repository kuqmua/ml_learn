// Урок 045. Строить простое правило, которое всегда предсказывает самый частый обучающий класс.
// Его доля верных ответов задаёт исходный ориентир для сравнения с более сложной моделью.

fn main() {
    let targets: [bool; 5] = [false, false, true, false, true];
    assert!(!targets.is_empty(), "для baseline нужна хотя бы одна метка");
    let pos_count: usize = targets.iter().filter(|&&target| target).count();
    let majority_class: bool = pos_count * 2 > targets.len();
    let _: f64 = targets
        .iter()
        .filter(|&&target| target == majority_class)
        .count() as f64
        / targets.len() as f64;

    // Выполняем вычисления из примера.
    let _ = (&targets, &pos_count);

    let accuracy = targets
        .iter()
        .filter(|&&target| target == majority_class)
        .count() as f64
        / targets.len() as f64;
    println!(
        "Всегда отвечаем {majority_class}; верных ответов {:.0}%",
        accuracy * 100.0
    );
    assert_eq!(accuracy, 0.6);
    println!(
        "Три ответа верны без изучения признаков: более сложную модель нужно сравнивать с таким ориентиром."
    );
}

// Чему учит этот урок:
// Учимся строить простое правило, которое всегда предсказывает самый частый обучающий класс.
// Его доля верных ответов задаёт исходный ориентир для сравнения с более сложной моделью.
