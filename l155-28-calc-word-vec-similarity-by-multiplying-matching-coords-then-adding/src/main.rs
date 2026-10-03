// Урок 155. Каждому слову уже сопоставлен список чисел. Сравниваем два таких списка:
// умножаем числа на одинаковых местах и складываем результаты.
// Большой положительный ответ может означать похожие направления, но зависит и от длин списков-стрелок.
// Для сравнения только направлений нужно дополнительно разделить ответ на обе длины.

use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

fn main() {
    let first_word_vec: [f64; 2] = [0.8, 0.2];
    let second_word_vec: [f64; 2] = [0.7, 0.3];
    let _: f64 =
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&first_word_vec, &second_word_vec)
            .expect("представления должны иметь одинаковое число координат");

    plot_coords_of_two_word_representations(first_word_vec, second_word_vec);
}

// Строим график по результатам урока.
fn plot_coords_of_two_word_representations(first_word_vec: [f64; 2], second_word_vec: [f64; 2]) {
    let first_dense_representation_points: Vec<(f64, f64)> =
        vec![(first_word_vec[0], first_word_vec[1])];
    let second_dense_representation_points: Vec<(f64, f64)> =
        vec![(second_word_vec[0], second_word_vec[1])];
    lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Похожие эмбеддинги",
        "первая координата",
        "вторая координата",
        &[
            lesson_visualization::Series {
                name: "первый",

                points: &first_dense_representation_points,
            },
            lesson_visualization::Series {
                name: "второй",

                points: &second_dense_representation_points,
            },
        ],
    )
    .expect("не удалось сохранить график");
}
