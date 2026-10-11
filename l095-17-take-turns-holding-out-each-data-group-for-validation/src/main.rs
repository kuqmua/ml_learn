// Урок 095. По очереди выделять одну группу данных для проверки, а остальные — для обучения.
// Так каждый объект может побывать проверочным, а оценка меньше зависит от одного разбиения.

fn main() {
    let rows = [0, 1, 2, 3, 4, 5];
    let mut held_out_counts = [0; 6];
    for fold_index in 0..3 {
        let training: Vec<_> = rows
            .iter()
            .copied()
            .filter(|row| row % 3 != fold_index)
            .collect();
        let validation: Vec<_> = rows
            .iter()
            .copied()
            .filter(|row| row % 3 == fold_index)
            .collect();
        println!("Группа {fold_index}: обучение={training:?}, проверка={validation:?}");
        for &row in &validation {
            assert!(!training.contains(&row));
            held_out_counts[row as usize] += 1;
        }
    }
    assert_eq!(held_out_counts, [1; 6]);
}

// Чему учит этот урок:
// Учимся по очереди выделять одну группу данных для проверки, а остальные — для обучения.
// Так каждый объект может побывать проверочным, а оценка меньше зависит от одного разбиения.
