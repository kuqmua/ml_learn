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
fn main() {
    lesson_trace::enable();
    lesson_trace::trace_note!(
        "Создаём набор значений `rates_of_change` для следующего шага примера."
    );
    lesson_trace::trace_note!(
        "Производную функции по параметру или вектор таких производных называют gradient."
    );
    let rates_of_change: [f64; 2] = [2.0, 4.0];
    lesson_trace::trace_step!(rates_of_change);
    lesson_trace::trace_note!("Проверяем ожидаемое свойство учебного примера.");
    lesson_trace::trace_note!("Используем подготовленное значение в следующем шаге примера.");
    lesson_trace::trace_note!("Передаём подпись или текстовое значение для следующего шага.");
    assert!(
        !rates_of_change.is_empty(),
        "мини-пакет градиентов не должен быть пустым"
    );
    lesson_trace::trace_note!(
        "Преобразуем входные данные и сохраняем полученную коллекцию в `small_batch_loss_rate_of_change`."
    );
    let small_batch_loss_rate_of_change: f64 =
        rates_of_change.iter().sum::<f64>() / rates_of_change.len() as f64;
    lesson_trace::trace_step!(small_batch_loss_rate_of_change);
    lesson_trace::trace_note!(
        "Сохраняем рассчитанное значение `old_weight` для следующих операций."
    );
    let old_weight: f64 = 1.0;
    lesson_trace::trace_step!(old_weight);
    lesson_trace::trace_note!(
        "Скорость 0.1 означает, что из веса вычитается десятая часть среднего градиента мини-батча."
    );
    let learning_rate: f64 = 0.1;
    lesson_trace::trace_step!(learning_rate);
    lesson_trace::trace_note!("Умножаем значения и сохраняем результат в `new_weight`.");
    let new_weight: f64 = old_weight - learning_rate * small_batch_loss_rate_of_change;
    lesson_trace::trace_step!(new_weight);
    lesson_trace::trace_note!(
        "Выводим рассчитанные значения, чтобы сравнить их с ожидаемым поведением."
    );
    println!("средний градиент={small_batch_loss_rate_of_change}, новый вес={new_weight}");

    lesson_trace::trace_note!("Построение графика вынесено из основного кода урока.");
    lesson_trace::disable();
    plot_weight_before_and_after_averaged_example_update(old_weight, new_weight);
}

// Строим график по результатам урока.
fn plot_weight_before_and_after_averaged_example_update(old_weight: f64, new_weight: f64) {
    lesson_trace::trace_note!("Сравниваем величины, вычисленные в примере.");
    lesson_trace::trace_note!("Передаём путь к каталогу текущего урока.");
    lesson_trace::trace_note!("Указываем имя SVG-файла.");
    lesson_trace::trace_note!("Указываем заголовок диаграммы.");
    lesson_trace::trace_note!("Указываем подпись вертикальной оси.");
    lesson_trace::trace_note!("Передаём ряды или значения для отрисовки графика.");
    lesson_trace::trace_note!("Прерываем пример с понятной ошибкой, если SVG не удалось записать.");
    let chart: std::path::PathBuf = lesson_visualization::bar_chart(
        env!("CARGO_MANIFEST_DIR"),
        "lesson-chart",
        "Mini-batch обновление",
        "вес",
        &[("до", old_weight), ("после", new_weight)],
    )
    .expect("не удалось сохранить график");
    lesson_trace::trace_note!("Печатаем путь к созданному SVG, чтобы его можно было открыть.");
    println!("график: {}", chart.display());
}
