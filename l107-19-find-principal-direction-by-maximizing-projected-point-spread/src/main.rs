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
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Создаём набор значений `centered_points` для следующего шага примера.");
    let centered_points: [[f64; 2]; 4] = [[-2.0, 0.0], [-1.0, 0.0], [1.0, 0.0], [2.0, 0.0]];
    trace_step!(centered_points);
    trace_note!("Создаём набор значений `principal_axis` для следующего шага примера.");
    let principal_axis: [f64; 2] = [1.0, 0.0];
    trace_step!(principal_axis);
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    for point in centered_points {
        trace_step!(point);
        trace_note!("Умножаем значения и сохраняем результат в `projection`.");
        trace_note!("Используем подготовленное значение в следующем шаге примера.");
        let projection: f64 =

            l001_01_calculate_scalar_product_by_multiplying_matching_coordinates_and_adding::calculate_scalar_product_by_multiplying_matching_coordinates_then_adding(&point, &principal_axis).unwrap();
        trace_step!(projection);
        trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
        println!("точка={point:?}, координата на главной оси={projection}");
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_centered_points_to_show_direction_of_greatest_spread(centered_points);
}

// Строим график по результатам урока.
fn plot_centered_points_to_show_direction_of_greatest_spread(centered_points: [[f64; 2]; 4]) {
    trace_note!("Значения из этого урока на графике.");
    let principal_direction_points: Vec<(f64, f64)> = centered_points
        .iter()
        .map(|data_point| (data_point[0], data_point[1]))
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::scatter_chart(
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
