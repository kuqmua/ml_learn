//! Вычисления и примеры урока part-085-lesson-16-stratification.

// Урок 16.2. Стратификация.
//
// Если разложить упорядоченные по классу данные подряд, в одной части может оказаться
// только положительный класс, в другой — только отрицательный. Стратификация смешивает классы.

pub fn run() {
    let positive = [1, 3, 5, 7];
    let negative = [0, 2, 4, 6];
    let bad_first = positive;
    let bad_second = negative;
    let first_fold = [positive[0], positive[1], negative[0], negative[1]];
    let second_fold = [positive[2], positive[3], negative[2], negative[3]];
    for (description, fold_a, fold_b, expected_positive) in [
        ("разбиение подряд", bad_first, bad_second, [4, 0]),
        ("стратификация", first_fold, second_fold, [2, 2]),
    ] {
        let counts = [
            fold_a.iter().filter(|&&value| value % 2 == 1).count(),
            fold_b.iter().filter(|&&value| value % 2 == 1).count(),
        ];
        assert_eq!(counts, expected_positive);
        println!("{description}: части {fold_a:?} и {fold_b:?}, положительных {counts:?}");
    }
}
