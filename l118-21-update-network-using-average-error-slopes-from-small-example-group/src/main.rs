// Урок 21.5. Обновление сети по средним скоростям изменения ошибки для небольшой группы примеров.
// Связь с принятой терминологией: Обновление параметров сети по среднему градиенту мини пакета.
// Зачем здесь эта тема: Градиент по одному примеру шумен, а по всему набору дорог; мини пакет даёт
//   промежуточный вариант.
// Почему код устроен так: Усредняем градиенты нескольких строк и делаем одно обновление параметров.
// Представь: Для градиентов 2 и 4 средний градиент мини пакета равен 3; шаг делаем по нему один
//   раз.
//
// Что изучаем: Mini-batch обучение.
// Зачем это нужно: Шаг обновления может использовать средний градиент небольшой группы примеров вместо
// одного объекта или всего набора.

// Точка входа: все определения и шаги примера выполняются внутри этой функции.
use lesson_trace::{disable, enable, trace_note, trace_step};

fn main() {
    enable();
    trace_note!("Создаём набор значений `rates_of_change` для следующего шага примера.");
    trace_note!("Производную функции по параметру или вектор таких производных называют gradient.");
    let rates_of_change: [f64; 2] = [2.0, 4.0];
    trace_step!(rates_of_change);
    trace_note!("Проверяем ожидаемое свойство учебного примера.");
    trace_note!("Используем подготовленное значение в следующем шаге примера.");
    trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !rates_of_change.is_empty(),
        "мини-пакет градиентов не должен быть пустым"
    );
    trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `small_batch_loss_rate_of_change`."
    );
    let small_batch_loss_rate_of_change: f64 =
        rates_of_change.iter().sum::<f64>() / rates_of_change.len() as f64;
    trace_step!(small_batch_loss_rate_of_change);
    trace_note!("Сохраняем рассчитанное значение `old_weight` для следующих операций.");
    let old_weight: f64 = 1.0;
    trace_step!(old_weight);
    trace_note!(
        "Скорость 0.1 означает, что из веса вычитается десятая часть среднего градиента мини-батча."
    );
    let learning_rate: f64 = 0.1;
    trace_step!(learning_rate);
    trace_note!("Умножаем значения и сохраняем результат в `new_weight`.");
    let new_weight: f64 = old_weight - learning_rate * small_batch_loss_rate_of_change;
    trace_step!(new_weight);
    trace_note!("Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением.");
    println!("средний градиент={small_batch_loss_rate_of_change}, новый вес={new_weight}");

    trace_note!("Построение графика вынесено из основного кода урока.");
    disable();
    plot_weight_before_and_after_averaged_example_update(old_weight, new_weight);
}

// Строим график по результатам урока.
fn plot_weight_before_and_after_averaged_example_update(old_weight: f64, new_weight: f64) {
    trace_note!("Сравниваем величины, вычисленные в примере.");
    trace_note!("Передаём путь к каталогу текущего урока.");
    trace_note!("Указываем имя SVG-файла.");
    trace_note!("Указываем заголовок диаграммы.");
    trace_note!("Указываем подпись вертикальной оси.");
    trace_note!("Передаём ряды или значения для отрисовки графика.");
    trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Mini-batch обновление",
        "вес",
        &[("до", old_weight), ("после", new_weight)],
    )
    .expect("не удалось сохранить график");
    trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
