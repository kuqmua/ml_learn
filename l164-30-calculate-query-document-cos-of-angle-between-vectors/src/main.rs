// Урок 30.3. cos угла между векторами запроса и документа: умножение координат, сложение и деление на длины.
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
use l006_01_calculate_cos_of_angle_between_vectors::calculate_cos_of_angle_between_vectors;

fn main() {
    let query: [f64; 2] = [1.0, 0.0];
    let document: [f64; 2] = [2.0, 0.0];
    let _cos: f64 = calculate_cos_of_angle_between_vectors(&query, &document)
        .expect("для вычисления cos нужны ненулевые векторы слов с одинаковым числом координат");

    plot_query_document_cos_of_angle(query);
}

// Строим график по результатам урока.
fn plot_query_document_cos_of_angle(query: [f64; 2]) {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
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
                    let rotated_document: [f64; 2] = [angle.cos(), angle.sin()];
                    (
                        degrees as f64,
                        calculate_cos_of_angle_between_vectors(&query, &rotated_document).unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
