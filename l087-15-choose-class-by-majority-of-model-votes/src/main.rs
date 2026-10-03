// Урок 15.3. Выбор класса большинством голосов моделей.
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

        let predicted_majority_class: bool =
            votes.iter().filter(|&&vote| vote).count() * 2 > votes.len();
        assert_eq!(predicted_majority_class, expected);
    }
}
