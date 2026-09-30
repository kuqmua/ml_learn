// Урок 04.4. Сравнение обновлений параметра по всем примерам и по одному примеру.
// Связь с принятой терминологией: Пакетное и стохастическое обновление по градиенту.
// Зачем здесь эта тема: Градиент можно оценить по всем примерам или по одному; это меняет шум и
//   стоимость шага.
// Почему код устроен так: Сравниваем оба режима на одних данных, фиксируя, какой набор дал
//   очередное обновление.
// Представь: Среднее по всем строкам даёт ровный шаг, а отдельная случайная строка может временно
//   толкнуть параметр в другую сторону.
//
// Что изучаем: Batch и stochastic обновления.
// Зачем это нужно: Batch использует средний градиент всех примеров, stochastic — градиент одного. На одном
// шаге их направления могут отличаться.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Создаём набор значений `data` для следующего шага примера.");
    let data: [(f64, f64); 2] = [(1.0, 2.0), (2.0, 4.0)];
    trace_step!(data);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    assert!(!data.is_empty(), "для градиента нужны обучающие примеры");
    trace_note!("Инициализируем значение `weight` начальным состоянием.");
    let weight: f64 = 0.0;
    trace_step!(weight);
    trace_note!(
        "Инициализируем изменяемый накопитель `summed_rates_of_change` начальным состоянием."
    );
    trace_note!("Производную функции по параметру или вектор таких производных называют gradient.");
    let mut summed_rates_of_change: f64 = 0.0;
    trace_step!(summed_rates_of_change);
    trace_note!("Повторяем следующий блок для каждого элемента указанной последовательности.");
    for (feature, target) in data {
        trace_step!(feature);
        trace_step!(target);
        trace_note!("Прибавляем очередной вклад к ранее накопленному результату.");
        summed_rates_of_change += 2.0 * (weight * feature - target) * feature;
        trace_step!(summed_rates_of_change);
    }
    trace_note!("Считаем количество элементов и сохраняем его в `batch_loss_rate_of_change`.");
    let batch_loss_rate_of_change: f64 = summed_rates_of_change / data.len() as f64;
    trace_step!(batch_loss_rate_of_change);
    trace_note!(
        "Сохраняем рассчитанное значение `(first_feature, first_target)` для следующих операций."
    );
    let (first_feature, first_target): (f64, f64) = data[0];
    trace_step!(first_feature);
    trace_step!(first_target);
    trace_note!("Умножаем значения и сохраняем результат в `single_example_loss_rate_of_change`.");
    let single_example_loss_rate_of_change: f64 =
        2.0 * (weight * first_feature - first_target) * first_feature;
    trace_step!(single_example_loss_rate_of_change);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("batch={batch_loss_rate_of_change}, stochastic={single_example_loss_rate_of_change}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_average_rate_of_change_and_single_example_rates(
        batch_loss_rate_of_change,
        single_example_loss_rate_of_change,
    );
}

// Строим график по результатам урока.
fn plot_average_rate_of_change_and_single_example_rates(
    batch_loss_rate_of_change: f64,
    single_example_loss_rate_of_change: f64,
) {
    trace_note!("Сравнение величин из этого урока.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Добавляем пару значений для сравнения или построения графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Batch и stochastic градиенты",
        "градиент",
        &[
            ("batch", batch_loss_rate_of_change),
            ("stochastic", single_example_loss_rate_of_change),
        ],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
