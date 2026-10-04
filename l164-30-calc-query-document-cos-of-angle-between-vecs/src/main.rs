// Урок 164. Сравнивать запрос и документ по направлениям их заданных векторов.
// Нормализация по длинам позволяет одинаково оценивать совпадающие направления при разном
// масштабе.

use l006_01_multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens;

fn main() {
    let query: [f64; 2] = [1.0, 0.0];
    let document: [f64; 2] = [2.0, 0.0];
    let _: f64 =
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(&query, &document)
            .expect(
                "для вычисления cos нужны ненулевые векторы слов с одинаковым числом координат",
            );

    // Выполняем вычисления из примера.
    let _ = query;

    let same =
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(&query, &document)
            .unwrap();
    let unrelated = multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
        &query,
        &[0.0, 2.0],
    )
    .unwrap();
    println!("Совпадающее направление, другая длина={same}; перпендикулярное={unrelated}");
    assert_eq!(same, 1.0);
    assert_eq!(unrelated, 0.0);
}

// Чему учит этот урок:
// Учимся сравнивать запрос и документ по направлениям их заданных векторов.
// Нормализация по длинам позволяет одинаково оценивать совпадающие направления при разном
// масштабе.
