// Урок 31.2. Сравнение запроса с ключом позиции через умножение соответствующих координат.
// Зачем здесь эта тема: Чтобы запрос оценил релевантность позиции, у той должен быть сравнимый
//   ключ.
// Почему код устроен так: Строим K из состояния токена и сравниваем его с Q: умножаем соответствующие числа
//   и складываем результаты.
// Представь: Ключ позиции позволяет запросу решить, насколько эта позиция подходит.
//
// Что изучаем: Вектор ключа K.
// Зачем это нужно: Key описывает, на какой запрос позиция отвечает; сравнение Q·K даёт оценку внимания.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

fn main() {
    let query: [f64; 2] = [1.0, 0.5];
    let key: [f64; 2] = [0.8, 0.2];

    plot_results_after_multiplying_matching_query_and_key_coords(
        query,
        key,
        multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&query, &key)
            .expect("запрос и ключ должны иметь одинаковое число координат"),
    );
}

// Строим график по результатам урока.
fn plot_results_after_multiplying_matching_query_and_key_coords(
    query: [f64; 2],
    key: [f64; 2],
    score: f64,
) {
    lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Вклады координат в Q·K",
        "вклад",
        &[
            ("координата 0", query[0] * key[0]),
            ("координата 1", query[1] * key[1]),
            ("сумма", score),
        ],
    )
    .expect("не удалось сохранить график");
}
