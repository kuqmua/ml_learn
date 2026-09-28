// Урок 36.2. Вращательные позиционные признаки RoPE.
// Позиция вращает пары координат Q и K, сохраняя их длину.

use part_189_lesson_36_rope::rotate_pair;
fn dot(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}
fn main() {
    let q = [1.0, 0.0];
    let k = [1.0, 0.0];
    let same = dot(rotate_pair(q, 3, 0.2), rotate_pair(k, 3, 0.2));
    let distant = dot(rotate_pair(q, 3, 0.2), rotate_pair(k, 8, 0.2));
    assert!((same - 1.0).abs() < 1e-12);
    assert!(distant < same);
    println!("одинаковая позиция: {same:.3}; разные позиции: {distant:.3}");
    visualize();
}

fn visualize() {
    let q = [1.0, 0.0];
    let q = rotate_pair(q, 0, 0.2);
    let points: Vec<_> = (0..=20)
        .map(|p| {
            let k = rotate_pair([1.0, 0.0], p, 0.2);
            (p as f64, dot(q, k))
        })
        .collect();
    let path = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "rope-relative",
        "RoPE и расстояние между позициями",
        "сдвиг позиции",
        "скалярное произведение",
        &[lesson_visualization::Series {
            name: "score",
            points: &points,
        }],
    )
    .expect("график");
    println!("график: {}", path.display());
}
