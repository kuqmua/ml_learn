// Урок 08.2. Простая модель для сравнения: прогноз самого частого обучающего класса.
// Связь с принятой терминологией: Базовый классификатор по наиболее частому классу.
// Зачем здесь эта тема: Сложной модели нужна точка отсчёта; большинство классов даёт простой
//   baseline.
// Почему код устроен так: Считаем частоты только на train и сравниваем более сложные модели с этим
//   прогнозом.
// Представь: Если 80% примеров относятся к классу A, постоянный ответ A уже даёт 80% accuracy;
//   модель должна превзойти его.
//
// Что изучаем: Базовая модель.
// Зачем это нужно: Baseline задаёт простую исходную точку. Классификатор большинства всегда предсказывает
// самый частый класс.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let targets: [bool; 5] = [false, false, true, false, true];
    assert!(!targets.is_empty(), "для baseline нужна хотя бы одна метка");
    let positive_count: usize = targets.iter().filter(|&&target| target).count();
    let majority_class: bool = positive_count * 2 > targets.len();
    let _accuracy: f64 = targets
        .iter()
        .filter(|&&target| target == majority_class)
        .count() as f64
        / targets.len() as f64;

    plot_counts_of_positive_and_negative_training_targets(targets, positive_count);
}

// Строим график по результатам урока.
fn plot_counts_of_positive_and_negative_training_targets(
    targets: [bool; 5],
    positive_count: usize,
) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Классы для baseline",
        "объектов",
        &[
            ("отрицательные", (targets.len() - positive_count) as f64),
            ("положительные", positive_count as f64),
        ],
    )
    .expect("не удалось сохранить график");
}
