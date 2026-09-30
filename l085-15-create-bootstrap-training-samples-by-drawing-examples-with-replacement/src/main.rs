// Урок 15.1. Повторные обучающие выборки: набор примеров с возвращением и возможными повторами.
// Связь с принятой терминологией: Выборка с возвращением для обучения ансамбля.
// Зачем здесь эта тема: Одно дерево может сильно зависеть от обучающих строк; выборки с
//   возвращением дают разные наборы для ансамбля.
// Почему код устроен так: Берём случайные индексы с возвращением, поэтому строка может повториться
//   или не попасть в выборку.
// Представь: Из строк [A, B, C] выборка с возвращением может стать [A, A, C]; строка B в неё не
//   попадёт.
//
// Что изучаем: Bootstrap-выборка.
// Зачем это нужно: Случайная выборка с возвращением может содержать один объект несколько раз и пропускать
// другой.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let data: [&str; 4] = ["A", "B", "C", "D"];
    let sampled_indices: [usize; 4] = [0, 2, 2, 3];
    let sample: [&str; 4] = sampled_indices.map(|index| data[index]);

    plot_repeated_appearances_in_sample_drawn_with_replacement(sample);
}

// Строим график по результатам урока.
fn plot_repeated_appearances_in_sample_drawn_with_replacement(sample: [&str; 4]) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Повторы в bootstrap-выборке",
        "число появлений",
        &[
            (
                "A",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "A")
                    .count() as f64,
            ),
            (
                "B",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "B")
                    .count() as f64,
            ),
            (
                "C",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "C")
                    .count() as f64,
            ),
            (
                "D",
                sample
                    .iter()
                    .filter(|&&horizontal_value| horizontal_value == "D")
                    .count() as f64,
            ),
        ],
    )
    .expect("не удалось сохранить график");
}
