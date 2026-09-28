//! Вычисления и примеры урока part-081-lesson-15-majority-vote.

// Урок 15.3. Голосование большинства.
//
// Больше половины голосов true даёт true, меньше — false.
// При равенстве голосов правило этого примера выбирает false.

pub fn run() {
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
}
