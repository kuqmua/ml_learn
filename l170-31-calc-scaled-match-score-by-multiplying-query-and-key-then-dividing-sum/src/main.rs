// Урок 170. Делить оценку совпадения запроса и ключа на корень из числа координат.
// Так масштабируем оценки перед превращением их в веса внимания; это не нормализация по длинам
// векторов.

use lesson_float_comparison::check_f64_eq_1e_minus_10;

use l001_01_multiply_matching_coords_then_add_results::multiply_matching_coords_then_add_results;

fn main() {
    let dimension: f64 = 2.0;
    let mut square_root_of_coord_count_to_limit_growth_of_match_scores: f64 = dimension;
    for _ in 0..80 {
        square_root_of_coord_count_to_limit_growth_of_match_scores =
            (square_root_of_coord_count_to_limit_growth_of_match_scores
                + dimension / square_root_of_coord_count_to_limit_growth_of_match_scores)
                / 2.0;
    }
    let query: [f64; 2] = [1.0, 1.0];
    let key: [f64; 2] = [2.0, 2.0];
    let _ = &(multiply_matching_coords_then_add_results(&query, &key)
        .expect("запрос и ключ должны иметь одинаковое число координат")
        / square_root_of_coord_count_to_limit_growth_of_match_scores);

    let raw = multiply_matching_coords_then_add_results(&query, &key).unwrap();
    let scaled = raw / square_root_of_coord_count_to_limit_growth_of_match_scores;
    println!(
        "Сумма произведений={raw}; число координат={dimension}; делитель={square_root_of_coord_count_to_limit_growth_of_match_scores}; оценка={scaled}"
    );
    assert!(scaled < raw);
    assert!(check_f64_eq_1e_minus_10(scaled, 2.0_f64.sqrt() * 2.0));
}

// Чему учит этот урок:
// Учимся делить оценку совпадения запроса и ключа на корень из числа координат.
// Так масштабируем оценки перед превращением их в веса внимания; это не нормализация по длинам
// векторов.
