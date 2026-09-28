// Урок 17.2. Инициализация k-means.
//
// Что изучаем: Инициализация k-means.
// Зачем это нужно: Начальные центры задают старт итераций и могут менять итоговое разбиение, поэтому их
// нужно фиксировать.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Создаём набор значений `points` для следующего шага примера.
    let points = [[0.0, 0.0], [0.1, 0.0], [5.0, 5.0], [5.1, 5.0]];
    // Создаём набор значений `first_start` для следующего шага примера.
    let first_start = [points[0], points[2]];
    // Создаём набор значений `second_start` для следующего шага примера.
    let second_start = [points[0], points[1]];
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!("разнесённые центры={first_start:?}; соседние центры={second_start:?}");
    // Значения из этого урока на графике.
    let chart_points_0: Vec<(f64, f64)> = points.iter().map(|p| (p[0], p[1])).collect();
    let chart_points_1: Vec<(f64, f64)> = first_start.iter().map(|p| (p[0], p[1])).collect();
    let chart_points_2: Vec<(f64, f64)> = second_start.iter().map(|p| (p[0], p[1])).collect();
    let chart = lesson_visualization::scatter_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Инициализация k-means",
        "x",
        "y",
        &[
            lesson_visualization::Series {
                name: "объекты",
                points: &chart_points_0,
            },
            lesson_visualization::Series {
                name: "разнесённые центры",
                points: &chart_points_1,
            },
            lesson_visualization::Series {
                name: "соседние центры",
                points: &chart_points_2,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
