// Урок 31.4. Масштабированная оценка совпадения: умножение координат запроса и ключа, сложение и деление на корень из числа координат.
// Связь с принятой терминологией: Масштабирование скалярного произведения векторов запроса и ключа.
// Зачем здесь эта тема: При большой размерности скалярные оценки Q·K растут и softmax может стать
//   слишком резким.
// Почему код устроен так: Делим оценку на корень из размерности ключа до нормировки.
// Представь: При длинных Q и K сумма произведений может стать большой; деление удерживает оценки в
//   более удобном масштабе.
//
// Что изучаем: Масштабирование скалярного произведения векторов запроса и ключа.
// Зачем это нужно: Оценку внимания делим на корень из размерности ключа, чтобы крупные векторы не делали
// softmax слишком резким.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coordinates_then_add_results::multiply_matching_coordinates_then_add_results;

fn main() {
    let dimension: f64 = 2.0;
    let mut scale: f64 = dimension;
    for _ in 0..80 {
        scale = (scale + dimension / scale) / 2.0;
    }
    let query: [f64; 2] = [1.0, 1.0];
    let key: [f64; 2] = [2.0, 2.0];
    let _ = &(multiply_matching_coordinates_then_add_results(&query, &key)
        .expect("запрос и ключ должны иметь одинаковое число координат")
        / scale);

    plot_attention_scale_as_one_divided_by_square_root_of_coordinate_count();
}

// Строим график по результатам урока.
fn plot_attention_scale_as_one_divided_by_square_root_of_coordinate_count() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Масштабирование скалярного произведения векторов запроса и ключа",
        "размерность d",
        "множитель 1/√d",
        &[lesson_visualization::Series {
            name: "масштаб",

            points: &(1..=64)
                .map(|vector_dimension| {
                    (
                        vector_dimension as f64,
                        1.0 / (vector_dimension as f64).sqrt(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
