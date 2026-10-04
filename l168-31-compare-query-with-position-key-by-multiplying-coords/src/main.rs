// Урок 168. Вычислять оценку совпадения запроса с ключом через сумму произведений координат.
// Эта оценка понадобится для выбора доли информации от соответствующей позиции.

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    let query: [f64; 2] = [1.0, 0.5];
    let key: [f64; 2] = [0.8, 0.2];

    // Выполняем вычисления из примера.
    let _ = (
        query,
        key,
        multiply_matching_coords_then_add_results(&query, &key)
            .expect("запрос и ключ должны иметь одинаковое число координат"),
    );

    let matching = multiply_matching_coords_then_add_results(&query, &key).unwrap();
    let opposite = multiply_matching_coords_then_add_results(&query, &key.map(|x| -x)).unwrap();
    println!("Оценка ключа={matching}, противоположного={opposite}");
    assert!(matching > opposite);
}

// Чему учит этот урок:
// Учимся вычислять оценку совпадения запроса с ключом через сумму произведений координат.
// Эта оценка понадобится для выбора доли информации от соответствующей позиции.
