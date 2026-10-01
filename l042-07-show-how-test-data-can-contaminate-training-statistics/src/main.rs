// Урок 07.6. Проверка влияния тестовых данных на статистики, используемые при обучении.
// Связь с принятой терминологией: Утечка информации из тестовых данных в обучение.
// Зачем здесь эта тема: Даже без передачи меток в обучение статистика test может сделать оценку
//   качества слишком оптимистичной.
// Почему код устроен так: Сравниваем честную подготовку с подготовкой, куда попали тестовые строки.
// Представь: Если в test есть выброс 1000, среднее train не должно измениться из-за этого числа.
//
// Что изучаем: Утечка данных.
// Зачем это нужно: Статистику подготовки признаков нельзя считать по test: иначе тестовые значения влияют
// на обучение.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let training_data: [f64; 3] = [1.0, 2.0, 3.0];
    assert!(
        !training_data.is_empty(),
        "обучающая выборка не должна быть пустой"
    );
    let training_mean: f64 = training_data.iter().sum::<f64>() / training_data.len() as f64;
    let test: [f64; 1] = [100.0];
    let contaminated_mean: f64 = (training_data.iter().sum::<f64>() + test.iter().sum::<f64>())
        / (training_data.len() + test.len()) as f64;

    plot_means_computed_with_and_without_test_data(training_mean, contaminated_mean);
}

// Строим график по результатам урока.
fn plot_means_computed_with_and_without_test_data(training_mean: f64, contaminated_mean: f64) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Утечка меняет статистику",
        "среднее",
        &[("train", training_mean), ("с утечкой", contaminated_mean)],
    )
    .expect("не удалось сохранить график");
}
