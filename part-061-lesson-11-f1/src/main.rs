// Урок 11.4. Мера F1.
//
// Объединяем precision и recall из двух предыдущих уроков.
// Если обе равны нулю, формула даёт 0/0, поэтому возвращаем None.

fn main() {
    for (description, precision, recall, expected) in [
        ("обе метрики высоки", 1.0, 1.0, Some(1.0)),
        ("одна ниже", 1.0, 0.5, Some(2.0 / 3.0)),
        ("одна равна нулю", 0.0, 0.5, Some(0.0)),
        ("обе равны нулю", 0.0, 0.0, None),
    ] {
        let f1 = part_061_lesson_11_f1::f1(Some(precision), Some(recall));
        assert_eq!(f1, expected);
        println!("{description}: precision={precision}, recall={recall}, F1={f1:?}");
    }
}
