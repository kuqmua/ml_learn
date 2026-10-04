// Урок 163. Усиливать вес слова, если оно часто встречается в документе и редко среди остальных
// документов.
// Так частота и редкость вместе дают оценку важности слова для поиска.

use lesson_float_comparison::check_f64_eq_1e_minus_12;

fn main() {
    let document_count = 10.0_f64;
    let weight = |frequency: f64, documents_with_word: f64| {
        frequency * ((document_count + 1.0) / (documents_with_word + 1.0)).ln()
    };
    let common = weight(3.0, 10.0);
    let rare = weight(3.0, 2.0);
    let twice = weight(6.0, 2.0);
    println!(
        "Одинаковая частота 3: слово во всех документах -> {common:.4}, слово только в двух -> {rare:.4}"
    );
    println!("Редкое слово встретилось 6 раз: {twice:.4}");
    assert!(rare > common);
    assert!(check_f64_eq_1e_minus_12(twice, 2.0 * rare));
}

// Чему учит этот урок:
// Учимся усиливать вес слова, если оно часто встречается в документе и редко среди остальных
// документов.
// Так частота и редкость вместе дают оценку важности слова для поиска.
