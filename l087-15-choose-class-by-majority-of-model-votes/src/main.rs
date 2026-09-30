// Урок 15.3. Выбор класса большинством голосов моделей.
// Связь с принятой терминологией: Голосование большинства по бинарным прогнозам моделей.
// Зачем здесь эта тема: Несколько моделей дают несколько ответов; для итогового класса нужна
//   агрегация.
// Почему код устроен так: Считаем голоса и отдельно фиксируем правило для ничьей.
// Представь: Если три модели ответили A, A, B, большинство выбирает A.
//
// Больше половины голосов true даёт true, меньше — false.
// При равенстве голосов правило этого примера выбирает false.

fn main() {
    let cases: [(&str, &[bool], bool); 4] = [
        ("большинство за true", &[true, true, false], true),
        ("большинство за false", &[true, false, false], false),
        ("ничья", &[true, false], false),
        ("один голос", &[true], true),
    ];
    for (_description, votes, expected) in cases {
        assert!(!votes.is_empty(), "для решения нужен хотя бы один голос");
        let positives: usize = votes.iter().filter(|&&vote| vote).count();
        let result: bool = positives * 2 > votes.len();
        assert_eq!(result, expected);
    }

    plot_number_of_votes_for_each_class();
}

// Строим график по результатам урока.
fn plot_number_of_votes_for_each_class() {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Большинство голосов",
        "количество голосов",
        &[("за", 3.0), ("против", 2.0)],
    )
    .expect("не удалось сохранить график");
}
