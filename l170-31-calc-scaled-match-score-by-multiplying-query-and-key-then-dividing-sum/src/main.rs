// Урок 31.4. Масштабированная оценка совпадения: умножение координат запроса и ключа, сложение и деление на корень из числа координат.
// Зачем здесь эта тема: При большом числе координат сумма произведений Q·K может вырасти, и выбор весов через экспоненты может стать
//   слишком резким.
// Почему код устроен так: Делим оценку на корень из размерности ключа до нормировки.
// Представь: При длинных Q и K сумма произведений может стать большой; деление удерживает оценки в
//   более удобном масштабе.
//
// Что изучаем: Масштабирование суммы произведений соответствующих координат векторов запроса и ключа.
// Зачем это нужно: Оценку внимания делим на корень из размерности ключа, чтобы крупные векторы не делали
// softmax слишком резким.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

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
    let _ = &(multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&query, &key)
        .expect("запрос и ключ должны иметь одинаковое число координат")
        / square_root_of_coord_count_to_limit_growth_of_match_scores);

    plot_attention_scale_as_one_divided_by_square_root_of_coord_count();
}

// Строим график по результатам урока.
fn plot_attention_scale_as_one_divided_by_square_root_of_coord_count() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Деление суммы произведений координат на корень из их количества",
        "размерность d",
        "множитель 1/√d",
        &[lesson_visualization::Series {
            name: "масштаб",

            points: &(1..=64)
                .map(|vec_dimension| (vec_dimension as f64, 1.0 / (vec_dimension as f64).sqrt()))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
