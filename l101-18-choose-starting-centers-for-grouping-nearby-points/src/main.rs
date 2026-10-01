// Урок 18.2. Выбор начальных центров для объединения близких точек в группы.
// Связь с принятой терминологией: Выбор начальных центроидов для кластеризации k-means.
// Зачем здесь эта тема: k-means требует начальных центров до первого назначения точек; от них может
//   зависеть результат.
// Почему код устроен так: Сравниваем варианты начального выбора на малых данных.
// Представь: Если поставить два начальных центра рядом, алгоритму сложнее сразу разделить две
//   далёкие группы.
//
// Что изучаем: Инициализация k-means.
// Зачем это нужно: Начальные центры задают старт итераций и могут менять итоговое разбиение, поэтому их
// нужно фиксировать.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let points: [[f64; 2]; 4] = [[0.0, 0.0], [0.1, 0.0], [5.0, 5.0], [5.1, 5.0]];
    let first_start: [[f64; 2]; 2] = [points[0], points[2]];
    let second_start: [[f64; 2]; 2] = [points[0], points[1]];

    plot_initial_cluster_centers(points, first_start, second_start);
}

// Строим график по результатам урока.
fn plot_initial_cluster_centers(
    points: [[f64; 2]; 4],
    first_start: [[f64; 2]; 2],
    second_start: [[f64; 2]; 2],
) {
    lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Инициализация k-means",
        "x",
        "y",
        &[
            lesson_visualization::Series {
                name: "объекты",

                points: &points
                    .iter()
                    .map(|data_point| (data_point[0], data_point[1]))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "разнесённые центры",

                points: &first_start
                    .iter()
                    .map(|data_point| (data_point[0], data_point[1]))
                    .collect::<Vec<_>>(),
            },
            lesson_visualization::Series {
                name: "соседние центры",

                points: &second_start
                    .iter()
                    .map(|data_point| (data_point[0], data_point[1]))
                    .collect::<Vec<_>>(),
            },
        ],
    )
    .expect("не удалось сохранить график");
}
