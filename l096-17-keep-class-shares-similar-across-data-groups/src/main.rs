// Урок 17.2. Сохранение близких долей классов в разных группах данных.
// Зачем здесь эта тема: При редком классе случайный блок может остаться без его примеров и исказить
//   метрику.
// Почему код устроен так: Распределяем метки по блокам так, чтобы доли классов были близки к
//   исходным.
// Представь: Если положительных случаев мало, распределяем их по блокам, чтобы один блок не
//   оказался пустым.
//
// Если разложить упорядоченные по классу данные подряд, в одной части может оказаться
// только положительный класс, в другой — только отрицательный. Стратификация смешивает классы.

fn main() {
    let pos: [i32; 4] = [1, 3, 5, 7];
    let bad_first: [i32; 4] = pos;
    let neg: [i32; 4] = [0, 2, 4, 6];
    let bad_second: [i32; 4] = neg;
    let first_fold: [i32; 4] = [pos[0], pos[1], neg[0], neg[1]];
    let second_fold: [i32; 4] = [pos[2], pos[3], neg[2], neg[3]];
    for (_description, first_fold, second_fold, expected_pos) in [
        ("разбиение подряд", bad_first, bad_second, [4, 0]),
        ("стратификация", first_fold, second_fold, [2, 2]),
    ] {
        let counts: [usize; 2] = [
            first_fold.iter().filter(|&&value| value % 2 == 1).count(),
            second_fold.iter().filter(|&&value| value % 2 == 1).count(),
        ];
        assert_eq!(counts, expected_pos);
    }

    plot_pos_example_count_in_each_validation_group();
}

// Строим график по результатам урока.
fn plot_pos_example_count_in_each_validation_group() {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Положительные в каждой части",
        "число",
        &[
            ("подряд: fold 1", 4.0),
            ("подряд: fold 2", 0.0),
            ("страты: fold 1", 2.0),
            ("страты: fold 2", 2.0),
        ],
    )
    .expect("не удалось сохранить график");
}
