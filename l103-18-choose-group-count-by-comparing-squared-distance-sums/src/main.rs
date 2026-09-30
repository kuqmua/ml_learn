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
    lesson_trace::enable();
    lesson_trace::trace_note!("Создаём набор значений `candidates` для следующего шага примера.");
    let candidates: [(i32, f64); 4] = [(1, 52.0), (2, 4.0), (3, 3.5), (4, 3.0)];
    lesson_trace::trace_step!(candidates);
    lesson_trace::trace_note!(
        "Повторяем следующий блок для каждого элемента указанной последовательности."
    );
    lesson_trace::trace_note!("Сумму квадратов расстояний до центров кластеров называют inertia.");
    for (cluster_count, total_squared_distance_to_cluster_centers) in candidates {
        lesson_trace::trace_step!(cluster_count);
        lesson_trace::trace_step!(total_squared_distance_to_cluster_centers);
        lesson_trace::trace_note!(
            "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
        );
        println!("k={cluster_count}, инерция={total_squared_distance_to_cluster_centers}");
    }
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("заметный излом кривой находится около k=2");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_within_cluster_squared_distance_sums_for_different_cluster_counts(candidates);
}

// Строим график по результатам урока.
fn plot_within_cluster_squared_distance_sums_for_different_cluster_counts(
    candidates: [(i32, f64); 4],
) {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Просматриваем элементы коллекции по ссылке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let cluster_count_candidate_points: Vec<(f64, f64)> = candidates
        .iter()
        .map(
            |&(cluster_count, total_squared_distance_to_cluster_centers)| {
                (
                    cluster_count as f64,
                    total_squared_distance_to_cluster_centers,
                )
            },
        )
        .collect();
    lesson_trace::trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок графика.");
    lesson_trace::trace_note!("Указываем подпись горизонтальной оси.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Указываем подпись этого ряда в легенде.");
    lesson_trace::trace_note!("Передаём рассчитанные координаты точек.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Метод локтя",
        "число кластеров k",
        "инерция",
        &[lesson_visualization::Series {
            name: "варианты из урока",

            points: &cluster_count_candidate_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
