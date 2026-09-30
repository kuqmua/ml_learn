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
use l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding;

fn main() {
    let centered_points: [[f64; 2]; 4] = [[-2.0, 0.0], [-1.0, 0.0], [1.0, 0.0], [2.0, 0.0]];
    let principal_axis: [f64; 2] = [1.0, 0.0];
    for point in centered_points {
        let _projection: f64 =
            calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(
                &point,
                &principal_axis,
            )
            .unwrap();
    }

    plot_centered_points_to_show_direction_of_greatest_spread(centered_points);
}

// Строим график по результатам урока.
fn plot_centered_points_to_show_direction_of_greatest_spread(centered_points: [[f64; 2]; 4]) {
    let principal_direction_points: Vec<(f64, f64)> = centered_points
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    let _chart: std::path::PathBuf = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Проекция на главное направление",
        "первая координата",
        "вторая координата",
        &[lesson_visualization::Series {
            name: "центрированные точки",

            points: &principal_direction_points,
        }],
    )
    .expect("не удалось сохранить график");
}
