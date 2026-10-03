// Урок 12.1. Квадрат расстояния между примерами: сложение квадратов разностей признаков.
// Связь с принятой терминологией: Квадрат расстояния по признакам для поиска ближайших соседей.
// Зачем здесь эта тема: kNN ищет похожие обучающие точки; сначала нужно определить сравнимое
//   расстояние.
// Почему код устроен так: Суммируем квадраты разностей признаков, пока корень не нужен для выбора
//   ближайшего.
// Представь: Чтобы выбрать ближайшую точку, сравнивать 9 и 16 достаточно; корни 3 и 4 не меняют
//   порядок.
//
// Что изучаем: Расстояния для ближайших соседей.
// Зачем это нужно: Сравниваем объекты по сумме квадратов разностей признаков; корень не нужен, если
// требуется только порядок соседей.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l004_01_calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther::calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther;

fn main() {
    let query: [f64; 2] = [1.0, 2.0];
    let candidates: [[f64; 2]; 2] = [[2.0, 2.0], [4.0, 6.0]];
    for candidate in candidates {
        let _: f64 = calc_squared_point_dist_by_summing_squared_coord_diffs_where_0_means_same_point_and_larger_means_farther(
            &query, &candidate,
        )
        .expect("координаты должны быть конечными, а квадрат расстояния — помещаться в f64");
    }

    plot_dist_from_query_for_changing_coord();
}

// Строим график по результатам урока.
fn plot_dist_from_query_for_changing_coord() {
    lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Расстояние до запроса",
        "первая координата",
        "евклидово расстояние",
        &[lesson_visualization::Series {
            name: "запрос [0,0]",

            points: &(-50..=50)
                .map(|plot_step_index| {
                    let horizontal_value: f64 = plot_step_index as f64 / 10.0;
                    (
                        horizontal_value,
                        (horizontal_value * horizontal_value + 1.0).sqrt(),
                    )
                })
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
