// Урок 15.3. Голосование большинства.
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
    for (description, votes, expected) in cases {
        assert!(!votes.is_empty(), "для решения нужен хотя бы один голос");
        let positives = votes.iter().filter(|&&vote| vote).count();
        let result = positives * 2 > votes.len();
        assert_eq!(result, expected);
        println!("{description}: {votes:?} → {result}");
    }
    // Сравнение величин из этого урока.
    let chart = lesson_visualization::bars(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Большинство голосов",
        "количество голосов",
        &[("за", 3.0), ("против", 2.0)],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
