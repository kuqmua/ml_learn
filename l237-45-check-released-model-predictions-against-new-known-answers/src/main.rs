// Урок 45.2. Проверка прогнозов выпущенной модели по новым известным ответам.
// Связь с принятой терминологией: Оценка качества модели после выпуска по новым истинным меткам.
// Зачем здесь эта тема: Сдвиг признаков не всегда означает падение качества; нужны новые истинные
//   метки.
// Почему код устроен так: Считаем прежнюю метрику на новых размеченных данных и сравниваем с
//   исходным уровнем.
// Представь: После выпуска появились новые правильные ответы; по ним можно посчитать свежую
//   accuracy.
//
// Что изучаем: Качество после релиза.
// Зачем это нужно: Когда появляются истинные метки, пересчитываем метрику на новых данных отдельно от
// учебной оценки.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let truth: [bool; 3] = [true, false, true];
    let predicted: [bool; 3] = [true, true, true];
    assert_eq!(
        truth.len(),
        predicted.len(),
        "число прогнозов должно совпадать с числом ответов"
    );
    assert!(
        !truth.is_empty(),
        "для accuracy нужна хотя бы одна пара значений"
    );
    let correct: usize = (0..truth.len())
        .filter(|&index| truth[index] == predicted[index])
        .count();
    let _ = &(correct as f64 / truth.len() as f64);

    plot_counts_of_correct_and_incorrect_predictions_after_release(truth, correct);
}

// Строим график по результатам урока.
fn plot_counts_of_correct_and_incorrect_predictions_after_release(
    truth: [bool; 3],
    correct: usize,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Качество после релиза",
        "число объектов",
        &[
            ("верно", correct as f64),
            ("ошибка", (truth.len() - correct) as f64),
        ],
    )
    .expect("не удалось сохранить график");
}
