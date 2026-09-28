// Урок 18.4. Объяснённая дисперсия.
//
// Доля первой оси равна её дисперсии, делённой на общую дисперсию.
// Она лежит от 0 до 1; при нулевой общей дисперсии долю определить нельзя.

fn main() {
    let cases: [(&str, [f64; 2], Option<f64>); 4] = [
        ("первая ось сохраняет почти всё", [9.0, 1.0], Some(0.9)),
        ("оси равноправны", [5.0, 5.0], Some(0.5)),
        ("первая ось ничего не сохраняет", [0.0, 4.0], Some(0.0)),
        ("изменчивости нет", [0.0, 0.0], None),
    ];
    for (description, eigenvalues, expected) in cases {
        assert!(
            eigenvalues.iter().all(|&value| value >= 0.0),
            "дисперсия не может быть отрицательной"
        );
        let total_variance = eigenvalues[0] + eigenvalues[1];
        let explained_fraction = if total_variance == 0.0 {
            None
        } else {
            Some(eigenvalues[0] / total_variance)
        };
        assert_eq!(explained_fraction, expected);
        println!("{description}: {eigenvalues:?} → доля первой оси {explained_fraction:?}");
    }
}
