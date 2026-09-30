// Урок 13.2. Начальные вероятности классов: доли их примеров в обучающем наборе.
// Связь с принятой терминологией: Априорные вероятности классов по обучающим данным.
// Зачем здесь эта тема: Чтобы сравнивать классы по Байесу, нужна исходная частота каждого класса до
//   чтения признаков.
// Почему код устроен так: Считаем prior по обучающим меткам и отделяем его от вероятности
//   признаков.
// Представь: Если 9 из 10 учебных текстов относятся к классу A, он изначально вероятнее до чтения
//   нового текста.
//
// Что изучаем: Априорные вероятности классов.
// Зачем это нужно: До чтения признаков модель учитывает, как часто встречается каждый класс в обучающем
// наборе.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let targets: [&str; 4] = ["code", "code", "code", "ml"];
    assert!(
        !targets.is_empty(),
        "для частоты класса нужна хотя бы одна метка"
    );
    let code_count: usize = targets.iter().filter(|&&target| target == "code").count();
    let code_prior: f64 = code_count as f64 / targets.len() as f64;
    let machine_learning_prior: f64 = 1.0 - code_prior;

    plot_training_class_shares(code_prior, machine_learning_prior);
}

// Строим график по результатам урока.
fn plot_training_class_shares(code_prior: f64, machine_learning_prior: f64) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Априорные вероятности",
        "вероятность",
        &[("code", code_prior), ("ml", machine_learning_prior)],
    )
    .expect("не удалось сохранить график");
}
