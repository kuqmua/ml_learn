// Урок 30.3. cos угла между векторами запроса и документа: умножение координат, сложение и деление на длины.
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
use l006_01_multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens::multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens;

fn main() {
    let query: [f64; 2] = [1.0, 0.0];
    let document: [f64; 2] = [2.0, 0.0];
    let _: f64 =
        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(&query, &document)
            .expect(
                "для вычисления cos нужны ненулевые векторы слов с одинаковым числом координат",
            );

    plot_query_document_cos_of_angle(query);
}

// Строим график по результатам урока.
fn plot_query_document_cos_of_angle(query: [f64; 2]) {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Сходство документов при изменении направления",
        "угол, градусы",
        "косинус",
        &[lesson_visualization::Series {
            name: "сходство",

            points: &(0..=180)
                .step_by(5)
                .map(|degrees| {
                    let angle: f64 = (degrees as f64).to_radians();
                    let cos_as_horizontal_coord_of_unit_direction: f64 = angle.cos();
                    let sin_as_vertical_coord_of_unit_direction: f64 = angle.sin();
                    let rotated_document: [f64; 2] = [
                        cos_as_horizontal_coord_of_unit_direction,
                        sin_as_vertical_coord_of_unit_direction,
                    ];
                    (
                        degrees as f64,
                        multiply_matching_coords_then_add_results_and_normalize_by_both_vec_lens(
                            &query,
                            &rotated_document,
                        )
                        .unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
