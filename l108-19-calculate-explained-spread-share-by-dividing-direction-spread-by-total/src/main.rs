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

use lesson_trace::{disable, enable_tracing, trace_note, trace_step};

fn main() {
    enable_tracing();
    trace_note!("Задаём учебные значения для `cases`.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    let cases: [(&str, [f64; 2], Option<f64>); 4] = [
        ("первая ось сохраняет почти всё", [9.0, 1.0], Some(0.9)),
        ("оси равноправны", [5.0, 5.0], Some(0.5)),
        ("первая ось ничего не сохраняет", [0.0, 4.0], Some(0.0)),
        ("изменчивости нет", [0.0, 0.0], None),
    ];
    trace_step!(cases);
    trace_note!("Повторяем расчёт для каждого элемента последовательности.");
    for (description, eigenvalues, expected) in cases {
        trace_step!(description);
        trace_step!(eigenvalues);
        trace_step!(expected);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        trace_note!("Обновляем значение результатом текущего вычисления.");
        trace_note!("Передаём подпись или текстовое значение для следующего шага.");
        assert!(
            eigenvalues.iter().all(|&value| value >= 0.0),
            "дисперсия не может быть отрицательной"
        );
        trace_note!("Сохраняем результат этого шага в `total_variance`.");
        let total_variance: f64 = eigenvalues[0] + eigenvalues[1];
        trace_step!(total_variance);
        trace_note!("Сохраняем результат этого шага в `variance_share_explained_by_first_axis`.");
        trace_note!(
            "Долю общей дисперсии, объяснённую осью, называют explained variance fraction."
        );
        let variance_share_explained_by_first_axis: Option<f64> = if total_variance == 0.0 {
            trace_note!("Отмечаем отсутствие подходящего значения.");
            None
        } else {
            trace_note!("Обрабатываем случай, когда предыдущее условие не выполнено.");
            trace_note!("Возвращаем присутствующее значение.");
            Some(eigenvalues[0] / total_variance)
        };
        trace_step!(variance_share_explained_by_first_axis);
        trace_note!("Проверяем ожидаемое свойство учебного примера.");
        assert_eq!(variance_share_explained_by_first_axis, expected);
        trace_note!("Печатаем рассчитанные значения для проверки примера.");
        println!(
            "{description}: {eigenvalues:?} → доля первой оси {variance_share_explained_by_first_axis:?}"
        );
    }

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_first_direction_share_for_changing_variance();
}

// Строим график по результатам урока.
fn plot_first_direction_share_for_changing_variance() {
    trace_note!("График величин и зависимостей, изученных в этом уроке.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let explained_variance_points: Vec<(f64, f64)> = (0..=50)
        .map(|plot_step_index| {
            trace_note!("Сохраняем результат этого шага в `loss_value`.");
            let loss_value: f64 = plot_step_index as f64 / 10.0;
            trace_note!("Добавляем пару значений для сравнения или построения графика.");
            (loss_value, loss_value / (loss_value + 1.0))
        })
        .collect();
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
