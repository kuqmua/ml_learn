// Урок 30.3. Сходство запроса и документа: умножение координат, сложение и деление на длины векторов.
// Связь с принятой терминологией: Косинусное сходство векторов запроса и документа.
// Зачем здесь эта тема: Весов слов недостаточно для ранжирования, если длины документов
//   различаются.
// Почему код устроен так: Сравниваем векторы запроса и документа через косинус, убирая общий
//   масштаб.
// Представь: Два документа одинаковой темы, но разной длины, сравниваются по направлению TF-IDF
//   векторов.
//
// Что изучаем: Косинусное сходство документов.
// Зачем это нужно: Нормировка суммы после попарного умножения координат позволяет сравнивать направления векторов слов, а не
// длину документов.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l006_01_calculate_direction_similarity_by_multiplying_coordinates_then_dividing_sum_by_lengths::calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths;

fn main() {
    let query: [f64; 2] = [1.0, 0.0];
    let document: [f64; 2] = [2.0, 0.0];
    let _similarity: f64 =
        calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths(
            &query, &document,
        )
        .expect("ненулевые векторы слов одинаковой размерности");

    plot_document_direction_similarity_after_multiplying_coordinates_and_dividing_by_lengths(query);
}

// Строим график по результатам урока.
fn plot_document_direction_similarity_after_multiplying_coordinates_and_dividing_by_lengths(
    query: [f64; 2],
) {
    let points: Vec<(f64, f64)> = (0..=180)
        .step_by(5)
        .map(|degrees| {
            let angle: f64 = (degrees as f64).to_radians();
            let rotated_document: [f64; 2] = [angle.cos(), angle.sin()];
            (
                degrees as f64,
                calculate_direction_similarity_by_multiplying_matching_coordinates_then_dividing_sum_by_vector_lengths(
                    &query,
                    &rotated_document,
                )
                .unwrap(),
            )
        })
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сходство документов при изменении направления",
        "угол, градусы",
        "косинус",
        &[lesson_visualization::Series {
            name: "сходство",

            points: &points,
        }],
    )
    .expect("не удалось сохранить график");
}
