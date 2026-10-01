// Урок 31.2. Сравнение запроса с ключом позиции через умножение соответствующих координат.
// Связь с принятой терминологией: Вектор ключа K для позиции механизма внимания.
// Зачем здесь эта тема: Чтобы запрос оценил релевантность позиции, у той должен быть сравнимый
//   ключ.
// Почему код устроен так: Строим K из состояния токена и сравниваем его с Q скалярным
//   произведением.
// Представь: Ключ позиции позволяет запросу решить, насколько эта позиция подходит.
//
// Что изучаем: Вектор ключа K.
// Зачем это нужно: Key описывает, на какой запрос позиция отвечает; сравнение Q·K даёт оценку внимания.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;

fn main() {
    let query: [f64; 2] = [1.0, 0.5];
    let key: [f64; 2] = [0.8, 0.2];
    let score: f64 = multiply_matching_coordinates_then_add_results(&query, &key)
        .expect("запрос и ключ должны иметь одинаковое число координат");

    plot_results_after_multiplying_matching_query_and_key_coordinates(query, key, score);
}

// Строим график по результатам урока.
fn plot_results_after_multiplying_matching_query_and_key_coordinates(
    query: [f64; 2],
    key: [f64; 2],
    score: f64,
) {
    let _chart: std::path::PathBuf = lesson_visualization::bar_chart(
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
