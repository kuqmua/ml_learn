// Урок 08.5. Версия данных.
//
// Отпечаток зависит от содержимого, поэтому одинаковое имя файла не гарантирует одинаковые данные.
// Для повторного запуска с теми же байтами отпечаток должен совпасть.

fn main() {
    let cases = [
        ("исходные данные", "1,0\n2,1\n"),
        ("те же данные", "1,0\n2,1\n"),
        ("изменилась одна метка", "1,0\n2,0\n"),
    ];
    let mut fingerprints = [0; 3];
    for (index, (description, data)) in cases.into_iter().enumerate() {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(data, &mut hasher);
        fingerprints[index] = std::hash::Hasher::finish(&hasher);
        println!("{description}: отпечаток {}", fingerprints[index]);
    }
    assert_eq!(fingerprints[0], fingerprints[1]);
    assert_ne!(fingerprints[0], fingerprints[2]);
}
