// Урок 155. Сравнивать заданные представления слов суммой произведений координат.
// Результат зависит и от направлений, и от длин векторов; смысловое сходство определяется
// качеством самих представлений.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    let word_vec1: [f64; 2] = [0.8, 0.2];
    let word_vec2: [f64; 2] = [0.7, 0.3];
    let _: f64 = multiply_matching_coords_then_add_results(&word_vec1, &word_vec2)
        .expect("представления должны иметь одинаковое число координат");

    // Выполняем вычисления из примера.
    let _ = (&word_vec1, &word_vec2);

    let same = multiply_matching_coords_then_add_results(&word_vec1, &word_vec1).unwrap();
    let opposite =
        multiply_matching_coords_then_add_results(&word_vec1, &word_vec1.map(|x| -x)).unwrap();
    let similar = multiply_matching_coords_then_add_results(&word_vec1, &word_vec2).unwrap();
    println!("С собой={same}, с похожим вектором={similar}, с противоположным={opposite}");
    assert!(similar > 0.0 && opposite < 0.0);
}

// Чему учит этот урок:
// Учимся сравнивать заданные представления слов суммой произведений координат.
// Результат зависит и от направлений, и от длин векторов; смысловое сходство определяется
// качеством самих представлений.
