// Урок 06.5. Доверительный интервал среднего: оценка границ неопределённости по выборке.
// Связь с принятой терминологией: Доверительный интервал для среднего генеральной совокупности.
// Зачем здесь эта тема: Выборочное среднее меняется от выборки к выборке; интервал отражает эту
//   неопределённость.
// Почему код устроен так: Соединяем среднее с оценкой стандартной ошибки и множителем 1,96; это
//   учебное нормальное приближение.
// Представь: Среднее из четырёх наблюдений — оценка; другой набор из той же совокупности мог бы
//   дать немного другое число.
//
// Что изучаем: Доверительный интервал среднего.
// Зачем это нужно: Интервал показывает неопределённость оценки среднего. Демонстрируем приближение mean ±
// 1.96·SE для небольшого учебного набора.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Создаём набор значений `values` для следующего шага примера.");
    let values: [f64; 4] = [2.0, 4.0, 6.0, 8.0];
    trace_step!(values);
    trace_note!("Сохраняем результат этого шага в `mean`.");
    let mean: f64 =
        l030_06_calculate_mean_by_summing_values_and_dividing_by_count::calculate_mean_by_summing_values_and_dividing_by_count(&values)
            .unwrap();
    trace_step!(mean);
    trace_note!("Сохраняем результат этого шага в `sample_variance`.");
    let sample_variance: f64 =
        l032_06_calculate_sample_variance_as_squared_deviation_sum_over_count_minus_one::calculate_sample_variance_as_squared_deviation_sum_divided_by_count_minus_one(
            &values,
        )
        .unwrap();
    trace_step!(sample_variance);
    trace_note!("Считаем количество элементов и сохраняем его в `standard_error_squared`.");
    let standard_error_squared: f64 = sample_variance / values.len() as f64;
    trace_step!(standard_error_squared);
    trace_note!("Создаём изменяемое значение `standard_error` для следующих операций.");
    let mut standard_error: f64 = standard_error_squared;
    trace_step!(standard_error);
    trace_note!("80 шагов Ньютона дают здесь устойчивую оценку корня из SE² в арифметике f64.");
    for _ in 0..80 {
        trace_note!("Среднее текущей оценки и SE²/оценка приближается к стандартной ошибке SE.");
        standard_error = (standard_error + standard_error_squared / standard_error) / 2.0;
        trace_step!(standard_error);
    }
    trace_note!(
        "1.96 — квантиль стандартного нормального распределения для двустороннего 95% интервала."
    );
    trace_note!("Формула mean ± 1.96·SE здесь является приближением для учебного примера.");
    let margin: f64 = 1.96 * standard_error;
    trace_step!(margin);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    trace_note!("Подставляем результаты в этот шаблон вывода или текстового значения.");
    trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
    trace_note!("Складываем или вычитаем величины согласно используемой формуле.");
    println!(
        "приближённый интервал: [{:.2}, {:.2}]",
        mean - margin,
        mean + margin
    );

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_observations_mean_and_uncertainty_bounds(values, mean, margin);
}

// Строим график по результатам урока.
fn plot_observations_mean_and_uncertainty_bounds(values: [f64; 4], mean: f64, margin: f64) {
    trace_note!("Границы интервала показаны рядом с наблюдениями и средним.");
    trace_note!("Просматриваем элементы коллекции по ссылке.");
    trace_note!("Добавляем порядковый номер к каждому элементу.");
    trace_note!("Преобразуем каждый элемент в новое значение.");
    trace_note!("Собираем результаты в коллекцию.");
    let observations: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(item_index, &element_value)| ((item_index + 1) as f64, element_value))
        .collect();
    trace_note!("Определяем размер данных и сохраняем его в `mean_line`.");
    let mean_line: [(f64, f64); 2] = [(1.0, mean), (values.len() as f64, mean)];
    trace_note!("Определяем размер данных и сохраняем его в `lower`.");
    let lower: [(f64, f64); 2] = [(1.0, mean - margin), (values.len() as f64, mean - margin)];
    trace_note!("Определяем размер данных и сохраняем его в `upper`.");
    let upper: [(f64, f64); 2] = [(1.0, mean + margin), (values.len() as f64, mean + margin)];
    trace_note!("Строим график по рассчитанным значениям и сохраняем его как SVG.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок графика.");
    trace_note!("Указываем подпись горизонтальной оси.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Добавляем ряд данных с подписью к графику.");
    trace_note!("Указываем подпись этого ряда в легенде.");
    trace_note!("Передаём рассчитанные координаты точек.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::line_chart(
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
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
