// Урок 28.3. Векторное представление слова: список числовых координат.
// Связь с принятой терминологией: Плотные векторные представления слов.
// Зачем здесь эта тема: Числовой id лишь обозначает токен и не выражает его сходство с другими.
// Почему код устроен так: Сопоставляем каждому id плотный вектор, чтобы последующие слои могли
//   обучать признаки.
// Представь: Id 2 не ближе по смыслу к id 3, чем к id 100; близость можно учить в векторах.
//
// Что изучаем: Плотные представления слов.
// Зачем это нужно: Вместо отдельного разреженного признака на слово используем короткий вектор числовых
// признаков.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let dense_representation_table: [[f64; 2]; 3] = [[0.0, 0.0], [0.8, 0.2], [0.7, 0.3]];
    let text_unit_identifier: usize = 2;
    let dense_numeric_representation: [f64; 2] = dense_representation_table[text_unit_identifier];

    plot_numeric_coordinates_representing_one_text_unit(dense_numeric_representation);
}

// Строим график по результатам урока.
fn plot_numeric_coordinates_representing_one_text_unit(dense_numeric_representation: [f64; 2]) {
    let dense_representations_points: Vec<(f64, f64)> = dense_numeric_representation
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| (item_index as f64, element_value))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Плотное представление токена",
        "измерение",
        "значение",
        &[lesson_visualization::Series {
            name: "эмбеддинг",

            points: &dense_representations_points,
        }],
    )
    .expect("не удалось сохранить график");
}
