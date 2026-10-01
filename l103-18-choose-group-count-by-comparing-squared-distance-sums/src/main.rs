// Урок 18.4. Выбор числа групп: сравнение сумм квадратов расстояний до центров.
// Связь с принятой терминологией: Выбор числа кластеров k-means по инерции.
// Зачем здесь эта тема: Меньшая инерция при большем числе кластеров сама по себе не доказывает
//   хороший выбор k.
// Почему код устроен так: Сравниваем заданные значения инерции для нескольких k и ищем заметный
//   излом кривой.
// Представь: Инерция падает при добавлении центров; ищем место, после которого улучшение стало
//   небольшим.
//
// Что изучаем: Выбор числа кластеров.
// Зачем это нужно: Увеличение k обычно уменьшает инерцию; выбираем число центров по качеству и простоте
// описания.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.

fn main() {
    let candidates: [(i32, f64); 4] = [(1, 52.0), (2, 4.0), (3, 3.5), (4, 3.0)];
    for (_cluster_count, _total_squared_distance_to_cluster_centers) in candidates {}

    plot_within_cluster_squared_distance_sums_for_different_cluster_counts(candidates);
}

// Строим график по результатам урока.
fn plot_within_cluster_squared_distance_sums_for_different_cluster_counts(
    candidates: [(i32, f64); 4],
) {
    let _chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Метод локтя",
        "число кластеров k",
        "инерция",
        &[lesson_visualization::Series {
            name: "варианты из урока",

            points: &candidates
                .iter()
                .map(
                    |&(cluster_count, total_squared_distance_to_cluster_centers)| {
                        (
                            cluster_count as f64,
                            total_squared_distance_to_cluster_centers,
                        )
                    },
                )
                .collect::<Vec<_>>(),
        }],
    )
    .expect("не удалось сохранить график");
}
