// Урок 06.5. Доверительный интервал среднего.
//
// Что изучаем: Доверительный интервал среднего.
// Зачем это нужно: Интервал показывает неопределённость оценки среднего. Демонстрируем приближение mean ±
// 1.96·SE для небольшого учебного набора.
// Что делает пример: на небольших проверяемых данных вычисляет результат этой темы и печатает его.
// Как проверить понимание: предскажи вывод до запуска, затем измени одно входное значение и объясни
// изменение результата.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
fn main() {
    // Создаём набор значений `values` для следующего шага примера.
    let values = [2.0, 4.0, 6.0, 8.0];
    let mean = lesson_029::mean(&values).unwrap();
    let sample_variance = lesson_031::sample_variance(&values).unwrap();
    // Считаем количество элементов и сохраняем его в `standard_error_squared`.
    let standard_error_squared = sample_variance / values.len() as f64;
    // Создаём изменяемое значение `standard_error` для следующих операций.
    let mut standard_error = standard_error_squared;
    // Повторяем следующий блок для каждого элемента указанной последовательности.
    for _ in 0..80 {
        // Присваиваем вычисленное значение соответствующей переменной или полю.
        standard_error = (standard_error + standard_error_squared / standard_error) / 2.0;
    }
    // Умножаем значения и сохраняем результат в `margin`.
    let margin = 1.96 * standard_error;
    // Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.
    println!(
        // Подставляем результаты в этот шаблон вывода или текстового значения.
        "приближённый интервал: [{:.2}, {:.2}]",
        // Складываем или вычитаем величины согласно используемой формуле.
        mean - margin,
        // Складываем или вычитаем величины согласно используемой формуле.
        mean + margin
    );
    // Границы интервала показаны рядом с наблюдениями и средним.
    let observations: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(i, &v)| ((i + 1) as f64, v))
        .collect();
    let mean_line = [(1.0, mean), (values.len() as f64, mean)];
    let lower = [(1.0, mean - margin), (values.len() as f64, mean - margin)];
    let upper = [(1.0, mean + margin), (values.len() as f64, mean + margin)];
    let chart = lesson_visualization::line_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Приближённый доверительный интервал",
        "номер наблюдения",
        "значение",
        &[
            lesson_visualization::Series {
                name: "выборка",
                points: &observations,
            },
            lesson_visualization::Series {
                name: "среднее",
                points: &mean_line,
            },
            lesson_visualization::Series {
                name: "нижняя граница",
                points: &lower,
            },
            lesson_visualization::Series {
                name: "верхняя граница",
                points: &upper,
            },
        ],
    )
    .expect("не удалось сохранить график");
    println!("график: {}", chart.display());
}
