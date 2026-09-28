// Урок 11.4. Мера F1.
//
// F1 растёт, когда высоки и precision, и recall. При одном нуле и другом положительном
// значении она равна нулю. Для пары (0, 0) формула даёт 0/0, поэтому метрика не определена.

fn main() {
    for (description, precision, recall, expected) in [
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        assert!((0.0..=1.0).contains(&precision) && (0.0..=1.0).contains(&recall));
        let f1 = if precision + recall == 0.0 {
            None
        } else {
            Some(2.0 * precision * recall / (precision + recall))
        };
        assert_eq!(f1, expected);
        println!("{description}: precision={precision}, recall={recall}, F1={f1:?}");
    }
}
