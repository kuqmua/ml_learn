// Урок 19.4. Сохранённая доля разброса: деление разброса вдоль выбранного направления на общий.
// Связь с принятой терминологией: Доля дисперсии, объяснённая главной компонентой.
// Зачем здесь эта тема: Уменьшение размерности теряет часть разброса; нужно измерять, сколько
//   сохранилось.
// Почему код устроен так: Делим дисперсию выбранной оси на сумму дисперсий всех осей.
// Представь: Если первая ось объясняет 90% разброса, проекция на неё сохраняет большую часть
//   различий между точками.
//
// Доля первой оси равна её дисперсии, делённой на общую дисперсию.
// Она лежит от 0 до 1; при нулевой общей дисперсии долю определить нельзя.

fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!("Задаём учебные значения для `cases`.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    lesson_trace::trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2], Option<f64>); 4] = [
        ("первая ось сохраняет почти всё", [9.0, 1.0], Some(0.9)),
        ("оси равноправны", [5.0, 5.0], Some(0.5)),
        ("первая ось ничего не сохраняет", [0.0, 4.0], Some(0.0)),
        ("изменчивости нет", [0.0, 0.0], None),
    ];
    lesson_trace::trace_step!(cases);
    lesson_trace::trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, eigenvalues, expected) in cases {
        lesson_trace::trace_step!(description);
        lesson_trace::trace_step!(eigenvalues);
        lesson_trace::trace_step!(expected);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        lesson_trace::trace_note!("Обновляем значение результатом текущего вычисления.");
        lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            eigenvalues.iter().all(|&value| value >= 0.0),
            "дисперсия не может быть отрицательной"
        );
        lesson_trace::trace_note!("Сохраняем результат этого шага в `total_variance`.");
        let total_variance: f64 = eigenvalues[0] + eigenvalues[1];
        lesson_trace::trace_step!(total_variance);
        lesson_trace::trace_note!(
            "Сохраняем результат этого шага в `variance_share_explained_by_first_axis`."
        );
        lesson_trace::trace_note!(
            "Долю общей дисперсии, объяснённую осью, называют explained variance fraction."
        );
        let variance_share_explained_by_first_axis: Option<f64> = if total_variance == 0.0 {
            lesson_trace::trace_note!("Отмечаем отсутствие подходящего значения.");
            None
        } else {
            lesson_trace::trace_note!(
                "Обрабатываем случай, когда предыдущее условие не выполнено."
            );
            lesson_trace::trace_note!("Возвращаем присутствующее значение.");
            Some(eigenvalues[0] / total_variance)
        };
        lesson_trace::trace_step!(variance_share_explained_by_first_axis);
        lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(variance_share_explained_by_first_axis, expected);
        lesson_trace::trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!(
            "{description}: {eigenvalues:?} → доля первой оси {variance_share_explained_by_first_axis:?}"
        );
    }

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_first_direction_share_for_changing_variance();
}

// Строим график по результатам урока.
fn plot_first_direction_share_for_changing_variance() {
    lesson_trace::trace_note!("График величин и зависимостей, изученных в этом уроке.");
    lesson_trace::trace_note!("Преобразуем каждый элемент в новое значение.");
    lesson_trace::trace_note!("Собираем результаты в коллекцию.");
    let explained_variance_points: Vec<(f64, f64)> = (0..=50)
        .map(|plot_step_index| {
            lesson_trace::trace_note!("Сохраняем результат этого шага в `loss_value`.");
            let loss_value: f64 = plot_step_index as f64 / 10.0;
            lesson_trace::trace_note!(
                "Добавляем пару значений для сравнения или построения графика."
            );
            (loss_value, loss_value / (loss_value + 1.0))
        })
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
        "Объяснённая дисперсия первой оси",
        "λ₁",
        "доля",
        &[lesson_visualization::Series {
            name: "λ₂=1",

            points: &explained_variance_points,
        }],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
