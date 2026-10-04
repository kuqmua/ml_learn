// Урок 223. Считать качество отдельно по подгруппам данных.
// Так можно заметить, что общая оценка скрывает разное число ошибок в разных группах.

fn main() {
    let groups: [(&str, bool, bool); 4] = [
        ("A", true, true),
        ("A", false, false),
        ("B", true, false),
        ("B", false, false),
    ];
    for group_name in ["A", "B"] {
        let matching: Vec<&(&str, bool, bool)> = groups
            .iter()
            .filter(|&&(name, _, _)| name == group_name)
            .collect();
        assert!(
            !matching.is_empty(),
            "для оценки группы нужен хотя бы один пример"
        );

        println!(
            "Точность подгруппы: {:?}",
            &(matching
                .iter()
                .filter(|sample| sample.1 == sample.2)
                .count() as f64
                / matching.len() as f64)
        );
    }

    // Выполняем вычисления из примера.
    println!("Точность подгруппы: {:?}", groups);
}

// Чему учит этот урок:
// Учимся считать качество отдельно по подгруппам данных.
// Так можно заметить, что общая оценка скрывает разное число ошибок в разных группах.
