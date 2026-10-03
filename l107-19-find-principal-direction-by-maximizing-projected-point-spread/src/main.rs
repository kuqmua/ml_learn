// Урок 19.3. Главное направление данных: поиск наибольшего разброса проекций точек.
// Связь с принятой терминологией: Главное направление PCA с наибольшей дисперсией.
// Зачем здесь эта тема: PCA выбирает ось, на которую проекция данных имеет наибольшую дисперсию.
// Почему код устроен так: Для двух признаков связываем ковариационную матрицу с направлением её
//   главного собственного вектора.
// Представь: Для точек вдоль диагонали главная ось идёт примерно по этой диагонали, а не поперёк
//   неё.
//
// Что изучаем: Главное направление PCA.
// Зачем это нужно: Собственное направление с наибольшей дисперсией служит первой осью PCA. На точках вдоль
// x эта ось совпадает с x.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use l001_01_multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec::multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec;

fn main() {
    let centered_points: [[f64; 2]; 4] = [[-2.0, 0.0], [-1.0, 0.0], [1.0, 0.0], [2.0, 0.0]];
    let unit_direction_preserving_largest_spread: [f64; 2] = [1.0, 0.0];
    for point in centered_points {
        let _projection_coord_where_0_means_no_component_along_axis_and_sign_shows_axis_direction: f64 =
            multiply_matching_coords_then_add_results_where_pos_means_angle_below_90_neg_means_angle_above_90_and_0_means_perpendicular_or_zero_vec(&point, &unit_direction_preserving_largest_spread).unwrap();
    }

    plot_centered_points_to_show_direction_of_greatest_spread(centered_points);
}

// Строим график по результатам урока.
fn plot_centered_points_to_show_direction_of_greatest_spread(centered_points: [[f64; 2]; 4]) {
    lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Проекция на главное направление",
        "первая координата",
        "вторая координата",
        &[lesson_visualization::Series {
            name: "центрированные точки",

            points: &centered_points
                .iter()
                .map(|data_point| (data_point[0], data_point[1]))
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
