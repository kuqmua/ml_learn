// Урок 28.4. Сходство векторов слов: сумма произведений соответствующих координат.
// Связь с принятой терминологией: Скалярное произведение двух эмбеддингов слов как мера сходства.
// Зачем здесь эта тема: После появления эмбеддингов можно сравнивать токены по направлению и
//   величине векторов.
// Почему код устроен так: Считаем скалярное произведение уже знакомым способом на маленькой
//   таблице.
// Представь: Похожие по направлению векторы могут иметь большое скалярное произведение, но его
//   величина зависит и от длин.
//
// Что изучаем: Сходство эмбеддингов.
// Зачем это нужно: Результат умножения пар координат и сложения двух представлений измеряет близость их
// направлений.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;

fn main() {
    let first: [f64; 2] = [0.8, 0.2];
    let second: [f64; 2] = [0.7, 0.3];
    let _sum_after_multiplying_coordinates: f64 =
        calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&first, &second)
            .expect("представления имеют одинаковую размерность");

    plot_coordinates_of_two_word_representations(first, second);
}

// Строим график по результатам урока.
fn plot_coordinates_of_two_word_representations(first: [f64; 2], second: [f64; 2]) {
    let first_dense_representation_points: Vec<(f64, f64)> = vec![(first[0], first[1])];
    let second_dense_representation_points: Vec<(f64, f64)> = vec![(second[0], second[1])];
    let _chart: std::path::PathBuf = lesson_visualization::scatter_chart(
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
